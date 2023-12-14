use std::collections::HashMap;

use crate::{
    box_score::{
        defense::{DefenseBoxScore, DefenseProjection, OffenseProjection, SpecialTeamsProjection},
        kicking::{KickerBoxScore, KickerProjection},
        passing::{PassingBoxScore, PassingProjection},
        skill_player::{SkillPlayerBoxScore, SkillPlayerProjection},
    },
    params::{skill_player::Position, GameParams},
    sim::play_result::{
        DropbackOutcome, DropbackResult, FieldGoalResult, KickingResult, PATDropbackOutcome,
        PATKickingOutcome, PATResult, PATRushingOutcome, PenaltyType, PlayResult, RunResult,
        RushingOutcome, SackOutcome, TargetOutcome, TurnoverOutcome,
    },
    start::HomeAway,
    state::{
        down::PlayState,
        game_state::{Plays, Score},
        yards_to_goal::YardsToGoal,
    },
};

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerFantasyPoints {
    pub draftkings: f32,
    pub fanduel: f32,
    pub full_ppr: f32,
    pub half_ppr: f32,
    pub standard: f32,
}

impl PlayerFantasyPoints {
    pub fn copy_standard(standard: f32) -> PlayerFantasyPoints {
        PlayerFantasyPoints {
            draftkings: standard,
            fanduel: standard,
            full_ppr: standard,
            half_ppr: standard,
            standard: standard,
        }
    }

    pub fn add_points(&mut self, n_sims: u32, points: PlayerFantasyPoints) {
        let n = n_sims as f32;

        self.draftkings += points.draftkings / n;
        self.fanduel += points.fanduel / n;
        self.full_ppr += points.full_ppr / n;
        self.half_ppr += points.half_ppr / n;
        self.standard += points.standard / n;
    }
}

pub struct FantasyPoints {
    pub players: HashMap<String, PlayerFantasyPoints>,
    pub defenses: HashMap<String, PlayerFantasyPoints>,
    pub kickers: HashMap<String, PlayerFantasyPoints>,
}

impl FantasyPoints {
    pub fn combine(fps: &Vec<FantasyPoints>) -> FantasyPoints {
        let mut combined_fp = FantasyPoints {
            players: HashMap::new(),
            defenses: HashMap::new(),
            kickers: HashMap::new(),
        };
        for fp in fps {
            for (p, pfp) in fp.players.clone() {
                combined_fp.players.insert(p, pfp);
            }
            for (d, dfp) in fp.defenses.clone() {
                combined_fp.defenses.insert(d, dfp);
            }
            for (k, kfp) in fp.kickers.clone() {
                combined_fp.kickers.insert(k, kfp);
            }
        }
        combined_fp
    }
}

#[derive(Debug, Clone)]
pub struct BoxScore {
    pub score: Score,
    pub plays: Plays,
    pub passers: HashMap<String, PassingBoxScore>,
    pub skill_players: HashMap<String, SkillPlayerBoxScore>,
    // team abbrev -> score
    pub defenses: HashMap<String, DefenseBoxScore>,
    pub kickers: HashMap<String, KickerBoxScore>,
    // team -> yardline_100 -> n_plays
    pub field_position: HashMap<String, HashMap<u8, u16>>,
    pub home: String,
    pub away: String,
}

impl BoxScore {
    pub fn new(home: String, away: String, params: &GameParams) -> BoxScore {
        let mut field_position = HashMap::new();
        field_position.insert(home.clone(), HashMap::new());
        field_position.insert(away.clone(), HashMap::new());
        BoxScore {
            score: Score::new(),
            plays: Plays::new(),
            passers: PassingBoxScore::new_map(params),
            skill_players: SkillPlayerBoxScore::new_map(params),
            defenses: DefenseBoxScore::new_map(home.clone(), away.clone()),
            kickers: KickerBoxScore::new_map(home.clone(), away.clone()),
            field_position,
            home,
            away,
        }
    }

