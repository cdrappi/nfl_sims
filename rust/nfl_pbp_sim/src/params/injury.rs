use std::collections::HashMap;

use crate::params::skill_player::Position;
use crate::params::TeamParams;
use crate::sim::play_result::{DropbackOutcome, PlayResult, RushingOutcome, TargetOutcome};
use crate::util::stats::random_bool;

// Carries
pub const SKILL_RUSH_YARDS_INJURY_PROB: f32 = 0.00_25;
// (they'll slide)
pub const QB_CARRY_INJURY_PROB: f32 = 0.00_25;
// really shouldn't be possible...
pub const SKILL_RUSH_TD_INJURY_PROB: f32 = 0.00_01;
// should be more likely than normal yards
pub const SKILL_RUSH_SAFETY_INJURY_PROB: f32 = 2.0 * SKILL_RUSH_YARDS_INJURY_PROB;
// should be MUCH more likely than normal yards
pub const SKILL_RUSH_FUMLOST_INJURY_PROB: f32 = 10.0 * SKILL_RUSH_YARDS_INJURY_PROB;

// dropbacks for QBs
pub const QB_SCRAMBLE_INJURY_PROB: f32 = 0.00_50;
pub const QB_SACKED_INJURY_PROB: f32 = 0.01;
pub const QB_THROW_INJURY_PROB: f32 = 0.00_01;

// targets for skill
pub const SKILL_CATCH_YARDS_INJURY_PROB: f32 = 0.00_25;
pub const SKILL_CATCH_TD_INJURY_PROB: f32 = 0.00_01;
pub const SKILL_INCOMPLETE_INJURY_PROB: f32 = 0.00_10;
pub const SKILL_INT_INJURY_PROB: f32 = 0.00_05;
pub const SKILL_CATCH_FUMBLE_INJURY_PROB: f32 = 0.01;

#[derive(Clone, Hash, PartialEq, serde::Serialize, Debug)]
pub enum Injury {
    Healthy,
    // if injured, when will they return?
    // default assume out for game
    Injured,
}

impl Injury {
    /// returns hashmap of players who have a different injury status than before the play
    /// for now, just means a list of guys who got injured    
    pub fn sim_injuries(
        play_result: &PlayResult,
        team_params: &TeamParams,
    ) -> HashMap<Position, HashMap<String, Injury>> {
        let mut injuries = HashMap::new();
        let player_probs = match play_result {
            PlayResult::Dropback(dropback) => {
                let qb_param = &team_params.skill_players[&dropback.passer_id];
                let qb_injury_mult = match qb_param.depth_chart == 1 {
                    true => qb_param.injury_mult,
                    // assume backup qbs cannot get injured
                    false => 0.0,
                };
                let qb_key = (Position::Quarterback, &dropback.passer_id);
                match &dropback.outcome {
                    DropbackOutcome::QbScramble(_) => {
                        vec![(qb_key, QB_SCRAMBLE_INJURY_PROB * qb_injury_mult)]
                    }
                    DropbackOutcome::Sack(_) => {
                        vec![(qb_key, QB_SACKED_INJURY_PROB * qb_injury_mult)]
                    }
                    DropbackOutcome::Target(tgt) => {
                        let tgted_param = &team_params.skill_players[&tgt.targeted_receiver_id];
                        let tgt_injury_mult = tgted_param.injury_mult;
                        let tgt_key = (tgted_param.position, &tgt.targeted_receiver_id);
                        let outcome_prob = match tgt.outcome {
                            TargetOutcome::Yards(_, _) => SKILL_CATCH_YARDS_INJURY_PROB,
                            TargetOutcome::Touchdown(_) => SKILL_CATCH_TD_INJURY_PROB,
                            TargetOutcome::Incomplete(_) => SKILL_INCOMPLETE_INJURY_PROB,
                            TargetOutcome::Interception(_, _) => SKILL_INT_INJURY_PROB,
                            TargetOutcome::CatchThenFumble(_, _) => SKILL_CATCH_FUMBLE_INJURY_PROB,
                        };
                        vec![
                            (qb_key, qb_injury_mult * QB_THROW_INJURY_PROB),
                            (tgt_key, tgt_injury_mult * outcome_prob),
                        ]
                    }
                    DropbackOutcome::Throwaway => {
                        vec![(qb_key, qb_injury_mult * QB_THROW_INJURY_PROB)]
                    }
                    // no injury possible
                    DropbackOutcome::QbSpike => vec![],
                }
            }
            PlayResult::DesignedRun(run) => {
                let rusher_param = &team_params.skill_players[&run.carrier_id];
                let rusher_key = (rusher_param.position, &run.carrier_id);
                let rusher_injury_mult = match rusher_param.position {
                    Position::Quarterback => match rusher_param.depth_chart {
                        1 => rusher_param.injury_mult,
                        // backup QBs cannot get injured
                        _ => 0.0,
                    },
                    _ => rusher_param.injury_mult,
                };
                let outcome_prob = match run.outcome {
                    RushingOutcome::Yards(_, _) => SKILL_RUSH_YARDS_INJURY_PROB,
                    RushingOutcome::Touchdown => SKILL_RUSH_TD_INJURY_PROB,
                    RushingOutcome::FumbleLost(_, _) => SKILL_RUSH_FUMLOST_INJURY_PROB,
                    RushingOutcome::Safety => SKILL_RUSH_SAFETY_INJURY_PROB,
                };
                vec![(rusher_key, rusher_injury_mult * outcome_prob)]
            }
            _ => vec![],
        };
        for ((pos, player_id), injury_prob) in player_probs {
            if random_bool(injury_prob) {
                if !injuries.contains_key(&pos) {
                    injuries.insert(pos, HashMap::new());
                }
                let pos_injuries = injuries.get_mut(&pos).unwrap();
                pos_injuries.insert(player_id.clone(), Injury::Injured);
            }
        }
        injuries
    }
}
