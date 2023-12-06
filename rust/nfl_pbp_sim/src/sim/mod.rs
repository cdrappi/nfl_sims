pub mod box_score;
pub mod play_result;

use std::collections::HashMap;

use crate::{
    game_loop::{
        field_goals::FG_SNAP_DISTANCE,
        play_calling::{choose_playcall, PlayType},
    },
    models::{
        clock::{ClockModel, PLAYCLOCK},
        defensive_timeout::DefensiveTimeoutModel,
        dropback::DropbackModel,
        features::PlaycallFeatures,
        field_goals::FgModel,
        kickoff::KickoffModel,
        offensive_timeout::OffensiveTimeoutModel,
        penalty::PenaltyModel,
        playcall::PlaycallModel,
        post_pass_penalty::PostPassPenaltyModel,
        punt::PuntModel,
        rushing::RushingModel,
        two_point_attempt::TwoPointAttemptModel,
    },
    params::{
        skill_player::Position, team, GameParams, GameParamsDistribution, Injury, TeamParams,
    },
    start::{GameStart, HomeAway},
    state::{
        clock::Quarter,
        down::{DownToGo, PlayState, ToGo},
        game_state::GameState,
        yards_to_goal::YardsToGoal,
    },
    util::stats::random_bool,
};

use crate::sim::box_score::BoxScore;
use crate::sim::play_result::{
    DropbackOutcome, FieldGoalResult, KickingResult, PlayResult, RunResult, RushingOutcome,
    SackOutcome, TurnoverOutcome,
};

use self::play_result::{
    ClockStatus, DropbackResult, PATKickingOutcome, PATResult, PenaltyType, PlaycallResult,
    TargetOutcome,
};

pub enum TimeoutCalled {
    Offense,
    Defense,
    NoTimeout,
}

pub struct GameSim {
    pub game_state: GameState,
    pub box_score: BoxScore,
    pub game_params: GameParams,
}

impl std::fmt::Display for GameSim {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.game_state.play {
            PlayState::Down(down_to_go) => {
                let offense = self.offense_params();
                let defense = self.defense_params();
                let loc = match down_to_go.yards_to_goal.0 {
                    1..=49 => format!("{} {}", defense.team.team, down_to_go.yards_to_goal),
                    50 => "50".into(),
                    51..=99 => format!("{} {}", offense.team.team, down_to_go.yards_to_goal.flip()),
                    _ => panic!("yards_to_goal is outside [1,99]"),
                };
                return write!(
                    f,
                    "[{}] {}, {} @ {}",
                    self.game_state.clock, offense.team.team, down_to_go, loc
                );
            }
            _ => write!(f, "[{}] {}", self.game_state.clock, self.game_state.play),
        }
    }
}

impl GameSim {
    pub fn new(game_params: GameParams, game_state: GameState) -> GameSim {
        let home = game_params.home.team.team.clone();
        let away = game_params.away.team.team.clone();
        let box_score = BoxScore::new(home, away, &game_params);
        GameSim {
            game_params,
            game_state,
            box_score,
        }
    }

    pub fn end_of_game(&self) -> bool {
        self.game_state.clock.quarter == Quarter::EndOfGame
    }

    fn register_down(&mut self) {
        if let PlayState::Down(dtg) = self.game_state.play {
            let yardline = dtg.yards_to_goal.0;
            // Now perform the mutable borrow
            // Extract the value needed from `self.offense_params()` and store it in a variable
            let offense_team = self.offense_params().team.team.clone();

            // Now perform the mutable borrow
            let fp = self
                .box_score
                .field_position
                .get_mut(&offense_team)
                .unwrap();
            match fp.get_mut(&yardline) {
                Some(count) => *count += 1,
                None => {
                    fp.insert(yardline, 1);
                }
            }
        }
    }

    /// given a set of injury statuses, edit live market shares
    pub fn apply_injuries(&mut self) {
        let team_params = match self.game_state.play.possession() {
            HomeAway::Home => &mut self.game_params.home,
            HomeAway::Away => &mut self.game_params.away,
        };
        team_params.apply_injuries();
    }

