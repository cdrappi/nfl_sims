use crate::{
    start::HomeAway,
    state::{
        clock::{GameClock, Quarter},
        yards_to_goal::YardsToGoal,
    },
};

#[derive(Debug, PartialEq, Clone)]
pub enum TurnoverOutcome {
    Touchdown,
    DefensiveSafetyReturn,
    // from perspective of new offense
    YardsToGoal(YardsToGoal),
}

impl TurnoverOutcome {
    pub fn can_have_post_play_penalty(&self) -> bool {
        match &self {
            TurnoverOutcome::Touchdown => false,
            TurnoverOutcome::DefensiveSafetyReturn => false,
            TurnoverOutcome::YardsToGoal(_) => true,
        }
    }
}

impl std::fmt::Display for TurnoverOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TurnoverOutcome::Touchdown => write!(f, "Returned for Touchdown"),
            TurnoverOutcome::DefensiveSafetyReturn => write!(f, "Defensive Safety Return"),
            TurnoverOutcome::YardsToGoal(yards) => write!(f, "Tackled {} yards from goal", yards),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SackOutcome {
    // yards lost, always will be positive
    YardsLost(u8),
    Safety,
    FumbleLost(TurnoverOutcome),
}

impl std::fmt::Display for SackOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SackOutcome::YardsLost(yards) => write!(f, "Sack for loss of {}", yards),
            SackOutcome::Safety => write!(f, "Sack for Safety"),
            SackOutcome::FumbleLost(outcome) => write!(f, "Sacked & Fumble Lost, {}", outcome),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ReceivingYards {
    pub air_yards: i8,
    pub yards_after_catch: i8,
}

impl std::fmt::Display for ReceivingYards {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} yards ({} ay)", self.total(), self.air_yards)
    }
}

