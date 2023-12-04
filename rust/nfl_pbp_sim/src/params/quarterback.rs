use std::collections::HashMap;

// use crate::params::Injury;
use crate::params::RushingParams;
use csv::Reader;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Quarterback {
    pub player_id: String,
    pub team: String,
    pub name: String,
    // how good of a passer are they?
    // NOTE: ue = under expectation, oe = over expectation
    pub cpoe: f32,   // completion percentage
    pub int_ue: f32, // interceptions
    // air yards
    pub ayoe: f32,
    pub ay_std: f32,

    pub scramble_rate: f32,
    // scrambling parameters (not designed rushes)
    pub syoe: f32,
    pub syoe_std: f32,
    // how likely are they to take sacks?
    pub prob_sack_given_hit: f32,
    // TODO: add depth chart/injuries
    // 1 for starter, 2 for first backup, etc.
    // pub depth_chart: u8,
    // pub injury: Injury,
    // given this QB is in, what's the passing rate above expectation?
    // pub proe: f32,
    // for backups only: if this QB fills in for the starter,
    // how much would the team's implied total change?
    // pub implied_total_adjustment: Option<f32>,
}

impl Quarterback {
    pub fn rushing_params(&self) -> RushingParams {
        RushingParams {
            yoe_mean: self.syoe,
            yoe_std: self.syoe_std,
        }
    }

    pub fn load(path: &String) -> HashMap<String, Vec<Quarterback>> {
        let mut qb_reader = Reader::from_path(format!("{}/QB-Table 1.csv", path)).unwrap();

        let mut qbs: HashMap<String, Vec<Quarterback>> = HashMap::new();
        for qb_csv in qb_reader.deserialize() {
            let qb: Quarterback = qb_csv.expect("failed to parse qb");
            match qbs.get_mut(&qb.team) {
                Some(qb_vec) => qb_vec.push(qb),
                None => {
                    qbs.insert(qb.team.clone(), vec![qb]);
                }
            };
        }
        qbs
    }
}