    pub fn next_play(&mut self) -> (PlayResult, u16, ClockStatus, bool) {
        log::debug!("{}", self);
        self.register_down();
        let play_call = choose_playcall(&self);
        let result: PlayResult = self.play_result(&play_call);
        log::debug!("{}\n", result);
        let injuries = Injury::sim_injuries(&result, self.offense_params());
        self.game_params
            .update_injuries(self.game_state.play.possession(), injuries);
        self.apply_injuries();

        // add stats to the box score
        self.box_score.apply_stats(&result, &self.game_state.play);
        self.game_state.score.update(&self.box_score.score);

        let (state, mut clock_stops) = GameSim::next_play_state(&result, &self.game_state.play);
        self.game_state.update_play(state);

        let mut duration = 0;
        let mut reset_to_kickoff = false;
        if play_call.consumes_clock() {
            duration = ClockModel::sim_play_duration(&self, &result);
            clock_stops = match self
                .game_state
                .advance_clock(duration, result.is_ot_ender(), false)
            {
                (true, reset) => {
                    reset_to_kickoff = reset;
                    ClockStatus::Stopped
                }
                (false, reset) => {
                    reset_to_kickoff = reset;
                    clock_stops
                }
            };
        }
        (result, duration, clock_stops, reset_to_kickoff)
    }

    pub fn check_timeout(&mut self) -> Option<HomeAway> {
        if OffensiveTimeoutModel::calls_timeout(&self) {
            let team = self.game_state.play.possession();
            self.game_state.charge_timeout(team);
            return Some(team);
        }
        if DefensiveTimeoutModel::calls_timeout(&self) {
            let team = self.game_state.play.possession().flip();
            self.game_state.charge_timeout(team);
            return Some(team);
        }
        return None;
    }

    /// return true if we should reset state to kickoff after
    pub fn runoff_clock(
        &mut self,
        play_result: &PlayResult,
        play_duration: u16,
        clock_status: ClockStatus,
    ) -> bool {
        if self.end_of_game() {
            return false;
        }

        let runoff = match clock_status {
            ClockStatus::Running => ClockModel::sim_runoff(&self, &play_result, play_duration),
            ClockStatus::Paused => {
                ClockModel::sim_paused_runoff(&self, &play_result, play_duration)
            }
            ClockStatus::Stopped => 0,
        };
        let (_, reset_to_kickoff) = self.game_state.advance_clock(runoff, false, true);
        reset_to_kickoff
    }

    pub fn next_play_state(play: &PlayResult, prev_state: &PlayState) -> (PlayState, ClockStatus) {
        // returns true if this transition stops the playclock
        let (np, clock_runs) = match play {
            PlayResult::Kickoff(_, result) => GameSim::next_kick_state(result, prev_state),
            PlayResult::FieldGoal(result) => match result {
                FieldGoalResult::AttemptedFg(made) => (
                    GameSim::next_field_goal_state(*made, prev_state),
                    ClockStatus::Stopped,
                ),
                FieldGoalResult::Blocked(turnover_outcome) => {
                    GameSim::next_turnover_state(turnover_outcome, prev_state)
                }
            },
            PlayResult::Punt(_, result) => GameSim::next_kick_state(result, prev_state),
            PlayResult::DesignedRun(run_result) => GameSim::next_run_state(run_result, prev_state),
            PlayResult::Dropback(pass_result) => GameSim::next_pass_state(pass_result, prev_state),
            PlayResult::OffensivePenaltyNoPlay(penalty_yards, loss_of_down) => {
                prev_state.offensive_penalty(*penalty_yards, *loss_of_down)
            }
            PlayResult::DefensivePenaltyNoPlay(penalty_yards, auto_first, ignore_half_distance) => {
                prev_state.defensive_penalty(*penalty_yards, *auto_first, *ignore_half_distance)
            }
            PlayResult::Timeout(_) => (prev_state.clone(), ClockStatus::Stopped),
            PlayResult::PointAfterTouchdown(_) => (
                PlayState::Kickoff(prev_state.possession()),
                ClockStatus::Stopped,
            ),
            PlayResult::QbSpike(_) => GameSim::advance_down(prev_state.expect_downtogo(), 0),
            PlayResult::QbKneel(run_result) => GameSim::next_run_state(run_result, prev_state),
        };

        if np.safe_yards_for_touchdown() == 0 {
            panic!(
                "next_play_state: safe_yards_for_touchdown == 0, {} - {}",
                prev_state, play
            );
        }
        (np, clock_runs)
    }

