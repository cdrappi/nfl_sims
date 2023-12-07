pub mod quarterback;
pub mod skill_player;
pub mod team;
pub mod weather;

use csv::Reader;
use serde::Deserialize;
use std::collections::HashMap;

use crate::params::skill_player::{Position, SkillPlayerDistribution};
use crate::params::weather::StadiumType;
use crate::params::{
    quarterback::Quarterback, skill_player::SkillPlayer, team::Team, weather::Weather,
};
use crate::sim::box_score::SalaryKey;
use crate::sim::play_result::{DropbackOutcome, PlayResult, RushingOutcome, TargetOutcome};
use crate::start::HomeAway;
use crate::util::stats::random_bool;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PlayerMeta {
    pub name: String,
    pub pos: Position,
    pub team: String,
    pub opp: String,
}

#[derive(Clone, Hash, PartialEq, serde::Serialize, Debug)]
pub enum Injury {
    Healthy,
    // if injured, when will they return?
    // default assume out for game
    Injured,
}

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
                let qb_depth = match &dropback.passer_id == &team_params.qbs[0].player_id {
                    true => 1,
                    false => 2,
                };
                let qb_param = &team_params.skill_players[&dropback.passer_id];
                let qb_injury_mult = match qb_depth == 1 {
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
                let rusher_injury_mult = rusher_param.injury_mult;
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

#[derive(Clone, Debug)]
pub struct TeamParamsDistribution {
    pub team: Team,
    pub qbs: Vec<Quarterback>,
    pub skill_players: HashMap<String, SkillPlayerDistribution>,
}

impl TeamParamsDistribution {
    pub fn to_team_params(&self) -> TeamParams {
        TeamParams {
            team: self.team.clone(),
            qbs: self.qbs.clone(),
            skill_players: self
                .skill_players
                .iter()
                .map(|(k, v)| (k.clone(), v.to_skill_player()))
                .collect(),
            injuries: HashMap::new(),
        }
    }
}

pub enum DepthType {
    OneStarter,
    TwoStarters,
    ThreeStarters,
}

impl TeamParams {
    pub fn update_injuries(&mut self, injuries: HashMap<Position, HashMap<String, Injury>>) {
        let inj_state = &mut self.injuries;
        for (pos, pos_injuries) in injuries {
            if !inj_state.contains_key(&pos) {
                inj_state.insert(pos, HashMap::new());
            }
            let pos_inj_state = inj_state.get_mut(&pos).unwrap();
            for (key, injury) in pos_injuries {
                pos_inj_state.insert(key, injury);
            }
        }
    }

    pub fn apply_injuries(&mut self) {
        for (pos, pos_injuries) in self.injuries.clone() {
            self.apply_pos_injuries(pos.clone(), pos_injuries);
        }
    }

    pub fn depth_type(pos: Position, depth_charts: Vec<u8>) -> DepthType {
        let num_depth_1 = depth_charts
            .iter()
            .filter(|depth| **depth == 1)
            .map(|_| 1)
            .sum();
        match num_depth_1 {
            0 => panic!("no starters for {:?}", pos),
            1 => DepthType::OneStarter,
            2 => DepthType::TwoStarters,
            3 => DepthType::ThreeStarters,
            _ => panic!("too many ({}) starters for {:?}", num_depth_1, pos),
        }
    }

    pub fn apply_pos_injuries(&mut self, pos: Position, injuries: HashMap<String, Injury>) {
        let pos_players: Vec<&SkillPlayer> = self
            .skill_players
            .iter()
            .map(|(_, param)| param)
            .filter(|param| param.position == pos)
            .collect();

        let all_depth_charts = SkillPlayer::depth_charts(&pos_players);
        let depth_type = TeamParams::depth_type(pos, all_depth_charts);
        // calculate type of depth chart & edit market shares
        let injured_players = pos_players
            .iter()
            .cloned()
            .filter(|p| injuries.contains_key(&p.player_id))
            .collect();
        let non_injured_players: Vec<&SkillPlayer> = pos_players
            .iter()
            .cloned()
            .filter(|p| !injuries.contains_key(&p.player_id))
            .collect();
        let injured_depth_charts = SkillPlayer::depth_charts(&injured_players);
        let injured_ms_carries: f32 = injured_players.iter().map(|p| p.ms_carries_init).sum();
        let injured_ms_targets: f32 = injured_players.iter().map(|p| p.ms_targets_init).sum();
    }
}

#[derive(Clone, Debug)]
pub struct TeamParams {
    pub team: Team,
    pub qbs: Vec<Quarterback>,
    pub skill_players: HashMap<String, SkillPlayer>,
    pub injuries: HashMap<Position, HashMap<String, Injury>>,
}

#[derive(Clone, Debug)]
pub struct GameParamsDistribution {
    pub home: TeamParamsDistribution,
    pub away: TeamParamsDistribution,
    pub weather: Weather,
    pub neutral_field: bool,
}

impl GameParamsDistribution {
    pub fn to_game_params(&self) -> GameParams {
        GameParams {
            home: self.home.to_team_params(),
            away: self.away.to_team_params(),
            weather: self.weather.clone(),
            neutral_field: self.neutral_field,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameParams {
    pub home: TeamParams,
    pub away: TeamParams,
    pub weather: Weather,
    pub neutral_field: bool,
}

#[derive(Debug, Deserialize)]
pub struct GameLoader {
    pub home: String,
    pub away: String,
    pub time: String,
    pub stadium_type: StadiumType,
    pub neutral_field: Option<u8>,
}

impl GameLoader {
    pub fn load(path: &String) -> Vec<GameLoader> {
        log::info!("loading games from {}", path);
        let games_path = format!("{}/Games-Table 1.csv", path);
        let mut game_reader = Reader::from_path(&games_path)
            .expect(&format!("Failed to load file at {}", &games_path));

        let mut games = vec![];
        for game_csv in game_reader.deserialize() {
            let game: GameLoader = game_csv.expect("failed to parse game");
            games.push(game);
        }

        games
    }
}

impl GameParams {
    pub fn get_team(&self, team: HomeAway) -> &TeamParams {
        match team {
            HomeAway::Home => &self.home,
            HomeAway::Away => &self.away,
        }
    }

    pub fn update_injuries(
        &mut self,
        home_away: HomeAway,
        injuries: HashMap<Position, HashMap<String, Injury>>,
    ) {
        match home_away {
            HomeAway::Home => self.home.update_injuries(injuries),
            HomeAway::Away => self.away.update_injuries(injuries),
        }
    }

    pub fn load(path: String) -> Vec<GameParamsDistribution> {
        let games: Vec<GameLoader> = GameLoader::load(&path);
        let teams: HashMap<String, Team> = Team::load(&path);
        let qbs: HashMap<String, Vec<Quarterback>> = Quarterback::load(&path);
        let skill_players: HashMap<String, HashMap<String, SkillPlayerDistribution>> =
            SkillPlayer::load(&path);

        games
            .iter()
            .map(|g| GameParamsDistribution {
                home: TeamParamsDistribution {
                    team: teams[&g.home].clone(),
                    qbs: qbs[&g.home].clone(),
                    skill_players: skill_players[&g.home].clone(),
                },
                away: TeamParamsDistribution {
                    team: teams[&g.away].clone(),
                    qbs: qbs[&g.away].clone(),
                    skill_players: skill_players[&g.away].clone(),
                },
                weather: Weather {
                    stadium_type: g.stadium_type.clone(),
                    start_time: g.time.clone(),
                },
                neutral_field: match g.neutral_field {
                    None => false,
                    Some(0) => false,
                    _ => true,
                },
            })
            .collect()
    }

    pub fn opponents_map(game_params: &Vec<GameParamsDistribution>) -> HashMap<String, String> {
        let mut opponents = HashMap::new();
        for game in game_params {
            opponents.insert(game.home.team.team.clone(), game.away.team.team.clone());
            opponents.insert(game.away.team.team.clone(), game.home.team.team.clone());
        }
        opponents
    }

    fn insert_returner(
        map: &mut HashMap<SalaryKey, PlayerMeta>,
        ret_id_opt: &Option<String>,
        team: &String,
        opp: &String,
    ) {
        if let Some(ret_id) = ret_id_opt {
            let sk = SalaryKey::NflId(ret_id.clone());
            if !map.contains_key(&sk) {
                map.insert(
                    sk,
                    PlayerMeta {
                        name: "Returner".into(),
                        pos: Position::Returner,
                        team: team.clone(),
                        opp: opp.clone(),
                    },
                );
            }
        }
    }

    pub fn player_meta(
        game_params: &Vec<GameParamsDistribution>,
    ) -> HashMap<SalaryKey, PlayerMeta> {
        let mut map = HashMap::new();
        for game in game_params {
            let home_team = &game.home.team.team;
            let away_team = &game.away.team.team;
            map.insert(
                SalaryKey::TeamPos(home_team.clone(), Position::Kicker),
                PlayerMeta {
                    name: format!("{} Kicker", home_team),
                    pos: Position::Kicker,
                    team: home_team.clone(),
                    opp: away_team.clone(),
                },
            );
            map.insert(
                SalaryKey::TeamPos(home_team.clone(), Position::Defense),
                PlayerMeta {
                    name: format!("{} Defense", home_team),
                    pos: Position::Defense,
                    team: home_team.clone(),
                    opp: away_team.clone(),
                },
            );
            map.insert(
                SalaryKey::TeamPos(away_team.clone(), Position::Kicker),
                PlayerMeta {
                    name: format!("{} Kicker", away_team),
                    pos: Position::Kicker,
                    team: away_team.clone(),
                    opp: home_team.clone(),
                },
            );
            map.insert(
                SalaryKey::TeamPos(away_team.clone(), Position::Defense),
                PlayerMeta {
                    name: format!("{} Defense", away_team),
                    pos: Position::Defense,
                    team: away_team.clone(),
                    opp: home_team.clone(),
                },
            );
            for (_, sp) in &game.home.skill_players {
                map.insert(
                    SalaryKey::NflId(sp.player_id.clone()),
                    PlayerMeta {
                        name: sp.name.clone(),
                        pos: sp.position.clone(),
                        team: game.home.team.team.clone(),
                        opp: game.away.team.team.clone(),
                    },
                );
            }
            for (_, sp) in &game.away.skill_players {
                map.insert(
                    SalaryKey::NflId(sp.player_id.clone()),
                    PlayerMeta {
                        name: sp.name.clone(),
                        pos: sp.position.clone(),
                        team: game.away.team.team.clone(),
                        opp: game.home.team.team.clone(),
                    },
                );
            }
            let away_returners = vec![
                &game.away.team.punt_returner_id,
                &game.away.team.kickoff_returner_id,
            ];
            for ret_id_opt in away_returners {
                GameParams::insert_returner(
                    &mut map,
                    ret_id_opt,
                    &game.away.team.team,
                    &game.home.team.team,
                )
            }
            let home_returners = vec![
                &game.home.team.punt_returner_id,
                &game.home.team.kickoff_returner_id,
            ];
            for ret_id_opt in home_returners {
                GameParams::insert_returner(
                    &mut map,
                    ret_id_opt,
                    &game.home.team.team,
                    &game.away.team.team,
                )
            }
        }
        map
    }

    pub fn get_time_map(game_params: &Vec<GameParamsDistribution>) -> HashMap<String, String> {
        let mut time_map = HashMap::new();
        for game in game_params {
            time_map.insert(game.away.team.team.clone(), game.weather.start_time.clone());
            time_map.insert(game.home.team.team.clone(), game.weather.start_time.clone());
        }
        time_map
    }
}

pub struct RushingParams {
    pub yoe_mean: f32,
    pub yoe_std: f32,
}
