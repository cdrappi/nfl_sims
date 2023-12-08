use std::collections::HashMap;

use crate::{
    params::GameParams,
    sim::play_result::{DropbackOutcome, ReceivingYards, SackOutcome, TargetOutcome},
};

#[derive(Debug, Clone)]
pub struct PassingBoxScore {
    pub attempts: u16,
    pub completions: u16,
    pub yards: i16,
    pub air_yards: i16,
    pub yards_after_catch: i16,
    pub touchdowns: u8,
    pub interceptions: u8,
    pub two_point_conversions: u8,
    pub fumbles_lost: u8,
    pub scrambles: u8,
    pub kneels: u8,
}

impl PassingBoxScore {
    pub fn new() -> PassingBoxScore {
        PassingBoxScore {
            attempts: 0,
            completions: 0,
            yards: 0,
            air_yards: 0,
            yards_after_catch: 0,
            touchdowns: 0,
            interceptions: 0,
            two_point_conversions: 0,
            fumbles_lost: 0,
            scrambles: 0,
            kneels: 0,
        }
    }

    pub fn new_map(game_params: &GameParams) -> HashMap<String, PassingBoxScore> {
        let mut map = HashMap::new();
        for player in &game_params.home.qbs {
            map.insert(player.player_id.clone(), PassingBoxScore::new());
        }
        for player in &game_params.away.qbs {
            map.insert(player.player_id.clone(), PassingBoxScore::new());
        }
        map
    }

    pub fn add_completion(&mut self, yards: &ReceivingYards) {
        self.attempts += 1;
        self.completions += 1;
        self.yards += yards.total() as i16;
        self.air_yards += yards.air_yards as i16;
        self.yards_after_catch += yards.yards_after_catch as i16;
    }

    pub fn add_scramble(&mut self) {
        self.scrambles += 1;
    }

    pub fn add_kneel(&mut self) {
        self.kneels += 1;
    }

    pub fn update(&mut self, passing_outcome: &DropbackOutcome) {
        // log::info!("updating passing box score: {:?}", passing_outcome);
        match passing_outcome {
            DropbackOutcome::Throwaway | DropbackOutcome::QbSpike => {
                self.attempts += 1;
            }
            DropbackOutcome::Sack(outcome) => match outcome {
                SackOutcome::FumbleLost(_) => {
                    self.fumbles_lost += 1;
                }
                _ => {}
            },
            DropbackOutcome::QbScramble(_) => {}
            DropbackOutcome::Target(result) => match &result.outcome {
                TargetOutcome::Incomplete(air_yards) => {
                    self.attempts += 1;
                    self.air_yards += *air_yards as i16;
                }
                TargetOutcome::Yards(yards, _) => self.add_completion(&yards),
                TargetOutcome::CatchThenFumble(yards, _) => self.add_completion(&yards),
                TargetOutcome::Touchdown(yards) => {
                    self.add_completion(&yards);
                    self.touchdowns += 1;
                }
                TargetOutcome::Interception(air_yards, _) => {
                    self.attempts += 1;
                    self.air_yards += *air_yards as i16;
                    self.interceptions += 1;
                }
            },
        }
    }

    pub fn add_two_point_conversion(&mut self) {
        self.two_point_conversions += 1;
    }

    pub fn passing_points(&self) -> f32 {
        let mut points = 0.0;
        points += 0.04 * (self.yards as f32);
        points += 4.0 * (self.touchdowns as f32);
        points += -1.0 * (self.interceptions as f32);
        points += 2.0 * (self.two_point_conversions as f32);
        points += -2.0 * (self.fumbles_lost as f32);
        points
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PassingProjection {
    pub attempts: f32,
    pub completions: f32,
    pub yards: f32,
    pub air_yards: f32,
    pub yards_after_catch: f32,
    pub touchdowns: f32,
    pub interceptions: f32,
    pub two_point_conversions: f32,
    pub scrambles: f32,
    pub kneels: f32,
    pub yards_sampled: Vec<i16>,
    pub tds_sampled: Vec<u8>,
}

impl PassingProjection {
    pub fn new() -> PassingProjection {
        PassingProjection {
            attempts: 0.0,
            completions: 0.0,
            yards: 0.0,
            air_yards: 0.0,
            yards_after_catch: 0.0,
            touchdowns: 0.0,
            interceptions: 0.0,
            two_point_conversions: 0.0,
            scrambles: 0.0,
            kneels: 0.0,
            yards_sampled: vec![],
            tds_sampled: vec![],
        }
    }
}