    fn play_result(&self, play_call: &PlayType) -> PlayResult {
        match play_call {
            PlayType::Down(playcall_result) => match playcall_result {
                PlaycallResult::DefensivePenalty => PenaltyModel::sim_defensive_penalty(self),
                PlaycallResult::OffensivePenalty => PenaltyModel::sim_offensive_penalty(self),
                PlaycallResult::FieldGoalAttempt => self.field_goal_result(),
                PlaycallResult::Punt => {
                    // let pr = self.punt_result();
                    // log::info!("punt result: {}", pr);
                    // pr
                    self.punt_result()
                }
                PlaycallResult::QbSpike => self.qb_spike_result(),
                PlaycallResult::QbKneel => self.qb_kneel_result(),
                PlaycallResult::QbDropback => PlayResult::Dropback(self.dropback_result()),
                PlaycallResult::DesignedRush => PlayResult::DesignedRun(self.rush_result()),
            },
            PlayType::Kickoff => {
                // let kr = self.kickoff_result();
                // log::info!("kickoff result: {}", kr);
                // kr
                self.kickoff_result()
            }
            PlayType::PuntAfterSafety => self.punt_after_safety_result(),
            PlayType::PointAfterTouchdown => {
                PlayResult::PointAfterTouchdown(self.point_after_touchdown_result())
            }
        }
    }

    pub fn expect_downtogo(&self) -> DownToGo {
        self.game_state.play.expect_downtogo()
    }

    fn dropback_result(&self) -> DropbackResult {
        let outcome = DropbackModel::simulate_dropback(self);
        let penalty = PostPassPenaltyModel::sample_post_pass_penalty(&self, &outcome);
        DropbackResult {
            passer_id: self.passer_id(),
            outcome,
            penalty,
        }
    }

    fn passer_id(&self) -> String {
        self.offense_params().qbs[0].player_id.clone()
    }

    fn rush_result(&self) -> RunResult {
        RushingModel::sim_designed_run(self)
    }

    fn punt_result(&self) -> PlayResult {
        PlayResult::Punt(
            self.offense_params().team.punt_returner_id.clone(),
            PuntModel::punt_result(self),
        )
    }

    fn field_goal_result(&self) -> PlayResult {
        PlayResult::FieldGoal(FgModel::get_result(self))
    }

    fn kickoff_result(&self) -> PlayResult {
        PlayResult::Kickoff(
            self.offense_params().team.kickoff_returner_id.clone(),
            KickoffModel::get_result(&self),
        )
    }

    fn punt_after_safety_result(&self) -> PlayResult {
        PlayResult::Punt(
            self.offense_params().team.kickoff_returner_id.clone(),
            KickingResult::ReturnedForYards(YardsToGoal::new(65)),
        )
    }

    fn point_after_touchdown_result(&self) -> PATResult {
        match self.pat_goes_for_two() {
            true => self.pat_two_point_result(),
            false => self.pat_kick_result(),
        }
    }

    fn pat_goes_for_two(&self) -> bool {
        TwoPointAttemptModel::goes_for_2(self)
    }

    fn pat_kick_result(&self) -> PATResult {
        match FgModel::get_result(&self) {
            FieldGoalResult::AttemptedFg(made) => PATResult::KickAttempted(match made {
                true => PATKickingOutcome::KickIsGood,
                false => PATKickingOutcome::KickMisses,
            }),
            FieldGoalResult::Blocked(turnover_outcome) => match turnover_outcome {
                TurnoverOutcome::Touchdown => {
                    PATResult::KickAttempted(play_result::PATKickingOutcome::DefensiveSafetyReturn)
                }
                TurnoverOutcome::YardsToGoal(_) => {
                    PATResult::KickAttempted(play_result::PATKickingOutcome::BlockedKickMisses)
                }
                TurnoverOutcome::DefensiveSafetyReturn => {
                    PATResult::KickAttempted(play_result::PATKickingOutcome::DefensiveSafetyReturn)
                }
            },
        }
    }

    fn pat_two_point_result(&self) -> PATResult {
        match PlaycallModel::is_qb_dropback(&PlaycallFeatures::new(self)) {
            true => {
                let dropback_result = self.dropback_result();
                dropback_result.to_2pt_dropback_result()
            }
            false => {
                let run_result = self.rush_result();
                PATResult::TwoPointDesignedRun(run_result.to_2pt_run_outcome())
            }
        }
    }