impl ReceivingYards {
    pub fn total(&self) -> i8 {
        self.air_yards + self.yards_after_catch
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OffensivePenaltyOutcome {
    pub yards: u8,
    pub loss_of_down: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DefensivePenaltyOutcome {
    pub yards: u8,
    pub automatic_first: bool,
    pub ignore_half_distance: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PenaltyType {
    // yards, loss of down
    Offensive(OffensivePenaltyOutcome),
    // yards, auto first
    Defensive(DefensivePenaltyOutcome),
}

#[derive(Debug, PartialEq)]
pub enum TargetOutcome {
    Incomplete(i8),              // air yards
    Yards(ReceivingYards, bool), // yards + true if clock pauses (e.g. goes OB)
    CatchThenFumble(ReceivingYards, TurnoverOutcome),
    Touchdown(ReceivingYards),
    Interception(i8, TurnoverOutcome),
}

impl std::fmt::Display for TargetOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetOutcome::Incomplete(_) => write!(f, "Incomplete pass"),
            TargetOutcome::Yards(yards, _) => {
                write!(f, "Completion for {}", yards)
            }
            TargetOutcome::CatchThenFumble(yards, outcome) => {
                write!(f, "Completion for {}. Fumble lost: {}", yards, outcome)
            }
            TargetOutcome::Touchdown(yards) => {
                write!(f, "Passing Touchdown for {}", yards)
            }
            TargetOutcome::Interception(air_yards, outcome) => {
                write!(f, "Interception: {} ({} air yards)", outcome, air_yards)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockStatus {
    Running,
    Paused,
    Stopped,
}

impl ClockStatus {
    pub fn merge(cs1: ClockStatus, cs2: ClockStatus, is_timeout: bool) -> ClockStatus {
        if is_timeout {
            return ClockStatus::Stopped;
        }
        match (cs1, cs2) {
            (ClockStatus::Stopped, _) => ClockStatus::Stopped,
            (_, ClockStatus::Stopped) => ClockStatus::Stopped,
            (ClockStatus::Paused, _) => ClockStatus::Paused,
            (_, ClockStatus::Paused) => ClockStatus::Paused,
            _ => ClockStatus::Running,
        }
    }

    pub fn transform_clock(&self, clock: &GameClock) -> ClockStatus {
        match self {
            ClockStatus::Stopped => ClockStatus::Stopped,
            ClockStatus::Running => ClockStatus::Running,
            ClockStatus::Paused => match clock.quarter {
                Quarter::Second => match clock.seconds_remaining {
                    0..=119 => ClockStatus::Stopped,
                    _ => ClockStatus::Paused,
                },
                Quarter::Fourth => match clock.seconds_remaining {
                    0..=299 => ClockStatus::Stopped,
                    _ => ClockStatus::Paused,
                },
                _ => ClockStatus::Paused,
            },
        }
    }
}

impl TargetOutcome {
    pub fn clock_status_after(&self) -> ClockStatus {
        match self {
            TargetOutcome::Yards(_, clock_paused) => match clock_paused {
                true => ClockStatus::Paused,
                false => ClockStatus::Running,
            },
            _ => ClockStatus::Stopped,
        }
    }

    pub fn can_have_post_play_penalty(&self) -> bool {
        match &self {
            TargetOutcome::Incomplete(_) => true,
            TargetOutcome::Yards(_, _) => true,
            TargetOutcome::CatchThenFumble(_, _) => false,
            TargetOutcome::Touchdown(_) => false,
            TargetOutcome::Interception(_, to) => to.can_have_post_play_penalty(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DropbackOutcome {
    Throwaway,
    QbScramble(RunResult),
    Sack(SackOutcome),
    Target(TargetResult),
    QbSpike,
}

impl std::fmt::Display for DropbackOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DropbackOutcome::Throwaway => write!(f, "Throwaway"),
            DropbackOutcome::QbScramble(result) => write!(f, "Scramble{}", result),
            DropbackOutcome::Sack(result) => write!(f, "{}", result),
            DropbackOutcome::Target(result) => write!(f, "{}", result),
            DropbackOutcome::QbSpike => write!(f, "QB Spike"),
        }
    }
}

impl DropbackOutcome {
    pub fn can_have_post_play_penalty(&self) -> bool {
        match self {
            DropbackOutcome::Throwaway => true,
            DropbackOutcome::QbScramble(run_result) => {
                run_result.outcome.can_have_post_play_penalty()
            }
            DropbackOutcome::Sack(so) => match so {
                SackOutcome::YardsLost(_) => true,
                SackOutcome::Safety => false,
                SackOutcome::FumbleLost(to) => to.can_have_post_play_penalty(),
            },
            DropbackOutcome::Target(tgt) => tgt.outcome.can_have_post_play_penalty(),
            DropbackOutcome::QbSpike => false,
        }
    }

    pub fn clock_status_after(&self) -> ClockStatus {
        match self {
            DropbackOutcome::Target(target_result) => match target_result.outcome {
                TargetOutcome::Yards(_, clock_paused) => match clock_paused {
                    true => ClockStatus::Paused,
                    false => ClockStatus::Running,
                },
                _ => ClockStatus::Stopped,
            },
            DropbackOutcome::QbScramble(result) => result.clock_status_after(),
            DropbackOutcome::Sack(_) => ClockStatus::Running,
            DropbackOutcome::Throwaway => ClockStatus::Stopped,
            DropbackOutcome::QbSpike => ClockStatus::Stopped,
        }
    }

    pub fn yards_gained(&self) -> f32 {
        match self {
            DropbackOutcome::Target(target_result) => match &target_result.outcome {
                TargetOutcome::Yards(yards, _) => yards.total() as f32,
                TargetOutcome::Touchdown(yards) => yards.total() as f32,
                _ => 0.0,
            },
            DropbackOutcome::QbScramble(result) => match result.outcome {
                RushingOutcome::Yards(yards, _) => yards as f32,
                // NOTE: this is wrong, but doesn't matter since only used for playclock
                _ => 0.0,
            },
            DropbackOutcome::Sack(result) => match result {
                SackOutcome::YardsLost(yards) => -(*yards as f32),
                _ => 0.0,
            },
            DropbackOutcome::Throwaway => 0.0,
            DropbackOutcome::QbSpike => 0.0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TargetResult {
    pub targeted_receiver_id: String,
    pub outcome: TargetOutcome,
}

impl std::fmt::Display for TargetResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.outcome)
    }
}

#[derive(Debug)]
pub struct DropbackResult {
    pub passer_id: String,
    pub outcome: DropbackOutcome,
    pub penalty: Option<PenaltyType>,
}

impl std::fmt::Display for DropbackResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.outcome)
    }
}

impl DropbackResult {
    pub fn clock_status_after(&self) -> ClockStatus {
        self.outcome.clock_status_after()
    }

    pub fn to_2pt_dropback_result(&self) -> PATResult {
        match &self.outcome {
            DropbackOutcome::QbScramble(run_result) => match run_result.outcome {
                RushingOutcome::Touchdown => PATResult::TwoPointDesignedRun(
                    PATRushingOutcome::Success(run_result.carrier_id.clone()),
                ),
                _ => PATResult::TwoPointDropback(PATDropbackOutcome::Fail),
            },
            DropbackOutcome::Target(target_result) => match &target_result.outcome {
                TargetOutcome::Touchdown(_) => {
                    PATResult::TwoPointDropback(PATDropbackOutcome::SuccessfulCompletion(
                        self.passer_id.clone(),
                        target_result.targeted_receiver_id.clone(),
                    ))
                }
                TargetOutcome::Interception(_, turnover_outcome) => match turnover_outcome {
                    TurnoverOutcome::Touchdown | TurnoverOutcome::DefensiveSafetyReturn => {
                        PATResult::TwoPointDropback(PATDropbackOutcome::DefensiveSafetyReturn)
                    }
                    _ => PATResult::TwoPointDropback(PATDropbackOutcome::Fail),
                },
                _ => PATResult::TwoPointDropback(PATDropbackOutcome::Fail),
            },
            _ => PATResult::TwoPointDropback(PATDropbackOutcome::Fail),
        }
    }
}

impl TargetResult {
    pub fn clock_status_after(&self) -> ClockStatus {
        self.outcome.clock_status_after()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RushingOutcome {
    Yards(i8, bool), // yards gained, true if clock is paused after (e.g. goes OB)
    Touchdown,
    // yards gained pre-loss, turnover outcome
    FumbleLost(i8, TurnoverOutcome),
    Safety,
}

impl std::fmt::Display for RushingOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RushingOutcome::Yards(yards, _) => write!(f, " for {} yards", yards),
            RushingOutcome::Touchdown => write!(f, " for Touchdown"),
            RushingOutcome::FumbleLost(yards, outcome) => {
                write!(f, " for {} yards. Fumble lost, {}", yards, outcome)
            }
            RushingOutcome::Safety => write!(f, ": Tackled for safety"),
        }
    }
}

impl RushingOutcome {
    pub fn yards_gained(&self, yards_for_touchdown: u8) -> f32 {
        match self {
            RushingOutcome::Yards(yards, _) => *yards as f32,
            RushingOutcome::FumbleLost(yards, _) => *yards as f32,
            RushingOutcome::Touchdown => yards_for_touchdown as f32,
            RushingOutcome::Safety => (100 - yards_for_touchdown) as f32,
        }
    }

    pub fn can_have_post_play_penalty(&self) -> bool {
        match self {
            RushingOutcome::Yards(_, _) => true,
            RushingOutcome::Touchdown => false,
            RushingOutcome::FumbleLost(_, to) => to.can_have_post_play_penalty(),
            RushingOutcome::Safety => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RunResult {
    pub carrier_id: String,
    pub outcome: RushingOutcome,
    pub penalty: Option<PenaltyType>,
}

impl std::fmt::Display for RunResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.outcome)
    }
}

impl RunResult {
    pub fn clock_status_after(&self) -> ClockStatus {
        match self.outcome {
            RushingOutcome::Yards(_, clock_paused) => match clock_paused {
                true => ClockStatus::Paused,
                false => ClockStatus::Running,
            },
            _ => ClockStatus::Stopped,
        }
    }

    pub fn to_2pt_run_outcome(&self) -> PATRushingOutcome {
        match &self.outcome {
            RushingOutcome::Touchdown => PATRushingOutcome::Success(self.carrier_id.clone()),
            RushingOutcome::FumbleLost(_, turnover_outcome) => match turnover_outcome {
                TurnoverOutcome::Touchdown | TurnoverOutcome::DefensiveSafetyReturn => {
                    PATRushingOutcome::DefensiveSafetyReturn
                }
                _ => PATRushingOutcome::Fail,
            },
            // this can't happen...
            RushingOutcome::Safety => PATRushingOutcome::DefensiveSafetyReturn,
            _ => PATRushingOutcome::Fail,
        }
    }
}

#[derive(Debug)]
pub enum KickingResult {
    ReturnedForYards(YardsToGoal), // to goal
    PuntTouchback,
    KickoffTouchback,
    ReturnedForTouchdown,
    FumbleLost(TurnoverOutcome),
    Blocked(TurnoverOutcome),
    OnsideRecovery(YardsToGoal),
}

impl std::fmt::Display for KickingResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            KickingResult::ReturnedForYards(yards_to_goal) => {
                write!(f, "Returner downed {} yards from goal", yards_to_goal)
            }
            KickingResult::PuntTouchback => write!(f, "Punt Touchback"),
            KickingResult::KickoffTouchback => write!(f, "Kickoff Touchback"),
            KickingResult::ReturnedForTouchdown => write!(f, "Returned for Touchdown"),
            KickingResult::FumbleLost(outcome) => write!(f, "Fumble Lost: {}", outcome),
            KickingResult::Blocked(outcome) => write!(f, "Blocked: {}", outcome),
            KickingResult::OnsideRecovery(yards_to_goal) => {
                write!(f, "Onside Kick Recovered at {}", yards_to_goal)
            }
        }
    }
}

