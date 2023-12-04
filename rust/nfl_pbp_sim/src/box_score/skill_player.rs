use std::collections::HashMap;

use crate::{
    params::GameParams,
    sim::play_result::{ReceivingYards, RushingOutcome, TargetOutcome},
    state::down::PlayState,
};

#[derive(Debug, Clone)]
pub struct SkillPlayerBoxScore {
    // rushing
    pub carries: u8,
    pub rushing_yards: i16,
    pub rushing_touchdowns: u8,
    // receiving
    pub targets: u8,
    pub catches: u8,
    pub receiving_yards: i16,
    pub air_yards: i16,
    pub yards_after_catch: i16,
    pub receiving_touchdowns: u8,
    // misc
    pub fumbles_lost: u8,
    pub return_touchdowns: u8,
    pub two_point_conversions: u8,
}

impl SkillPlayerBoxScore {
    pub fn new() -> SkillPlayerBoxScore {
        SkillPlayerBoxScore {
            carries: 0,
            rushing_yards: 0,
            rushing_touchdowns: 0,
            targets: 0,
            catches: 0,
            receiving_yards: 0,
            air_yards: 0,
            yards_after_catch: 0,
            receiving_touchdowns: 0,
            fumbles_lost: 0,
            return_touchdowns: 0,
            two_point_conversions: 0,
        }
    }

    pub fn new_map(game_params: &GameParams) -> HashMap<String, SkillPlayerBoxScore> {
        let mut map = HashMap::new();
        for player in &game_params.home.qbs {
            map.insert(player.player_id.clone(), SkillPlayerBoxScore::new());
        }
        for player in &game_params.away.qbs {
            map.insert(player.player_id.clone(), SkillPlayerBoxScore::new());
        }
        for player in game_params.home.skill_players.values() {
            map.insert(player.player_id.clone(), SkillPlayerBoxScore::new());
        }
        for player in game_params.away.skill_players.values() {
            map.insert(player.player_id.clone(), SkillPlayerBoxScore::new());
        }
        map
    }

    pub fn add_carry(&mut self, yards_gained: i8) {
        self.carries += 1;
        self.rushing_yards += yards_gained as i16;
    }

    pub fn add_incomplete_target(&mut self, air_yards: i8) {
        self.targets += 1;
        self.air_yards += air_yards as i16;
    }

    pub fn add_catch(&mut self, yards: &ReceivingYards) {
        self.targets += 1;
        self.catches += 1;
        self.receiving_yards += yards.total() as i16;
        self.air_yards += yards.air_yards as i16;
        self.yards_after_catch += yards.yards_after_catch as i16;
    }

    pub fn add_fumble_lost(&mut self) {
        self.fumbles_lost += 1;
    }

    pub fn add_rushing_touchdown(&mut self) {
        self.rushing_touchdowns += 1;
    }

    pub fn add_receiving_touchdown(&mut self) {
        self.receiving_touchdowns += 1;
    }

    pub fn add_return_touchdown(&mut self) {
        self.return_touchdowns += 1;
    }

    pub fn update_rushing(&mut self, outcome: &RushingOutcome, prev_state: &PlayState) {
        match outcome {
            RushingOutcome::FumbleLost(yards_gained, _) => {
                self.add_carry(*yards_gained);
                self.add_fumble_lost();
            }
            RushingOutcome::Yards(yards_gained, _) => {
                self.add_carry(*yards_gained);
            }
            RushingOutcome::Touchdown => {
                self.add_carry(prev_state.expect_downtogo().yards_to_goal.0 as i8);
                self.add_rushing_touchdown();
            }
            RushingOutcome::Safety => {
                self.add_carry(prev_state.yards_for_safety());
            }
        }
    }

    pub fn update_receiving(&mut self, outcome: &TargetOutcome) {
        match outcome {
            TargetOutcome::Incomplete(air_yards) => {
                self.add_incomplete_target(*air_yards);
            }
            TargetOutcome::Yards(yards, _) => {
                self.add_catch(yards);
            }
            TargetOutcome::CatchThenFumble(yards, _) => {
                self.add_catch(yards);
                self.add_fumble_lost();
            }
            TargetOutcome::Touchdown(yards) => {
                self.add_catch(yards);
                self.add_receiving_touchdown();
            }
            TargetOutcome::Interception(air_yards, _) => {
                self.add_incomplete_target(*air_yards);
            }
        }
    }