    fn qb_spike_result(&self) -> PlayResult {
        PlayResult::QbSpike(DropbackResult {
            passer_id: self.passer_id(),
            outcome: DropbackOutcome::QbSpike,
            penalty: None,
        })
    }

    fn qb_kneel_result(&self) -> PlayResult {
        PlayResult::QbKneel(RunResult {
            carrier_id: self.passer_id(),
            outcome: RushingOutcome::Yards(-1, false),
            penalty: None,
        })
    }

    /// this is called when there is no INT/FumRec.
    /// here is where we handle turnover on downs
    fn advance_down(down_to_go: DownToGo, yards_gained: i8) -> (PlayState, ClockStatus) {
        let (next_down, change_of_possession) = down_to_go.down.next();

        let offense_ends_at = down_to_go.yards_to_goal - yards_gained;
        let next_down_to_go = match change_of_possession {
            true => {
                let defense_starts_at = offense_ends_at.flip();
                DownToGo::first_and_ten(down_to_go.possession.flip(), defense_starts_at)
            }
            false => {
                // log::info!(
                //     "ytg = {:?}. ygained = {:?}",
                //     down_to_go.yards_to_goal,
                //     yards_gained
                // );
                match down_to_go.to_go {
                    ToGo::Yards(to_go) => match yards_gained >= to_go as i8 {
                        true => DownToGo::first_and_ten(down_to_go.possession, offense_ends_at),
                        // false means no first down, so use next down from before
                        false => DownToGo {
                            possession: down_to_go.possession,
                            down: next_down,
                            yards_to_goal: offense_ends_at,
                            to_go: ToGo::Yards((to_go as i8 - yards_gained) as u8),
                        },
                    },
                    ToGo::Goal => DownToGo {
                        possession: down_to_go.possession,
                        down: next_down,
                        to_go: ToGo::Goal,
                        yards_to_goal: offense_ends_at,
                    },
                }
            }
        };
        let clock_status = match change_of_possession {
            true => ClockStatus::Stopped,
            false => ClockStatus::Running,
        };
        (PlayState::Down(next_down_to_go), clock_status)
    }

    fn apply_post_snap_penalty(
        next_play_state: PlayState,
        clock_status: ClockStatus,
        penalty: &Option<PenaltyType>,
    ) -> (PlayState, ClockStatus) {
        match penalty {
            Some(penalty) => match penalty {
                PenaltyType::Defensive(dp) => next_play_state.defensive_penalty(
                    dp.yards,
                    dp.automatic_first,
                    dp.ignore_half_distance,
                ),
                PenaltyType::Offensive(op) => {
                    next_play_state.offensive_penalty(op.yards, op.loss_of_down)
                }
            },
            None => (next_play_state, clock_status),
        }
    }

    fn next_run_state(run_result: &RunResult, prev_state: &PlayState) -> (PlayState, ClockStatus) {
        let (next_play_state, clock_status) = match &run_result.outcome {
            RushingOutcome::Yards(yards_gained, _) => {
                let down_to_go = prev_state.expect_downtogo();
                GameSim::advance_down(down_to_go, *yards_gained)
            }
            RushingOutcome::Touchdown => (
                PlayState::PointAfterTouchdown(prev_state.possession()),
                ClockStatus::Stopped,
            ),
            RushingOutcome::FumbleLost(_, turnover_outcome) => match turnover_outcome {
                TurnoverOutcome::Touchdown => (
                    PlayState::PointAfterTouchdown(prev_state.possession().flip()),
                    ClockStatus::Stopped,
                ),
                TurnoverOutcome::YardsToGoal(yards_to_goal) => (
                    PlayState::Down(DownToGo::first_and_ten(
                        prev_state.possession().flip(),
                        *yards_to_goal,
                    )),
                    ClockStatus::Stopped,
                ),
                TurnoverOutcome::DefensiveSafetyReturn => {
                    panic!("Cannot have safety after fumble recovery")
                }
            },
            RushingOutcome::Safety => (
                PlayState::PuntAfterSafety(prev_state.possession()),
                ClockStatus::Stopped,
            ),
        };
        GameSim::apply_post_snap_penalty(next_play_state, clock_status, &run_result.penalty)
    }