#[derive(Debug)]
pub enum FieldGoalResult {
    AttemptedFg(bool),
    Blocked(TurnoverOutcome),
}

impl std::fmt::Display for FieldGoalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FieldGoalResult::AttemptedFg(success) => {
                if *success {
                    write!(f, "FG: Good")
                } else {
                    write!(f, "FG: Missed")
                }
            }
            FieldGoalResult::Blocked(outcome) => write!(f, "FG blocked: {}", outcome),
        }
    }
}

pub enum PlaycallResult {
    OffensivePenalty,
    DefensivePenalty,
    FieldGoalAttempt,
    Punt,
    QbSpike,
    QbKneel,
    QbDropback,
    DesignedRush,
}

impl std::fmt::Display for PlaycallResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PlaycallResult::OffensivePenalty => write!(f, "Offensive Penalty"),
            PlaycallResult::DefensivePenalty => write!(f, "Defensive Penalty"),
            PlaycallResult::FieldGoalAttempt => write!(f, "FG Attempt"),
            PlaycallResult::Punt => write!(f, "Punt"),
            PlaycallResult::QbSpike => write!(f, "QB Spike"),
            PlaycallResult::QbKneel => write!(f, "QB Kneel"),
            PlaycallResult::QbDropback => write!(f, "Dropback"),
            PlaycallResult::DesignedRush => write!(f, "Run"),
        }
    }
}

