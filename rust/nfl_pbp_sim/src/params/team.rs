use std::collections::HashMap;

use csv::Reader;
use serde::Deserialize;

use crate::params::skill_player::{PROB_1YTG_GIVEN_CARRY, PROB_GZ_GIVEN_CARRY, PROB_RZ_TARGET};

#[derive(Debug, Clone, Deserialize)]
pub struct TeamLoader {
    // general
    pub team: String,
    // offense
    pub pace_z: f32,
    pub offense_proe: f32,
    pub offense_rz_proe: f32,
    pub oline_rushing_z: f32,
    pub offense_pass_rush_z: f32,

    // for defending against the pass
    pub defense_proe: f32,
    pub defense_rz_proe: f32,
    pub dline_rushing_z: f32,
    pub defense_pass_rush_z: f32,

    pub defense_completion_z: f32,
    pub defense_interception_z: f32,
    pub defense_yac_oe: f32,

    // special teams
    // place kicker
    pub short_fg_z: f32,
    pub long_fg_z: f32,

    // misc
    pub offense_penalty_z: f32,
    pub defense_penalty_z: f32,

    pub kickoff_returner_id: Option<String>,
    pub punt_returner_id: Option<String>,
}

impl TeamLoader {
    pub fn to_team(&self) -> Team {
        Team {
            team: self.team.clone(),
            pace_z: self.pace_z,
            offense_proe: self.offense_proe,
            offense_rz_proe: self.offense_rz_proe,
            oline_rushing_z: self.oline_rushing_z,
            offense_pass_rush_z: self.offense_pass_rush_z,
            defense_proe: self.defense_proe,
            defense_rz_proe: self.defense_rz_proe,
            dline_rushing_z: self.dline_rushing_z,
            defense_pass_rush_z: self.defense_pass_rush_z,
            defense_completion_z: self.defense_completion_z,
            defense_interception_z: self.defense_interception_z,
            defense_yac_oe: self.defense_yac_oe,
            short_fg_z: self.short_fg_z,
            long_fg_z: self.long_fg_z,
            offense_penalty_z: self.offense_penalty_z,
            defense_penalty_z: self.defense_penalty_z,
            kickoff_returner_id: self.kickoff_returner_id.clone(),
            punt_returner_id: self.punt_returner_id.clone(),
            prob_1ytg_given_carry: PROB_1YTG_GIVEN_CARRY,
            prob_gz_given_carry: PROB_GZ_GIVEN_CARRY,
            prob_rz_given_target: PROB_RZ_TARGET,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Team {
    // general
    pub team: String,
    // offense
    pub pace_z: f32,
    pub offense_proe: f32,
    pub offense_rz_proe: f32,
    pub oline_rushing_z: f32,
    pub offense_pass_rush_z: f32,

    // for defending against the pass
    pub defense_proe: f32,
    pub defense_rz_proe: f32,
    pub dline_rushing_z: f32,
    pub defense_pass_rush_z: f32,

    pub defense_completion_z: f32,
    pub defense_interception_z: f32,
    pub defense_yac_oe: f32,

    // special teams
    // place kicker
    pub short_fg_z: f32,
    pub long_fg_z: f32,

    // misc
    pub offense_penalty_z: f32,
    pub defense_penalty_z: f32,

    pub kickoff_returner_id: Option<String>,
    pub punt_returner_id: Option<String>,

    // filled in by burning in with N sims
    pub prob_1ytg_given_carry: f32,
    pub prob_gz_given_carry: f32,
    pub prob_rz_given_target: f32,
}

impl Team {
    pub fn load(path: &String) -> HashMap<String, Team> {
        let mut game_reader = Reader::from_path(format!("{}/Teams-Table 1.csv", path)).unwrap();

        let mut teams = HashMap::new();
        for team_csv in game_reader.deserialize() {
            let team_loader: TeamLoader = team_csv.expect("failed to parse team");
            teams.insert(team_loader.team.clone(), team_loader.to_team());
        }

        teams
    }
}
