pub mod quarterback;
pub mod skill_player;
pub mod team;
pub mod weather;
use csv::Reader;

use std::collections::HashMap;

use crate::sim::box_score::SalaryKey;
use crate::start::HomeAway;
use crate::state::clock::Quarter;

use crate::params::{
    quarterback::Quarterback, skill_player::SkillPlayer, team::Team, weather::Weather,
};

use self::skill_player::{Position, SkillPlayerDistribution};
use self::weather::StadiumType;
use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PlayerMeta {
    pub name: String,
    pub pos: Position,
    pub team: String,
    pub opp: String,
}

#[derive(Clone, Hash, PartialEq, serde::Serialize)]
pub enum Injury {
    Healthy,
    // if injured, when will they return?
    Injured(Quarter),
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
        }
    }
}

#[derive(Clone, Debug)]
pub struct TeamParams {
    pub team: Team,
    pub qbs: Vec<Quarterback>,
    pub skill_players: HashMap<String, SkillPlayer>,
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