#[derive(Debug)]
pub enum PATRushingOutcome {
    // ball carrier ID
    Success(String),
    Fail,
    DefensiveSafetyReturn,
}

impl std::fmt::Display for PATRushingOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PATRushingOutcome::Success(_) => write!(f, "Success"),
            PATRushingOutcome::Fail => write!(f, "Failed"),
            PATRushingOutcome::DefensiveSafetyReturn => write!(f, "Defensive Safety Return"),
        }
    }
}

#[derive(Debug)]
pub enum PATDropbackOutcome {
    SuccessfulCompletion(String, String),
    SuccessfulScramble(String),
    Fail,
    DefensiveSafetyReturn,
}

impl std::fmt::Display for PATDropbackOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PATDropbackOutcome::SuccessfulCompletion(_, _) => {
                write!(f, "Successful Completion")
            }
            PATDropbackOutcome::SuccessfulScramble(_) => {
                write!(f, "Successful Scramble")
            }
            PATDropbackOutcome::Fail => write!(f, "Failed Pass"),
            PATDropbackOutcome::DefensiveSafetyReturn => write!(f, "Defensive Safety Return"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PATKickingOutcome {
    KickIsGood,
    KickMisses,
    DefensiveSafetyReturn,
    BlockedKickMisses,
}

impl std::fmt::Display for PATKickingOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PATKickingOutcome::KickIsGood => write!(f, "Good"),
            PATKickingOutcome::KickMisses => write!(f, "Missed"),
            PATKickingOutcome::DefensiveSafetyReturn => write!(f, "Defensive Safety Return"),
            PATKickingOutcome::BlockedKickMisses => write!(f, "Kick Blocked"),
        }
    }
}

