use std::collections::HashMap;

use crate::models::targets::PROB_RZ_TARGET;
use crate::params::RushingParams;
use crate::util::stats::sample_beta;

use csv::Reader;
use serde::Deserialize;

use super::Injury;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Position {
    #[serde(rename = "QB")]
    Quarterback,
    #[serde(rename = "RB")]
    Halfback,
    #[serde(rename = "FB")]
    Fullback,
    #[serde(rename = "WR")]
    WideReceiver,
    #[serde(rename = "TE")]
    TightEnd,
    #[serde(rename = "K")]
    Kicker,
    #[serde(rename = "DEF")]
    Defense,
    #[serde(rename = "RET")]
    Returner,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn order(pos: &Position) -> u8 {
            match pos {
                Position::Quarterback => 0,
                Position::Halfback => 1,
                Position::Fullback => 2,
                Position::WideReceiver => 3,
                Position::TightEnd => 4,
                Position::Returner => 5,
                Position::Kicker => 6,
                Position::Defense => 7,
            }
        }

        order(self).cmp(&order(other))
    }
}

impl Position {
    pub fn to_string(&self) -> String {
        match self {
            Position::Quarterback => String::from("QB"),
            Position::Halfback => String::from("HB"),
            Position::Fullback => String::from("FB"),
            Position::WideReceiver => String::from("WR"),
            Position::TightEnd => String::from("TE"),
            Position::Kicker => String::from("K"),
            Position::Defense => String::from("DEF"),
            Position::Returner => String::from("RET"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum MarketShare {
    Constant(f32),
    Beta(f32, f32),
}

impl MarketShare {
    pub fn new(ms: f32, std_opt: Option<f32>) -> MarketShare {
        match std_opt {
            None => MarketShare::Constant(ms),
            Some(std_val) => match std_val == 0.0 {
                true => MarketShare::Constant(ms),
                false => {
                    assert!(
                        std_val.powi(2) <= ms * (1.0 - ms),
                        "Invalid beta parameters: {}, {}",
                        ms,
                        std_val
                    );
                    let alpha = ms.powi(2) * ((1.0 - ms) / std_val.powi(2) - 1.0 / ms);
                    let beta = alpha * (1.0 / ms - 1.0);
                    MarketShare::Beta(alpha, beta)
                }
            },
        }
    }

    pub fn collapse(&self) -> f32 {
        match self {
            MarketShare::Constant(ms) => *ms,
            MarketShare::Beta(shape_a, shape_b) => sample_beta(*shape_a, *shape_b),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SkillPlayer {
    pub player_id: String,
    pub team: String,
    pub name: String,
    pub position: Position,
    pub injury: Injury,
    pub depth_chart: u8,
    // init = before injuries
    // live = updated after injuries
    pub ms_carries_init: f32,
    pub ms_carries_live: f32,

    pub ms_targets_init: f32,
    pub ms_targets_live: f32,
    pub prob_1ytg_given_carry: f32,
    // green zone = 10 yards or less from end zone
    pub prob_gz_given_carry: f32,

    // designed run ability (YPC proxy)
    pub ryoe: f32,
    pub ryoe_std: f32,

    // red zone = 20 yards or less from end zone
    pub prob_rz_given_target: f32,

    // influences P(catch|DoT)
    pub adot: f32,
    pub adot_std: f32,
    pub prob_catch_oe: f32,
    pub xyac: f32,
    pub yac_oe: f32,
    // misc ball carrying
    // TODO: fumble rate
    // pub fumble_rate: f32,
    pub injury_mult: f32,
}

#[derive(Clone, Debug)]
pub struct SkillPlayerDistribution {
    pub player_id: String,
    pub team: String,
    pub name: String,
    pub position: Position,
    // pub injury: Injury,
    pub depth_chart: u8,
    pub ms_carries: MarketShare,
    pub ms_targets: MarketShare,

    pub prob_1ytg_given_carry: f32,
    // green zone = 10 yards or less from end zone
    pub prob_gz_given_carry: f32,

    // designed run ability (YPC proxy)
    pub ryoe: f32,
    pub ryoe_std: f32,

    // red zone = 20 yards or less from end zone
    pub prob_rz_given_target: f32,

    // influences P(catch|DoT)
    pub adot: f32,
    pub adot_std: f32,
    pub prob_catch_oe: f32,
    pub xyac: f32,
    pub yac_oe: f32,
    // misc ball carrying
    // TODO: fumble rate
    // pub fumble_rate: f32,
    pub injury_mult: f32,
}

impl SkillPlayerDistribution {
    pub fn to_skill_player(&self) -> SkillPlayer {
        let ms_carries_init = self.ms_carries.collapse();
        let ms_targets_init = self.ms_targets.collapse();
        SkillPlayer {
            player_id: self.player_id.clone(),
            team: self.team.clone(),
            name: self.name.clone(),
            position: self.position,
            depth_chart: self.depth_chart,
            injury: Injury::Healthy,
            ms_carries_init,
            ms_carries_live: ms_carries_init,
            ms_targets_init,
            ms_targets_live: ms_targets_init,
            prob_1ytg_given_carry: self.prob_1ytg_given_carry,
            prob_gz_given_carry: self.prob_gz_given_carry,
            ryoe: self.ryoe,
            ryoe_std: self.ryoe_std,
            prob_rz_given_target: self.prob_rz_given_target,
            adot: self.adot,
            adot_std: self.adot_std,
            prob_catch_oe: self.prob_catch_oe,
            xyac: self.xyac,
            yac_oe: self.yac_oe,
            injury_mult: self.injury_mult,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct SkillPlayerLoader {
    pub player_id: String,
    pub team: String,
    pub name: String,
    pub pos: Position,
    // pub injury: Injury,
    pub depth_chart: u8,
    pub ms_carries: Option<f32>,
    pub msc_std: Option<f32>,
    pub ms_targets: Option<f32>,
    pub mst_std: Option<f32>,

    pub prob_1ytg_given_carry: Option<f32>,
    // green zone = 10 yards or less from end zone
    pub prob_gz_given_carry: Option<f32>,

    // designed run ability (YPC proxy)
    pub ryoe: Option<f32>,
    pub ryoe_std: Option<f32>,

    // red zone = 20 yards or less from end zone
    pub prob_rz_given_target: Option<f32>,

    // influences P(catch|DoT)
    pub adot: Option<f32>,
    pub adot_std: Option<f32>,
    pub prob_catch_oe: Option<f32>,
    pub xyac: Option<f32>,
    pub yac_oe: Option<f32>,
    // misc ball carrying
    // TODO: fumble rate
    // pub fumble_rate: f32,
    pub injury_mult: Option<f32>,
}

impl SkillPlayerLoader {
    pub fn fill_na(&self) -> SkillPlayerDistribution {
        SkillPlayerDistribution {
            player_id: self.player_id.clone(),
            team: self.team.clone(),
            name: self.name.clone(),
            position: self.pos.clone(),
            depth_chart: self.depth_chart,
            ms_carries: MarketShare::new(self.ms_carries.unwrap_or(0.0), self.msc_std),
            ms_targets: MarketShare::new(self.ms_targets.unwrap_or(0.0), self.mst_std),
            prob_1ytg_given_carry: self.prob_1ytg_given_carry.unwrap_or(0.102),
            prob_gz_given_carry: self.prob_gz_given_carry.unwrap_or(0.063),
            ryoe: self.ryoe.unwrap_or(0.0),
            ryoe_std: self.ryoe_std.unwrap_or(3.0),
            prob_rz_given_target: self.prob_rz_given_target.unwrap_or(PROB_RZ_TARGET),
            adot: self.adot.unwrap_or(5.0),
            adot_std: self.adot_std.unwrap_or(5.0),
            prob_catch_oe: self.prob_catch_oe.unwrap_or(0.0),
            xyac: self.xyac.unwrap_or(5.5),
            yac_oe: self.yac_oe.unwrap_or(0.0),
            injury_mult: self.injury_mult.unwrap_or(1.0),
        }
    }
}

impl SkillPlayer {
    pub fn rushing_params(&self) -> RushingParams {
        RushingParams {
            yoe_mean: self.ryoe,
            yoe_std: self.ryoe_std,
        }
    }

    pub fn depth_charts(skill_players: &Vec<&SkillPlayer>) -> Vec<u8> {
        let mut depth_charts: Vec<u8> = skill_players.iter().map(|sp| sp.depth_chart).collect();
        depth_charts.sort();
        depth_charts
    }

    pub fn load(path: &String) -> HashMap<String, HashMap<String, SkillPlayerDistribution>> {
        let mut sp_reader = Reader::from_path(format!("{}/Skill-Table 1.csv", path)).unwrap();

        let mut sp: HashMap<String, HashMap<String, SkillPlayerDistribution>> = HashMap::new();
        for sp_csv in sp_reader.deserialize() {
            let skill_player: SkillPlayerLoader = sp_csv.expect("failed to parse skill player");
            match sp.get_mut(&skill_player.team) {
                Some(sp_map) => {
                    sp_map.insert(skill_player.player_id.clone(), skill_player.fill_na());
                }
                None => {
                    let mut sp_map = HashMap::new();
                    sp_map.insert(skill_player.player_id.clone(), skill_player.fill_na());
                    sp.insert(skill_player.team.clone(), sp_map);
                }
            };
        }
        sp
    }
}