    pub fn fantasy_points(&self) -> FantasyPoints {
        let mut fp = FantasyPoints {
            players: HashMap::new(),
            defenses: self
                .defenses
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        PlayerFantasyPoints::copy_standard(v.standard_fantasy_points()),
                    )
                })
                .collect(),
            kickers: self
                .kickers
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        PlayerFantasyPoints::copy_standard(v.standard_fantasy_points()),
                    )
                })
                .collect(),
        };
        for (player_id, _) in self.skill_players.iter() {
            fp.players
                .insert(player_id.clone(), self.skill_fantasy_points(player_id));
        }
        for (player_id, _) in self.passers.iter() {
            match fp.players.get_mut(player_id) {
                Some(player_fp) => {
                    let passer_fp = self.passing_fantasy_points(player_id);
                    player_fp.draftkings += passer_fp.draftkings;
                    player_fp.fanduel += passer_fp.fanduel;
                    player_fp.full_ppr += passer_fp.full_ppr;
                    player_fp.half_ppr += passer_fp.half_ppr;
                    player_fp.standard += passer_fp.standard;
                }
                None => {
                    fp.players
                        .insert(player_id.clone(), self.passing_fantasy_points(player_id));
                }
            }
        }
        fp
    }

    pub fn player_fantasy_points(&self, player_id: &String) -> PlayerFantasyPoints {
        let passing = self.passing_fantasy_points(player_id);
        let skill = self.skill_fantasy_points(player_id);
        let half_ppr = passing.half_ppr + skill.half_ppr;
        PlayerFantasyPoints {
            draftkings: passing.draftkings + skill.draftkings,
            fanduel: half_ppr,
            full_ppr: passing.full_ppr + skill.full_ppr,
            half_ppr,
            standard: passing.standard + skill.standard,
        }
    }

    pub fn passing_fantasy_points(&self, player_id: &String) -> PlayerFantasyPoints {
        let (standard, fumbles_lost, dk_pass_bonus) = match self.passers.get(player_id) {
            Some(box_score) => (
                box_score.passing_points(),
                box_score.fumbles_lost as f32,
                if box_score.yards >= 300 { 3.0 } else { 0.0 },
            ),
            None => (0.0, 0.0, 0.0),
        };

        let half_ppr = standard;
        let full_ppr = standard;

        PlayerFantasyPoints {
            // fumbles lost is only -1 on draftkings and -2 elsewhere
            draftkings: full_ppr + dk_pass_bonus + fumbles_lost,
            fanduel: half_ppr,
            full_ppr,
            half_ppr,
            standard,
        }
    }

    pub fn skill_fantasy_points(&self, player_id: &String) -> PlayerFantasyPoints {
        let (standard, receptions, fumbles_lost, dk_skill_bonus) =
            match self.skill_players.get(player_id) {
                Some(box_score) => {
                    let dk_rush_bonus = match box_score.rushing_yards >= 100 {
                        true => 3.0,
                        false => 0.0,
                    };
                    let dk_rec_bonus = match box_score.receiving_yards >= 100 {
                        true => 3.0,
                        false => 0.0,
                    };

                    (
                        box_score.standard_fantasy_points(),
                        box_score.catches as f32,
                        box_score.fumbles_lost as f32,
                        dk_rush_bonus + dk_rec_bonus,
                    )
                }
                None => (0.0, 0.0, 0.0, 0.0),
            };

        let half_ppr = standard + 0.5 * receptions;
        let full_ppr = standard + 1.0 * receptions;

        PlayerFantasyPoints {
            // fumbles lost is only -1 on draftkings and -2 elsewhere
            draftkings: full_ppr + dk_skill_bonus + fumbles_lost,
            fanduel: half_ppr,
            full_ppr,
            half_ppr,
            standard,
        }
    }

    pub fn add_points(&mut self, points: u8, home_away: &HomeAway, against_defense: bool) {
        match home_away {
            HomeAway::Away => {
                self.score.away += points;
            }
            HomeAway::Home => {
                self.score.home += points;
            }
        }
        if against_defense {
            let dst = self
                .defenses
                .get_mut(&self.team_name(home_away.flip()))
                .unwrap();
            dst.points_allowed += points;
        }
    }

    fn team_name(&self, home_away: HomeAway) -> String {
        match home_away {
            HomeAway::Away => self.away.clone(),
            HomeAway::Home => self.home.clone(),
        }
    }

    fn apply_kick_block(&mut self, turnover_outcome: &TurnoverOutcome, defense: HomeAway) {
        let team = self.team_name(defense);
        let box_score = self.defenses.get_mut(&team).unwrap();
        box_score.add_kick_block(turnover_outcome);
    }

    fn apply_offensive_penalty(&mut self, prev_state: &PlayState, raw_yards: &u8) {
        let team = self.team_name(prev_state.possession());
        let box_score = self.defenses.get_mut(&team).unwrap();
        let penalty_yards = prev_state.adjusted_offensive_penalty_yards(*raw_yards);
        box_score.offensive_penalties += 1;
        box_score.offensive_penalty_yards += penalty_yards;
    }

    fn apply_defensive_penalty(
        &mut self,
        prev_state: &PlayState,
        raw_yards: &u8,
        ignore_half_distance: bool,
    ) {
        let team = self.team_name(prev_state.defense());
        let box_score = self.defenses.get_mut(&team).unwrap();
        let penalty_yards =
            prev_state.adjusted_defensive_penalty_yards(*raw_yards, ignore_half_distance);
        box_score.defensive_penalties += 1;
        box_score.defensive_penalty_yards += penalty_yards;
    }

    fn apply_penalty_opt(&mut self, penalty_opt: &Option<PenaltyType>, prev_state: &PlayState) {
        if let Some(penalty) = penalty_opt {
            match penalty {
                PenaltyType::Offensive(op) => {
                    self.apply_offensive_penalty(prev_state, &op.yards);
                }
                PenaltyType::Defensive(dp) => {
                    self.apply_defensive_penalty(prev_state, &dp.yards, dp.ignore_half_distance);
                }
            }
        }
    }

    pub fn apply_stats(&mut self, play: &PlayResult, prev_state: &PlayState) {
        match play {
            PlayResult::Kickoff(returner_id, outcome) => {
                self.apply_return_stats(returner_id, outcome, prev_state);
            }
            PlayResult::FieldGoal(fg_result) => match fg_result {
                FieldGoalResult::AttemptedFg(made) => self.apply_fg_stats(*made, prev_state),
                FieldGoalResult::Blocked(turnover_outcome) => {
                    self.apply_kick_block(turnover_outcome, prev_state.defense())
                }
            },
            PlayResult::Punt(returner_id, outcome) => {
                let team = self.team_name(prev_state.kicking_team());
                let box_score = self.kickers.get_mut(&team).unwrap();
                box_score.add_punt();

                self.apply_return_stats(returner_id, outcome, prev_state);
                match outcome {
                    KickingResult::Blocked(turnover_outcome) => {
                        self.apply_kick_block(turnover_outcome, prev_state.defense())
                    }
                    _ => {}
                }
            }
            PlayResult::OffensivePenaltyNoPlay(yards, _) => {
                self.apply_offensive_penalty(prev_state, yards);
            }
            PlayResult::DefensivePenaltyNoPlay(yards, _, ignore_half_distance) => {
                self.apply_defensive_penalty(prev_state, yards, *ignore_half_distance);
            }
            PlayResult::PointAfterTouchdown(result) => match result {
                PATResult::KickAttempted(outcome) => {
                    let is_good = *outcome == PATKickingOutcome::KickIsGood;
                    self.add_kicker_pat_attempt(prev_state.kicking_team(), is_good);
                    match outcome {
                        PATKickingOutcome::KickIsGood => {
                            self.add_points(1, &prev_state.possession(), true);
                        }
                        PATKickingOutcome::KickMisses => {}
                        PATKickingOutcome::DefensiveSafetyReturn => self.apply_kick_block(
                            &TurnoverOutcome::DefensiveSafetyReturn,
                            prev_state.defense(),
                        ),
                        PATKickingOutcome::BlockedKickMisses => self.apply_kick_block(
                            &TurnoverOutcome::YardsToGoal(YardsToGoal(80)),
                            prev_state.defense(),
                        ),
                    }
                }
                PATResult::TwoPointDesignedRun(outcome) => match outcome {
                    PATRushingOutcome::DefensiveSafetyReturn => {
                        let box_score = self
                            .defenses
                            .get_mut(&self.team_name(prev_state.defense()))
                            .unwrap();
                        box_score.add_safety();
                        self.add_points(2, &prev_state.defense(), false);
                    }
                    PATRushingOutcome::Fail => {}
                    PATRushingOutcome::Success(carrier_id) => {
                        self.apply_rushing_two_point_conversion(
                            carrier_id,
                            &prev_state.possession(),
                        );
                    }
                },
                PATResult::TwoPointDropback(outcome) => match outcome {
                    PATDropbackOutcome::DefensiveSafetyReturn => {}
                    PATDropbackOutcome::Fail => {}
                    PATDropbackOutcome::SuccessfulCompletion(passer_id, receiver_id) => {
                        match self.passers.get_mut(passer_id) {
                            Some(box_score) => box_score.add_two_point_conversion(),
                            None => {
                                let mut box_score = PassingBoxScore::new();
                                box_score.add_two_point_conversion();
                                self.passers.insert(passer_id.clone(), box_score);
                            }
                        }
                        match self.skill_players.get_mut(receiver_id) {
                            Some(box_score) => box_score.add_two_point_conversion(),
                            None => {
                                let mut box_score = SkillPlayerBoxScore::new();
                                box_score.add_two_point_conversion();
                                self.skill_players.insert(receiver_id.clone(), box_score);
                            }
                        }
                        self.add_points(2, &prev_state.possession(), true);
                    }
                    PATDropbackOutcome::SuccessfulScramble(carrier_id) => self
                        .apply_rushing_two_point_conversion(carrier_id, &prev_state.possession()),
                },
            },
            PlayResult::Timeout(_) => {}
            PlayResult::Dropback(dropback_result) => {
                self.apply_pass_stats(dropback_result, prev_state);
                // if dropback_result.penalty.is_some() {
                //     log::info!("dropback result: {:?}", dropback_result.outcome);
                // }
                self.apply_penalty_opt(&dropback_result.penalty, prev_state);
            }
            PlayResult::DesignedRun(run_result) => {
                self.apply_run_stats(run_result, prev_state);
                // if run_result.penalty.is_some() {
                //     log::info!("run result: {:?}", run_result.outcome);
                // }
                self.apply_penalty_opt(&run_result.penalty, prev_state);
            }
            PlayResult::QbSpike(dropback_result) => {
                self.apply_pass_stats(dropback_result, prev_state)
            }
            PlayResult::QbKneel(run_result) => {
                self.add_qb_kneel(run_result);
                self.apply_run_stats(run_result, prev_state)
            }
        }
    }

    fn apply_run_stats(&mut self, run_result: &RunResult, prev_state: &PlayState) {
        // TODO: add post rush penalties
        // TODO: add post-non-target penalties e.g. sacks
        self.plays.increment(
            prev_state.possession(),
            false,
            false,
            prev_state.expect_downtogo(),
        );
        self.apply_run_result(run_result, prev_state);
    }

    fn apply_run_result(&mut self, run_result: &RunResult, prev_state: &PlayState) {
        match self.skill_players.get_mut(&run_result.carrier_id) {
            Some(box_score) => {
                box_score.update_rushing(&run_result.outcome, prev_state);
            }
            None => {
                let mut box_score = SkillPlayerBoxScore::new();
                box_score.update_rushing(&run_result.outcome, prev_state);
                self.skill_players
                    .insert(run_result.carrier_id.clone(), box_score);
            }
        }
        match run_result.outcome {
            RushingOutcome::Touchdown => {
                self.add_points(6, &prev_state.possession(), true);
            }
            RushingOutcome::Safety => {
                self.add_points(2, &prev_state.possession().flip(), false);
            }
            _ => {}
        }
    }

    fn add_kicker_pat_attempt(&mut self, kicking_team: HomeAway, is_good: bool) {
        let box_score = self.kickers.get_mut(&self.team_name(kicking_team)).unwrap();
        box_score.add_pat_attempt(is_good);
    }

    fn apply_pass_stats(&mut self, dropback_result: &DropbackResult, prev_state: &PlayState) {
        let is_target = match &dropback_result.outcome {
            DropbackOutcome::Target(_) => true,
            _ => false,
        };
        self.plays.increment(
            prev_state.possession(),
            true,
            is_target,
            prev_state.expect_downtogo(),
        );
        self.apply_passer_stats(dropback_result);
        self.apply_receiver_stats(dropback_result);
        self.apply_defense_stats(dropback_result, prev_state);
        match &dropback_result.outcome {
            DropbackOutcome::Sack(SackOutcome::Safety) => {
                self.add_points(2, &prev_state.possession().flip(), false);
            }
            DropbackOutcome::Sack(SackOutcome::FumbleLost(_)) => {
                match self.skill_players.get_mut(&dropback_result.passer_id) {
                    Some(box_score) => box_score.add_fumble_lost(),
                    None => {
                        let mut box_score = SkillPlayerBoxScore::new();
                        box_score.add_fumble_lost();
                        self.skill_players
                            .insert(dropback_result.passer_id.clone(), box_score);
                    }
                }
            }
            DropbackOutcome::Target(target_result) => match &target_result.outcome {
                TargetOutcome::Touchdown(_) => {
                    self.add_points(6, &prev_state.possession(), true);
                }
                TargetOutcome::Interception(_, TurnoverOutcome::Touchdown) => {
                    self.add_points(6, &prev_state.defense(), false);
                }
                _ => {}
            },
            DropbackOutcome::QbScramble(run_result) => {
                self.add_qb_scramble(run_result);
                self.apply_run_result(run_result, prev_state)
            }
            _ => {}
        }
    }

    fn add_qb_kneel(&mut self, run_result: &RunResult) {
        match self.passers.get_mut(&run_result.carrier_id) {
            Some(box_score) => {
                box_score.add_kneel();
            }
            None => {
                let mut box_score = PassingBoxScore::new();
                box_score.add_kneel();
                self.passers
                    .insert(run_result.carrier_id.clone(), box_score);
            }
        }
    }

    fn add_qb_scramble(&mut self, run_result: &RunResult) {
        match self.passers.get_mut(&run_result.carrier_id) {
            Some(box_score) => {
                box_score.add_scramble();
            }
            None => {
                let mut box_score = PassingBoxScore::new();
                box_score.add_scramble();
                self.passers
                    .insert(run_result.carrier_id.clone(), box_score);
            }
        }
    }

    fn apply_passer_stats(&mut self, pass_result: &DropbackResult) {
        match self.passers.get_mut(&pass_result.passer_id) {
            Some(box_score) => {
                box_score.update(&pass_result.outcome);
            }
            None => {
                let mut box_score = PassingBoxScore::new();
                box_score.update(&pass_result.outcome);
                self.passers
                    .insert(pass_result.passer_id.clone(), box_score);
            }
        }
    }

    fn apply_defense_stats(&mut self, pass_result: &DropbackResult, prev_state: &PlayState) {
        match &pass_result.outcome {
            DropbackOutcome::Sack(sack_outcome) => {
                let box_score = self
                    .defenses
                    .get_mut(&self.team_name(prev_state.defense()))
                    .unwrap();
                box_score.add_sack();
                match sack_outcome {
                    SackOutcome::Safety => box_score.add_safety(),
                    SackOutcome::FumbleLost(turnover_outcome) => {
                        box_score.add_fumble_recovery(&turnover_outcome);
                    }
                    SackOutcome::YardsLost(_) => {}
                }
            }
            DropbackOutcome::Target(target_result) => match &target_result.outcome {
                TargetOutcome::Interception(_, turnover_outcome) => {
                    let box_score = self
                        .defenses
                        .get_mut(&self.team_name(prev_state.defense()))
                        .unwrap();
                    box_score.add_interception(&turnover_outcome);
                }
                TargetOutcome::CatchThenFumble(_, turnover_outcome) => {
                    let box_score = self
                        .defenses
                        .get_mut(&self.team_name(prev_state.defense()))
                        .unwrap();
                    box_score.add_fumble_recovery(&turnover_outcome);
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn apply_receiver_stats(&mut self, dropback_result: &DropbackResult) {
        match &dropback_result.outcome {
            DropbackOutcome::Target(target_result) => {
                let receiver_id = &target_result.targeted_receiver_id;
                match self.skill_players.get_mut(receiver_id) {
                    Some(box_score) => {
                        box_score.update_receiving(&target_result.outcome);
                    }
                    None => {
                        let mut box_score = SkillPlayerBoxScore::new();
                        box_score.update_receiving(&target_result.outcome);
                        self.skill_players.insert(receiver_id.clone(), box_score);
                    }
                }
            }
            _ => {}
        }
    }

    fn apply_fg_stats(&mut self, made: bool, prev_state: &PlayState) {
        let distance = prev_state.field_goal_distance();
        let box_score = self
            .kickers
            .get_mut(&self.team_name(prev_state.kicking_team()))
            .unwrap();
        match made {
            true => {
                box_score.add_made_fg(distance);
                self.add_points(3, &prev_state.kicking_team(), true);
            }
            false => box_score.add_missed_fg(distance),
        }
    }

    fn apply_return_td(&mut self, returner_id_opt: &Option<String>, returning_team: &String) {
        let def_box_score = self.defenses.get_mut(returning_team).unwrap();
        def_box_score.touchdowns += 1;

        if let Some(returner_id) = returner_id_opt {
            match self.skill_players.get_mut(returner_id) {
                Some(box_score) => box_score.add_return_touchdown(),
                None => {
                    let mut box_score = SkillPlayerBoxScore::new();
                    box_score.add_return_touchdown();
                    self.skill_players.insert(returner_id.clone(), box_score);
                }
            }
        }
    }

    fn apply_return_stats(
        &mut self,
        returner_id_opt: &Option<String>,
        outcome: &KickingResult,
        prev_state: &PlayState,
    ) {
        match outcome {
            KickingResult::ReturnedForTouchdown => {
                self.apply_return_td(
                    returner_id_opt,
                    &self.team_name(prev_state.returning_team()),
                );
                self.add_points(6, &prev_state.kicking_team().flip(), true);
            }
            KickingResult::FumbleLost(turnover_outcome) => {
                if *turnover_outcome == TurnoverOutcome::Touchdown {
                    self.add_points(6, &prev_state.kicking_team(), true);
                }
                let def_box_score = self
                    .defenses
                    .get_mut(&self.team_name(prev_state.defense()))
                    .unwrap();
                def_box_score.add_fumble_recovery(turnover_outcome);
                if let Some(returner_id) = returner_id_opt {
                    match self.skill_players.get_mut(returner_id) {
                        Some(box_score) => box_score.add_fumble_lost(),
                        None => {
                            let mut box_score = SkillPlayerBoxScore::new();
                            box_score.add_fumble_lost();
                            self.skill_players.insert(returner_id.clone(), box_score);
                        }
                    }
                }
            }
            KickingResult::ReturnedForYards(_) => {}
            KickingResult::PuntTouchback => {}
            KickingResult::KickoffTouchback => {}
            KickingResult::OnsideRecovery(_) => {}
            KickingResult::Blocked(turnover_outcome) => {
                let def_box_score = self
                    .defenses
                    .get_mut(&self.team_name(prev_state.defense()))
                    .unwrap();
                def_box_score.add_fumble_recovery(turnover_outcome);
            }
        }
    }

    fn apply_rushing_two_point_conversion(&mut self, carrier_id: &String, possession: &HomeAway) {
        self.add_points(2, possession, true);
        match self.skill_players.get_mut(carrier_id) {
            Some(box_score) => box_score.add_two_point_conversion(),
            None => {
                let mut box_score = SkillPlayerBoxScore::new();
                box_score.add_two_point_conversion();
                self.skill_players.insert(carrier_id.clone(), box_score);
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct QbProjection {
    pub n_sims: f32,
    pub points: PlayerFantasyPoints,
    pub passing: PassingProjection,
    pub skill: SkillPlayerProjection,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SkillProjection {
    pub n_sims: f32,
    pub points: PlayerFantasyPoints,
    pub skill: SkillPlayerProjection,
}
#[derive(Debug, PartialEq, Clone)]
pub struct TeamProjection {
    pub n_sims: f32,
    pub points: PlayerFantasyPoints,
    pub defense: DefenseProjection,
    pub offense: OffenseProjection,
    pub special_teams: SpecialTeamsProjection,
    pub field_position: HashMap<u8, u64>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct KickingProjection {
    pub n_sims: f32,
    pub points: PlayerFantasyPoints,
    pub kicking: KickerProjection,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd)]
pub enum PlayerKey {
    NflId(String),
    TeamPos(String, Position),
}

impl PlayerKey {
    pub fn expect_team(&self) -> String {
        match self {
            PlayerKey::NflId(_) => panic!("Expected team, got nfl id"),
            PlayerKey::TeamPos(team, _) => team.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PlayerKey::NflId(nfl_id) => nfl_id.clone(),
            PlayerKey::TeamPos(team, pos) => format!("{}-{}", team, pos.to_string()),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Projection {
    Qb(QbProjection),
    Skill(SkillProjection),
    Team(TeamProjection),
    Kicker(KickingProjection),
}

impl Projection {
    pub fn dk_points(&self) -> f32 {
        self.points().draftkings
    }

    pub fn fd_points(&self) -> f32 {
        self.points().fanduel
    }

    pub fn points(&self) -> &PlayerFantasyPoints {
        match self {
            Projection::Qb(projection) => &projection.points,
            Projection::Skill(projection) => &projection.points,
            Projection::Team(projection) => &projection.points,
            Projection::Kicker(projection) => &projection.points,
        }
    }
}

impl QbProjection {
    pub fn new(n_sims: u32) -> QbProjection {
        QbProjection {
            n_sims: n_sims as f32,
            points: PlayerFantasyPoints::copy_standard(0.0),
            passing: PassingProjection::new(),
            skill: SkillPlayerProjection::new(),
        }
    }

    pub fn add(&mut self, stats: &PassingBoxScore) {
        let qb_stats = &mut self.passing;
        qb_stats.attempts += stats.attempts as f32 / self.n_sims;
        qb_stats.completions += stats.completions as f32 / self.n_sims;
        qb_stats.yards += stats.yards as f32 / self.n_sims;
        qb_stats.air_yards += stats.air_yards as f32 / self.n_sims;
        qb_stats.yards_after_catch += stats.yards_after_catch as f32 / self.n_sims;
        qb_stats.touchdowns += stats.touchdowns as f32 / self.n_sims;
        qb_stats.interceptions += stats.interceptions as f32 / self.n_sims;
        qb_stats.two_point_conversions += stats.two_point_conversions as f32 / self.n_sims;
        qb_stats.scrambles += stats.scrambles as f32 / self.n_sims;
        qb_stats.kneels += stats.kneels as f32 / self.n_sims;
        qb_stats.yards_sampled.push(stats.yards);
        qb_stats.tds_sampled.push(stats.touchdowns);
    }
}

impl SkillProjection {
    pub fn new(n_sims: u32) -> SkillProjection {
        SkillProjection {
            n_sims: n_sims as f32,
            points: PlayerFantasyPoints::copy_standard(0.0),
            skill: SkillPlayerProjection::new(),
        }
    }

    pub fn add(&mut self, stats: &SkillPlayerBoxScore) {
        self.skill.add(stats, self.n_sims);
    }
}

impl KickingProjection {
    pub fn new(n_sims: u32) -> KickingProjection {
        KickingProjection {
            n_sims: n_sims as f32,
            points: PlayerFantasyPoints::copy_standard(0.0),
            kicking: KickerProjection::new(),
        }
    }

    pub fn add(&mut self, stats: &KickerBoxScore) {
        let kicking: &mut KickerProjection = &mut self.kicking;

        kicking.pats_made += stats.pats_made as f32 / self.n_sims;
        kicking.pats_attempted += (stats.pats_made + stats.pats_missed) as f32 / self.n_sims;
        for fg_made_distance in &stats.fgs_made {
            match fg_made_distance {
                0..=29 => kicking.fg_made_u30 += 1.0 / self.n_sims,
                30..=39 => kicking.fg_made_30_39 += 1.0 / self.n_sims,
                40..=49 => kicking.fg_made_40_49 += 1.0 / self.n_sims,
                _ => kicking.fg_made_50o += 1.0 / self.n_sims,
            };
            kicking.fgs_made += 1.0 / self.n_sims;
            kicking.fgs_attempted += 1.0 / self.n_sims;
        }
        for _ in &stats.fgs_missed {
            kicking.fgs_attempted += 1.0 / self.n_sims;
        }
        for _ in 0..stats.num_punts {
            kicking.punts += 1.0 / self.n_sims;
        }
    }
}

impl TeamProjection {
    pub fn new(n_sims: u32) -> TeamProjection {
        TeamProjection {
            n_sims: n_sims as f32,
            points: PlayerFantasyPoints::copy_standard(0.0),
            defense: DefenseProjection::new(),
            offense: OffenseProjection::new(),
            special_teams: SpecialTeamsProjection::new(),
            field_position: HashMap::new(),
        }
    }

    pub fn update_field_position(&mut self, fp: &HashMap<u8, u16>) {
        for (k, v) in fp {
            let entry = self.field_position.entry(*k).or_insert(0);
            *entry += *v as u64;
        }
    }
}