#[derive(Debug)]
pub enum PATResult {
    KickAttempted(PATKickingOutcome),
    TwoPointDesignedRun(PATRushingOutcome),
    TwoPointDropback(PATDropbackOutcome),
}

impl std::fmt::Display for PATResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PATResult::KickAttempted(outcome) => write!(f, "PAT Kick: {}", outcome),
            PATResult::TwoPointDesignedRun(outcome) => {
                write!(f, "2-PT Run: {}", outcome)
            }
            PATResult::TwoPointDropback(outcome) => write!(f, "2-PT Dropback: {}", outcome),
        }
    }
}

#[derive(Debug)]
pub enum PlayResult {
    // yards, loss of down
    OffensivePenaltyNoPlay(u8, bool),
    // yards, automatic first, ignore half distance to goal
    DefensivePenaltyNoPlay(u8, bool, bool),
    Dropback(DropbackResult),
    DesignedRun(RunResult),
    FieldGoal(FieldGoalResult),
    Timeout(HomeAway),
    PointAfterTouchdown(PATResult),
    // kick returner ID + outcome
    Punt(Option<String>, KickingResult),
    Kickoff(Option<String>, KickingResult),
    // QB Id
    QbSpike(DropbackResult),
    QbKneel(RunResult),
}

impl std::fmt::Display for PlayResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayResult::OffensivePenaltyNoPlay(yards, loss_of_down) => {
                let loss_of_down_str = match loss_of_down {
                    true => " & loss of down",
                    false => "",
                };
                write!(f, "Offensive penalty: {} yards{}", yards, loss_of_down_str)
            }
            PlayResult::DefensivePenaltyNoPlay(yards, automatic_first, _) => {
                let auto_first_str = match automatic_first {
                    true => " & automatic first down",
                    false => "",
                };
                write!(f, "Defensive penalty: {} yards{}", yards, auto_first_str)
            }
            PlayResult::Dropback(dropback_result) => write!(f, "{}", dropback_result),
            PlayResult::DesignedRun(run_result) => write!(f, "Rush{}", run_result),
            PlayResult::FieldGoal(field_goal_result) => write!(f, "{}", field_goal_result),
            PlayResult::Punt(_, kicking_result) => {
                write!(f, "Punt: {}", kicking_result)
            }
            PlayResult::Kickoff(_, kicking_result) => {
                write!(f, "Kickoff: {}", kicking_result)
            }
            PlayResult::Timeout(possession) => write!(f, "Timeout by {}", possession),
            PlayResult::PointAfterTouchdown(pat_result) => write!(f, "{}", pat_result),
            PlayResult::QbSpike(_) => write!(f, "QB Spike"),
            PlayResult::QbKneel(_) => write!(f, "QB Kneel"),
        }
    }
}