    pub fn add_two_point_conversion(&mut self) {
        self.two_point_conversions += 1;
    }

    pub fn standard_fantasy_points(&self) -> f32 {
        let rushing_points =
            0.1 * (self.rushing_yards as f32) + 6.0 * (self.rushing_touchdowns as f32);
        let receiving_points =
            0.1 * (self.receiving_yards as f32) + 6.0 * (self.receiving_touchdowns as f32);
        rushing_points
            + receiving_points
            + 6.0 * (self.return_touchdowns as f32)
            + 2.0 * self.two_point_conversions as f32
            - 2.0 * self.fumbles_lost as f32
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SkillPlayerProjection {
    // rushing
    pub carries: f32,
    pub rushing_yards: f32,
    pub rushing_touchdowns: f32,
    // receiving
    pub targets: f32,
    pub catches: f32,
    pub receiving_yards: f32,
    pub air_yards: f32,
    pub yards_after_catch: f32,
    pub receiving_touchdowns: f32,
    // misc
    pub fumbles_lost: f32,
    pub return_touchdowns: f32,
    pub two_point_conversions: f32,

    // for medians
    pub rushing_yards_sampled: Vec<i16>,
    pub receiving_yards_sampled: Vec<i16>,
    pub total_yards_sampled: Vec<i16>,
    pub rushing_tds_sampled: Vec<u8>,
    pub receiving_tds_sampled: Vec<u8>,
    pub total_tds_sampled: Vec<u8>,
}

impl SkillPlayerProjection {
    pub fn new() -> SkillPlayerProjection {
        SkillPlayerProjection {
            carries: 0.0,
            rushing_yards: 0.0,
            rushing_touchdowns: 0.0,
            targets: 0.0,
            catches: 0.0,
            receiving_yards: 0.0,
            air_yards: 0.0,
            yards_after_catch: 0.0,
            receiving_touchdowns: 0.0,
            fumbles_lost: 0.0,
            return_touchdowns: 0.0,
            two_point_conversions: 0.0,
            rushing_yards_sampled: vec![],
            receiving_yards_sampled: vec![],
            total_yards_sampled: vec![],
            rushing_tds_sampled: vec![],
            receiving_tds_sampled: vec![],
            total_tds_sampled: vec![],
        }
    }

    pub fn add(&mut self, stats: &SkillPlayerBoxScore, n_sims: f32) {
        self.carries += stats.carries as f32 / n_sims;
        self.rushing_yards += stats.rushing_yards as f32 / n_sims;
        self.rushing_touchdowns += stats.rushing_touchdowns as f32 / n_sims;
        self.targets += stats.targets as f32 / n_sims;
        self.catches += stats.catches as f32 / n_sims;
        self.receiving_yards += stats.receiving_yards as f32 / n_sims;
        self.receiving_touchdowns += stats.receiving_touchdowns as f32 / n_sims;
        self.air_yards += stats.air_yards as f32 / n_sims;
        self.yards_after_catch += stats.yards_after_catch as f32 / n_sims;
        self.fumbles_lost += stats.fumbles_lost as f32 / n_sims;
        self.return_touchdowns += stats.return_touchdowns as f32 / n_sims;
        self.two_point_conversions += stats.two_point_conversions as f32 / n_sims;
        self.rushing_yards_sampled.push(stats.rushing_yards);
        self.receiving_yards_sampled.push(stats.receiving_yards);
        self.total_yards_sampled
            .push(stats.rushing_yards + stats.receiving_yards);
        self.rushing_tds_sampled.push(stats.rushing_touchdowns);
        self.receiving_tds_sampled.push(stats.receiving_touchdowns);
        self.total_tds_sampled
            .push(stats.rushing_touchdowns + stats.receiving_touchdowns);
    }
}