    fn next_turnover_state(
        turnover_outcome: &TurnoverOutcome,
        prev_state: &PlayState,
    ) -> (PlayState, ClockStatus) {
        let play_state = match turnover_outcome {
            TurnoverOutcome::Touchdown => PlayState::PointAfterTouchdown(prev_state.possession()),
            TurnoverOutcome::YardsToGoal(yards_to_goal) => PlayState::Down(
                DownToGo::first_and_ten(prev_state.possession().flip(), *yards_to_goal),
            ),
            TurnoverOutcome::DefensiveSafetyReturn => PlayState::Kickoff(prev_state.possession()),
        };
        (play_state, ClockStatus::Stopped)
    }

    fn next_pass_state(
        pass_result: &DropbackResult,
        prev_state: &PlayState,
    ) -> (PlayState, ClockStatus) {
        let (next_play_state, clock_status) = match &pass_result.outcome {
            DropbackOutcome::Throwaway => GameSim::advance_down(prev_state.expect_downtogo(), 0),
            DropbackOutcome::QbSpike => GameSim::advance_down(prev_state.expect_downtogo(), 0),
            DropbackOutcome::QbScramble(run_result) => {
                GameSim::next_run_state(&run_result, prev_state)
            }
            DropbackOutcome::Target(target_result) => match &target_result.outcome {
                TargetOutcome::Incomplete(_) => {
                    let down_to_go = prev_state.expect_downtogo();
                    GameSim::advance_down(down_to_go, 0)
                }
                TargetOutcome::Yards(yards, _) => {
                    let down_to_go = prev_state.expect_downtogo();
                    GameSim::advance_down(down_to_go, yards.total())
                }
                TargetOutcome::CatchThenFumble(_, turnover_outcome) => {
                    GameSim::next_turnover_state(&turnover_outcome, prev_state)
                }
                TargetOutcome::Touchdown(_) => (
                    PlayState::PointAfterTouchdown(prev_state.possession()),
                    ClockStatus::Stopped,
                ),
                TargetOutcome::Interception(_, turnover_outcome) => {
                    GameSim::next_turnover_state(&turnover_outcome, prev_state)
                }
            },
            DropbackOutcome::Sack(sack_outcome) => match sack_outcome {
                SackOutcome::Safety => (
                    PlayState::PuntAfterSafety(prev_state.possession()),
                    ClockStatus::Stopped,
                ),
                SackOutcome::YardsLost(yards_lost) => {
                    let down_to_go = prev_state.expect_downtogo();
                    GameSim::advance_down(down_to_go, -1 * (*yards_lost as i8))
                }
                SackOutcome::FumbleLost(turnover_outcome) => {
                    GameSim::next_turnover_state(&turnover_outcome, prev_state)
                }
            },
        };
        GameSim::apply_post_snap_penalty(next_play_state, clock_status, &pass_result.penalty)
    }

    fn next_field_goal_state(made: bool, prev_state: &PlayState) -> PlayState {
        let down_to_go = prev_state.expect_downtogo();
        match made {
            true => PlayState::Kickoff(down_to_go.possession),
            false => PlayState::Down(DownToGo::first_and_ten(
                down_to_go.possession.flip(),
                down_to_go.yards_to_goal.flip() - FG_SNAP_DISTANCE,
            )),
        }
    }