impl PlayResult {
    pub fn clock_status_after(&self) -> ClockStatus {
        match self {
            PlayResult::DesignedRun(result) => result.clock_status_after(),
            PlayResult::Dropback(result) => result.clock_status_after(),
            PlayResult::QbKneel(_) => ClockStatus::Running,
            _ => ClockStatus::Stopped,
        }
    }

    pub fn is_kickoff(&self) -> f32 {
        match self {
            PlayResult::Kickoff(_, _) => 1.0,
            _ => 0.0,
        }
    }

    pub fn is_field_goal(&self) -> f32 {
        match self {
            PlayResult::FieldGoal(_) => 1.0,
            _ => 0.0,
        }
    }

    pub fn is_punt(&self) -> f32 {
        match self {
            PlayResult::Punt(_, _) => 1.0,
            _ => 0.0,
        }
    }

    pub fn is_run(&self) -> f32 {
        match self {
            PlayResult::DesignedRun(_) => 1.0,
            PlayResult::Dropback(result) => match result.outcome {
                DropbackOutcome::QbScramble(_) => 1.0,
                _ => 0.0,
            },
            _ => 0.0,
        }
    }

    pub fn is_pass(&self) -> f32 {
        match self {
            PlayResult::Dropback(result) => match result.outcome {
                DropbackOutcome::QbScramble(_) => 0.0,
                _ => 1.0,
            },
            _ => 0.0,
        }
    }

    pub fn is_no_play(&self) -> f32 {
        match self {
            PlayResult::OffensivePenaltyNoPlay(_, _) => 1.0,
            PlayResult::DefensivePenaltyNoPlay(_, _, _) => 1.0,
            PlayResult::Timeout(_) => 1.0,
            _ => 0.0,
        }
    }

    pub fn yards_gained(&self, yards_for_touchdown: u8) -> f32 {
        match self {
            PlayResult::Dropback(result) => result.outcome.yards_gained(),
            PlayResult::DesignedRun(result) => result.outcome.yards_gained(yards_for_touchdown),
            _ => 0.0,
        }
    }

    pub fn is_ot_ender(&self) -> bool {
        match self {
            PlayResult::Dropback(result) => match &result.outcome {
                DropbackOutcome::Target(target_result) => match target_result.outcome {
                    TargetOutcome::Touchdown(_) => true,
                    TargetOutcome::Interception(_, TurnoverOutcome::Touchdown) => true,
                    _ => false,
                },
                DropbackOutcome::Sack(SackOutcome::Safety) => true,
                DropbackOutcome::Sack(SackOutcome::FumbleLost(TurnoverOutcome::Touchdown)) => true,
                _ => false,
            },
            PlayResult::DesignedRun(result) => match result.outcome {
                RushingOutcome::Touchdown => true,
                RushingOutcome::FumbleLost(_, TurnoverOutcome::Touchdown) => true,
                _ => false,
            },
            PlayResult::Kickoff(_, KickingResult::ReturnedForTouchdown) => true,
            PlayResult::Kickoff(_, KickingResult::FumbleLost(TurnoverOutcome::Touchdown)) => true,
            PlayResult::Kickoff(_, KickingResult::Blocked(TurnoverOutcome::Touchdown)) => true,
            PlayResult::Punt(_, KickingResult::ReturnedForTouchdown) => true,
            PlayResult::Punt(_, KickingResult::FumbleLost(TurnoverOutcome::Touchdown)) => true,
            _ => false,
        }
    }
}
