pub mod burn_in;
pub mod injury;
pub mod quarterback;
pub mod skill_player;
pub mod team;
pub mod weather;

use csv::Reader;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;

use crate::params::injury::Injury;
use crate::params::skill_player::{Position, SkillPlayerDistribution};
use crate::params::weather::StadiumType;
use crate::params::{
    quarterback::Quarterback, skill_player::SkillPlayer, team::Team, weather::Weather,
};
use crate::sim::box_score::PlayerKey;
use crate::start::HomeAway;

use self::burn_in::TeamFpParams;

const MAX_QB2_MS_RUSH: f32 = 0.05;

lazy_static! {
    static ref MAX_INJURIES_PER_POS: HashMap<Position, u8> = {
        let mut m = HashMap::new();
        m.insert(Position::Quarterback, 1);
        m.insert(Position::Halfback, 2);
        m.insert(Position::Fullback, 1);
        m.insert(Position::WideReceiver, 3);
        m.insert(Position::TightEnd, 2);
        m
    };
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PlayerMeta {
    pub name: String,
    pub pos: Position,
    pub team: String,
    pub opp: String,
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

    pub fn update_fp_params(&self, fp_params: &TeamFpParams) -> TeamParamsDistribution {
        let tm = &self.team;
        TeamParamsDistribution {
            team: Team {
                team: tm.team.clone(),
                pace_z: tm.pace_z,
                offense_proe: tm.offense_proe,
                offense_rz_proe: tm.offense_rz_proe,
                oline_rushing_z: tm.oline_rushing_z,
                offense_pass_rush_z: tm.offense_pass_rush_z,
                defense_proe: tm.defense_proe,
                defense_rz_proe: tm.defense_rz_proe,
                dline_rushing_z: tm.dline_rushing_z,
                defense_pass_rush_z: tm.defense_pass_rush_z,
                defense_completion_z: tm.defense_completion_z,
                defense_interception_z: tm.defense_interception_z,
                defense_yac_oe: tm.defense_yac_oe,
                short_fg_z: tm.short_fg_z,
                long_fg_z: tm.long_fg_z,
                offense_penalty_z: tm.offense_penalty_z,
                defense_penalty_z: tm.defense_penalty_z,
                kickoff_returner_id: tm.kickoff_returner_id.clone(),
                punt_returner_id: tm.punt_returner_id.clone(),
                prob_1ytg_given_carry: fp_params.prob_1ytg_given_carry,
                prob_gz_given_carry: fp_params.prob_gz_given_carry,
                prob_rz_given_target: fp_params.prob_rz_given_target,
            },
            qbs: self.qbs.clone(),
            skill_players: self.skill_players.clone(),
        }
    }

    pub fn update_ms_targets(
        &self,
        realized_ms_targets: &HashMap<String, f32>,
    ) -> TeamParamsDistribution {
        let mut skill_players = HashMap::new();
        for (player_id, sp) in &self.skill_players {
            let new_sp = sp.update_ms_targets(realized_ms_targets[player_id]);
            skill_players.insert(player_id.clone(), new_sp);
        }
        TeamParamsDistribution {
            team: self.team.clone(),
            qbs: self.qbs.clone(),
            skill_players,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DepthType {
    OneStarter,
    TwoStarters,
    ThreeStarters,
}

#[derive(Debug, Clone, Copy)]
pub enum InjuredDepthType {
    Starter,
    SecondString,
    ThirdString,
    Benchwarmer,
}

impl TeamParams {
    fn qb_at_depth(&self, depth: u8) -> (&Quarterback, &SkillPlayer) {
        for qb in &self.qbs {
            let skill = self.skill_players.get(&qb.player_id).unwrap();
            if skill.depth_chart == depth {
                return (qb, skill);
            }
        }
        panic!("no qb at depth {}", depth);
    }

    pub fn quarterback(&self) -> &Quarterback {
        let (qb1, qb1_skill) = self.qb_at_depth(1);
        let (qb2, _) = self.qb_at_depth(2);
        if qb1_skill.injury == Injury::Injured {
            // log::info!("QB2 playing for {}", self.team.team);
            return qb2;
        }
        return qb1;
    }

    pub fn update_injuries(&mut self, injuries: HashMap<Position, HashMap<String, Injury>>) {
        let inj_state = &mut self.injuries;
        for (pos, pos_injuries) in injuries {
            if !inj_state.contains_key(&pos) {
                inj_state.insert(pos, HashMap::new());
            }
            let pos_inj_state = inj_state.get_mut(&pos).unwrap();
            for (key, injury) in pos_injuries {
                if pos_inj_state.len() < *MAX_INJURIES_PER_POS.get(&pos).unwrap_or(&0) as usize {
                    pos_inj_state.insert(key, injury);
                }
            }
        }
    }

    pub fn apply_injuries(&mut self) {
        for (pos, pos_injuries) in self.injuries.clone() {
            self.apply_pos_injuries(pos.clone(), pos_injuries, self.team.team.clone());
        }
    }

    pub fn depth_type(depth_charts: Vec<u8>, team_pos: &String) -> DepthType {
        let num_depth_1 = depth_charts
            .iter()
            .filter(|depth| **depth == 1)
            .map(|_| 1)
            .sum();
        match num_depth_1 {
            0 => panic!("no starters for {}", team_pos),
            1 => DepthType::OneStarter,
            2 => DepthType::TwoStarters,
            3 => DepthType::ThreeStarters,
            _ => panic!("too many ({}) starters for {}", num_depth_1, team_pos),
        }
    }

    pub fn injured_depth_type(depth_charts: &Vec<u8>) -> InjuredDepthType {
        let num_depth_1: u8 = depth_charts
            .iter()
            .filter(|depth| **depth == 1)
            .map(|_| 1)
            .sum();
        let num_depth_2: u8 = depth_charts
            .iter()
            .filter(|depth| **depth == 2)
            .map(|_| 1)
            .sum();
        let num_depth_3: u8 = depth_charts
            .iter()
            .filter(|depth| **depth == 3)
            .map(|_| 1)
            .sum();
        if num_depth_1 > 0 {
            return InjuredDepthType::Starter;
        }
        if num_depth_2 > 0 {
            return InjuredDepthType::SecondString;
        }
        if num_depth_3 > 0 {
            return InjuredDepthType::ThirdString;
        }
        return InjuredDepthType::Benchwarmer;
    }

    fn get_extra_ms_carries(
        injured_ms_carries: f32,
        non_injured_players: &Vec<&SkillPlayer>,
        depth_type: DepthType,
        injured_depth_type: InjuredDepthType,
        pos: Position,
    ) -> HashMap<String, f32> {
        let mut extra_ms_carries = HashMap::new();

        for skill_player in non_injured_players {
            let mult = match depth_type {
                /*
                1 RB:
                - RB1 injured: RB1 gets 1/3, split remaining 3/4 pro rata
                - RB2 injured: RB1 gets 1/5, RB2-3 get 4/5 pro rata
                - RB3 injured: RB1 gets 1/10, split remaining 9/10 pro rata
                */
                DepthType::OneStarter => match injured_depth_type {
                    InjuredDepthType::Starter => 1.0,
                    InjuredDepthType::SecondString => 1.0,
                    InjuredDepthType::ThirdString => 1.0,
                    InjuredDepthType::Benchwarmer => 1.0,
                },
                /*
                2 RB:
                    - RB1 injured: RB1 gets 1/4, Of 3/4 remaining, split evenly pro rata
                    - RB2 injured: RB1s gets 1/3 each, split 1/3 to RB3-4
                    - RB3 injured: RB1s 1/4 + RB2 1/4 + rest get 1/4 combined
                */
                DepthType::TwoStarters => match injured_depth_type {
                    InjuredDepthType::Starter => 1.0,
                    InjuredDepthType::SecondString => 1.0,
                    InjuredDepthType::ThirdString => 1.0,
                    InjuredDepthType::Benchwarmer => 1.0,
                },
                /*
                3 WR1s
                    - WR1 injured. 1/10 WR1s, 2/5 WR2, 1/5 WR3, rest evenly
                    - WR2 injured. 1/10 WR1s, rest evenly
                    - WR3 injured. WR1s no change, rest evenly
                */
                DepthType::ThreeStarters => match injured_depth_type {
                    InjuredDepthType::Starter => 1.0,
                    InjuredDepthType::SecondString => 1.0,
                    InjuredDepthType::ThirdString => 1.0,
                    InjuredDepthType::Benchwarmer => 1.0,
                },
            };

            let msc_init = match pos {
                // when QB1 goes down, QB2 gets all of his MS carries,
                // so we have to do this because usually QB2 will have 0.0 ms_carries_init
                Position::Quarterback => skill_player.ms_carries_init.max(0.01),
                _ => skill_player.ms_carries_init,
            };
            extra_ms_carries.insert(skill_player.player_id.clone(), msc_init * mult);
        }

        let extra_sum: f32 = extra_ms_carries.values().sum();
        if extra_sum > 0.0 {
            for (_, v) in extra_ms_carries.iter_mut() {
                let raw_to_add = injured_ms_carries / extra_sum;
                let to_add = match pos {
                    // this is to cover for a situation where a high rushing volume QB1 gets injured.
                    // in this case, we don't want to give all of their carries to QB2,
                    // so we cap it at a fixed number
                    // Motivating example: Jalen Hurts with 30% MS carries getting injured
                    Position::Quarterback => raw_to_add.min(MAX_QB2_MS_RUSH),
                    _ => raw_to_add,
                };
                *v *= to_add;
            }
        }
        extra_ms_carries
    }

    fn get_extra_ms_targets(
        injured_ms_targets: f32,
        non_injured_players: &Vec<&SkillPlayer>,
        depth_type: DepthType,
        injured_depth_type: InjuredDepthType,
        pos: Position,
    ) -> HashMap<String, f32> {
        let mut extra_ms_targets = HashMap::new();

        if pos == Position::Quarterback {
            return extra_ms_targets;
        }

        for skill_player in non_injured_players {
            let mult = match depth_type {
                /*
                1 WR1 on depth chart
                    - WR1 injured. split evenly to remaining pro rata. everyone multiple = 1
                    - WR2 injured. WR1 1/10, split rest evenly
                    - WR3+ injured. WR1 no change, split rest evenly
                */
                DepthType::OneStarter => match injured_depth_type {
                    InjuredDepthType::Starter => 1.0,
                    InjuredDepthType::SecondString => 1.0,
                    InjuredDepthType::ThirdString => 1.0,
                    InjuredDepthType::Benchwarmer => 1.0,
                },
                /*
                2 WR1s
                - WR1 injured. 1/5 WR1, 2/5 WR2, 2/5 rest
                - WR2 injured. 1/10 WR1s, 1/2 WR2, rest evenly
                - WR3 injured. WR1s no change, rest split evenly
                - WR4 injured. WR1s no change, rest split evenly
                */
                DepthType::TwoStarters => match injured_depth_type {
                    InjuredDepthType::Starter => 1.0,
                    InjuredDepthType::SecondString => 1.0,
                    InjuredDepthType::ThirdString => 1.0,
                    InjuredDepthType::Benchwarmer => 1.0,
                },
                /*
                3 WR1s
                    - WR1 injured. 1/10 WR1s, 2/5 WR2, 1/5 WR3, rest evenly
                    - WR2 injured. 1/10 WR1s, rest evenly
                    - WR3 injured. WR1s no change, rest evenly
                */
                DepthType::ThreeStarters => match injured_depth_type {
                    InjuredDepthType::Starter => 1.0,
                    InjuredDepthType::SecondString => 1.0,
                    InjuredDepthType::ThirdString => 1.0,
                    InjuredDepthType::Benchwarmer => 1.0,
                },
            };
            extra_ms_targets.insert(
                skill_player.player_id.clone(),
                skill_player.ms_targets_init * mult,
            );
        }

        let extra_sum: f32 = extra_ms_targets.values().sum();
        if extra_sum > 0.0 {
            for (_, v) in extra_ms_targets.iter_mut() {
                *v *= injured_ms_targets / extra_sum;
            }
        }
        extra_ms_targets
    }

    pub fn apply_pos_injuries(
        &mut self,
        pos: Position,
        injuries: HashMap<String, Injury>,
        team: String,
    ) {
        let pos_players: Vec<&SkillPlayer> = self
            .skill_players
            .iter()
            .map(|(_, param)| param)
            .filter(|param| param.position == pos)
            .collect();

        let all_depth_charts = SkillPlayer::depth_charts(&pos_players);
        let team_pos = format!("{} {:?}", team, pos);
        let depth_type = TeamParams::depth_type(all_depth_charts, &team_pos);
        // calculate type of depth chart & edit market shares
        let injured_players = pos_players
            .iter()
            .cloned()
            .filter(|p| injuries.contains_key(&p.player_id))
            .collect();
        let injured_depth_charts = SkillPlayer::depth_charts(&injured_players);
        let non_injured_players: Vec<&SkillPlayer> = pos_players
            .iter()
            .cloned()
            .filter(|p| !injuries.contains_key(&p.player_id))
            .collect();
        let injured_depth_type = TeamParams::injured_depth_type(&injured_depth_charts);
        let injured_ms_carries: f32 = injured_players.iter().map(|p| p.ms_carries_init).sum();
        let extra_ms_carries = TeamParams::get_extra_ms_carries(
            injured_ms_carries,
            &non_injured_players,
            depth_type,
            injured_depth_type,
            pos,
        );
        let injured_ms_targets: f32 = injured_players.iter().map(|p| p.ms_targets_init).sum();
        let extra_ms_targets = TeamParams::get_extra_ms_targets(
            injured_ms_targets,
            &non_injured_players,
            depth_type,
            injured_depth_type,
            pos,
        );
        for (player_id, skill_player) in self.skill_players.iter_mut() {
            if injuries.contains_key(player_id) {
                skill_player.ms_carries_live = 0.0;
                skill_player.ms_targets_live = 0.0;
                skill_player.injury = Injury::Injured;
            } else {
                skill_player.ms_carries_live =
                    skill_player.ms_carries_init + *extra_ms_carries.get(player_id).unwrap_or(&0.0);
                skill_player.ms_targets_live =
                    skill_player.ms_targets_init + *extra_ms_targets.get(player_id).unwrap_or(&0.0);
                skill_player.injury = Injury::Healthy;
            }
        }
        let new_car_sum: f32 = self
            .skill_players
            .values()
            .map(|sp| sp.ms_carries_live)
            .sum();
        let new_tgt_sum: f32 = self
            .skill_players
            .values()
            .map(|sp| sp.ms_targets_live)
            .sum();
        if new_car_sum == 0.0 {
            log::info!("no carries for {}", team_pos);
            log::info!("injuries: {:?}", injuries);
            log::info!("{:#?}", self.skill_players);
        }
        if new_tgt_sum == 0.0 {
            log::info!("no targets for {}", team_pos);
            log::info!("injuries: {:?}", injuries);
            log::info!("{:#?}", self.skill_players);
        }
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
            sim_injuries: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameParams {
    pub home: TeamParams,
    pub away: TeamParams,
    pub weather: Weather,
    pub neutral_field: bool,
    pub sim_injuries: bool,
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
    pub fn injuries(&self, sim_injuries: bool) -> GameParams {
        GameParams {
            home: self.home.clone(),
            away: self.away.clone(),
            weather: self.weather.clone(),
            neutral_field: self.neutral_field,
            sim_injuries,
        }
    }

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
        map: &mut HashMap<PlayerKey, PlayerMeta>,
        ret_id_opt: &Option<String>,
        team: &String,
        opp: &String,
    ) {
        if let Some(ret_id) = ret_id_opt {
            let sk = PlayerKey::NflId(ret_id.clone());
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
    ) -> HashMap<PlayerKey, PlayerMeta> {
        let mut map = HashMap::new();
        for game in game_params {
            let home_team = &game.home.team.team;
            let away_team = &game.away.team.team;
            map.insert(
                PlayerKey::TeamPos(home_team.clone(), Position::Kicker),
                PlayerMeta {
                    name: format!("{} Kicker", home_team),
                    pos: Position::Kicker,
                    team: home_team.clone(),
                    opp: away_team.clone(),
                },
            );
            map.insert(
                PlayerKey::TeamPos(home_team.clone(), Position::Defense),
                PlayerMeta {
                    name: format!("{} Defense", home_team),
                    pos: Position::Defense,
                    team: home_team.clone(),
                    opp: away_team.clone(),
                },
            );
            map.insert(
                PlayerKey::TeamPos(away_team.clone(), Position::Kicker),
                PlayerMeta {
                    name: format!("{} Kicker", away_team),
                    pos: Position::Kicker,
                    team: away_team.clone(),
                    opp: home_team.clone(),
                },
            );
            map.insert(
                PlayerKey::TeamPos(away_team.clone(), Position::Defense),
                PlayerMeta {
                    name: format!("{} Defense", away_team),
                    pos: Position::Defense,
                    team: away_team.clone(),
                    opp: home_team.clone(),
                },
            );
            for (_, sp) in &game.home.skill_players {
                map.insert(
                    PlayerKey::NflId(sp.player_id.clone()),
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
                    PlayerKey::NflId(sp.player_id.clone()),
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

    pub fn update_fp_params(
        gp_dist: &Vec<GameParamsDistribution>,
        team_fp_params: &HashMap<String, TeamFpParams>,
    ) -> Vec<GameParamsDistribution> {
        gp_dist
            .iter()
            .cloned()
            .map(|gp| GameParamsDistribution {
                home: gp
                    .home
                    .update_fp_params(&team_fp_params[&gp.home.team.team]),
                away: gp
                    .away
                    .update_fp_params(&team_fp_params[&gp.away.team.team]),
                weather: gp.weather,
                neutral_field: gp.neutral_field,
            })
            .collect()
    }

    pub fn update_ms_targets(
        gp_dist: &Vec<GameParamsDistribution>,
        realized_ms_targets: &HashMap<String, f32>,
    ) -> Vec<GameParamsDistribution> {
        gp_dist
            .iter()
            .cloned()
            .map(|gp| GameParamsDistribution {
                home: gp.home.update_ms_targets(realized_ms_targets),
                away: gp.away.update_ms_targets(realized_ms_targets),
                weather: gp.weather,
                neutral_field: gp.neutral_field,
            })
            .collect()
    }
}

pub struct RushingParams {
    pub yoe_mean: f32,
    pub yoe_std: f32,
}