    fn next_kick_state(result: &KickingResult, prev_state: &PlayState) -> (PlayState, ClockStatus) {
        let play_state = match result {
            KickingResult::FumbleLost(turnover_outcome) => match turnover_outcome {
                TurnoverOutcome::Touchdown => {
                    PlayState::PointAfterTouchdown(prev_state.kicking_team())
                }
                TurnoverOutcome::YardsToGoal(yards_to_goal) => PlayState::Down(
                    DownToGo::first_and_ten(prev_state.kicking_team(), *yards_to_goal),
                ),
                TurnoverOutcome::DefensiveSafetyReturn => {
                    panic!("Cannot have defensive safety return after fumble recovery")
                }
            },
            KickingResult::PuntTouchback => PlayState::Down(DownToGo::first_and_ten(
                prev_state.returning_team(),
                YardsToGoal::touchback(),
            )),
            KickingResult::KickoffTouchback => PlayState::Down(DownToGo::first_and_ten(
                prev_state.returning_team(),
                YardsToGoal::kickoff_touchback(),
            )),
            KickingResult::ReturnedForTouchdown => {
                PlayState::PointAfterTouchdown(prev_state.returning_team())
            }
            KickingResult::ReturnedForYards(yards_to_goal) => PlayState::Down(
                DownToGo::first_and_ten(prev_state.returning_team(), *yards_to_goal),
            ),
            KickingResult::OnsideRecovery(yards_to_goal) => PlayState::Down(
                DownToGo::first_and_ten(prev_state.kicking_team(), *yards_to_goal),
            ),
            KickingResult::Blocked(turnover_outcome) => match turnover_outcome {
                TurnoverOutcome::Touchdown => {
                    PlayState::PointAfterTouchdown(prev_state.kicking_team().flip())
                }
                TurnoverOutcome::YardsToGoal(yards_to_goal) => PlayState::Down(
                    DownToGo::first_and_ten(prev_state.kicking_team().flip(), *yards_to_goal),
                ),
                TurnoverOutcome::DefensiveSafetyReturn => {
                    PlayState::Kickoff(prev_state.kicking_team().flip())
                }
            },
        };

        (play_state, ClockStatus::Stopped)
    }

    pub fn offense_params(&self) -> &TeamParams {
        match self.game_state.play.possession() {
            HomeAway::Home => &self.game_params.home,
            HomeAway::Away => &self.game_params.away,
        }
    }

    pub fn defense_params(&self) -> &TeamParams {
        match self.game_state.play.possession() {
            HomeAway::Home => &self.game_params.away,
            HomeAway::Away => &self.game_params.home,
        }
    }
}

pub fn sim_game(game_params: &GameParamsDistribution) -> BoxScore {
    let game_start = GameStart::new();
    let kicks_h2_start = game_start.received_h1.clone();
    let game_state = GameState::new(game_start);
    let mut sim = GameSim::new(game_params.to_game_params(), game_state);
    // log::info!("\n\nbeginning of game");
    // let (mut last_play, mut last_result) = (
    //     sim.game_state.play.clone(),
    //     PlayResult::Timeout(HomeAway::Away),
    // );
    while !sim.end_of_game() {
        let (result, play_duration, clock_stops, rtk_end_play) = sim.next_play();

        let is_timeout = match sim.check_timeout() {
            None => false,
            Some(team) => {
                log::debug!("Timeout called by {}\n", team);
                true
            }
        };

        let clock_status = ClockStatus::merge(clock_stops, result.clock_status_after(), is_timeout)
            .transform_clock(&sim.game_state.clock);
        // clock does not run if play type indicates we don't stop clock,
        // or there's a timeout with < 3 minutes left in the half
        let rtk_run_clock = match clock_status {
            ClockStatus::Stopped => {
                if is_timeout
                    && clock_stops != ClockStatus::Stopped
                    && sim.game_state.clock.half_minutes_remaining() > 3.0
                    && sim.game_state.clock.seconds_remaining >= PLAYCLOCK as u16
                {
                    // if there's a timeout with > 3 minutes left in the half,
                    // assume that they are taking a timeout to prevent
                    // a delay-of-game penalty. in this case, run off the maximum time
                    // from the clock
                    sim.game_state
                        .advance_clock(PLAYCLOCK as u16 - 1, false, true);
                }
                false
            }
            _ => match rtk_end_play {
                false => sim.runoff_clock(&result, play_duration, clock_status),
                true => false,
            },
        };
        if rtk_end_play | rtk_run_clock {
            let kicking_team = match sim.game_state.clock.quarter {
                Quarter::Third => kicks_h2_start,
                Quarter::Overtime => match random_bool(0.5) {
                    true => kicks_h2_start.flip(),
                    false => kicks_h2_start,
                },
                _ => panic!("cannot reset to kickoff outside Q3/OT"),
            };
            sim.game_state.play = PlayState::Kickoff(kicking_team);
        }
        // if sim.game_state.play.safe_yards_for_touchdown() == 0 {
        //     log::info!("{} - {}", last_play, last_result);
        // }
        // last_play = sim.game_state.play.clone();
        // last_result = result;
    }
    log::debug!(
        "\nend of game. score: {} - {}\n\n",
        sim.box_score.score.home,
        sim.box_score.score.away
    );
    sim.box_score
}
