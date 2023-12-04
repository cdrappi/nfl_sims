use std::collections::HashMap;

use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
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
}

impl Team {
    pub fn load(path: &String) -> HashMap<String, Team> {
        let mut game_reader = Reader::from_path(format!("{}/Teams-Table 1.csv", path)).unwrap();

        let mut teams = HashMap::new();
        for team_csv in game_reader.deserialize() {
            let team: Team = team_csv.expect("failed to parse team");
            teams.insert(team.team.clone(), team);
        }

        teams
    }
}
