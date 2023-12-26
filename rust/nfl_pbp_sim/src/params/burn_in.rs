use std::collections::HashMap;

use crate::{
    params::{GameParams, GameParamsDistribution, PlayerMeta},
    sim::{
        box_score::{BoxScore, PlayerKey},
        run::sim_box_scores_rayon,
    },
    state::game_state::TeamPlays,
};

pub fn burn_in_params(
    n: u32,
    game_params_vec: &Vec<GameParamsDistribution>,
    player_meta: &HashMap<PlayerKey, PlayerMeta>,
) -> Vec<GameParamsDistribution> {
    // let burn_in_box_scores = sim_many(N_BURN_IN_FP, &game_params_vec);
    let burn_in_box_scores = sim_box_scores_rayon(n, &game_params_vec, false, "FP burn-in");
    let team_fp_params = accumulate_team_fp_params(&burn_in_box_scores);
    // log::info!("Team FP params:\n{:#?}", &team_fp_params);
    let game_params_fp: Vec<GameParamsDistribution> =
        GameParams::update_fp_params(&game_params_vec, &team_fp_params);

    let burn_in_ms_tgt = sim_box_scores_rayon(n, &game_params_fp, false, "MST burn-in");
    let mst_updates = accumulate_ms_targets(&burn_in_ms_tgt, player_meta);
    let game_params_mst = GameParams::update_ms_targets(&game_params_vec, &mst_updates);

    game_params_mst
}

#[derive(Debug)]
pub struct TeamFpCounts {
    pub targets_rz: usize,
    pub carries_1ytg: usize,
    pub carries_gz: usize,
    pub n_targets: usize,
    pub n_carries: usize,
}

#[derive(Debug)]
pub struct TeamFpParams {
    pub prob_rz_given_target: f32,
    pub prob_1ytg_given_carry: f32,
    pub prob_gz_given_carry: f32,
}

impl TeamFpCounts {
    pub fn new() -> TeamFpCounts {
        TeamFpCounts {
            carries_1ytg: 0,
            carries_gz: 0,
            targets_rz: 0,
            n_targets: 0,
            n_carries: 0,
        }
    }

    pub fn to_probs(&self) -> TeamFpParams {
        let n_carries = self.n_carries as f32;
        TeamFpParams {
            prob_1ytg_given_carry: self.carries_1ytg as f32 / n_carries,
            prob_gz_given_carry: self.carries_gz as f32 / n_carries,
            prob_rz_given_target: self.targets_rz as f32 / self.n_targets as f32,
        }
    }

    pub fn add(&mut self, team_plays: &TeamPlays) {
        self.n_targets += team_plays.targets as usize;
        self.n_carries += team_plays.run as usize;
        self.carries_1ytg += team_plays.run_1ytg as usize;
        self.carries_gz += team_plays.run_gz as usize;
        self.targets_rz += team_plays.targets_rz as usize;
    }
}

fn accumulate_team_fp_counts(sims: &Vec<Vec<BoxScore>>) -> HashMap<String, TeamFpCounts> {
    let mut team_fp_counts = HashMap::new();
    for box_scores in sims {
        for box_score in box_scores {
            let away_team = box_score.away.clone();
            if !team_fp_counts.contains_key(&away_team) {
                team_fp_counts.insert(away_team.clone(), TeamFpCounts::new());
            }
            let away_fp = team_fp_counts.get_mut(&away_team).unwrap();
            away_fp.add(&box_score.plays.away);

            let home_team = box_score.home.clone();
            if !team_fp_counts.contains_key(&home_team) {
                team_fp_counts.insert(home_team.clone(), TeamFpCounts::new());
            }
            let home_fp = team_fp_counts.get_mut(&home_team).unwrap();
            home_fp.add(&box_score.plays.home);
        }
    }
    team_fp_counts
}

fn accumulate_team_fp_params(sims: &Vec<Vec<BoxScore>>) -> HashMap<String, TeamFpParams> {
    let team_fp_counts = accumulate_team_fp_counts(sims);
    let mut team_fp_params = HashMap::new();
    for (team, fp_params) in team_fp_counts {
        team_fp_params.insert(team, fp_params.to_probs());
    }
    team_fp_params
}

fn accumulate_ms_targets(
    sims: &Vec<Vec<BoxScore>>,
    player_meta: &HashMap<PlayerKey, PlayerMeta>,
) -> HashMap<String, f32> {
    let mut target_counts = HashMap::new();
    let mut team_target_counts = HashMap::new();
    for box_scores in sims {
        for box_score in box_scores {
            for (player_id, spb) in &box_score.skill_players {
                if !target_counts.contains_key(player_id) {
                    target_counts.insert(player_id.clone(), 0.0);
                }
                let player_tgts = target_counts.get_mut(player_id).unwrap();
                *player_tgts += spb.targets as f32;

                let team = &player_meta[&PlayerKey::NflId(player_id.clone())].team;
                if !team_target_counts.contains_key(team) {
                    team_target_counts.insert(team.clone(), 0.0);
                }
                let team_tgts = team_target_counts.get_mut(team).unwrap();
                *team_tgts += spb.targets as f32;
            }
        }
    }
    // log::info!("N sims = {}", sims.len());
    // log::info!("Player targets:\n{:#?}", target_counts);
    // log::info!("Team target counts:\n{:#?}", team_target_counts);

    let mut ms_targets = HashMap::new();
    for (player_id, target_count) in target_counts {
        let team = &player_meta[&PlayerKey::NflId(player_id.clone())].team;
        ms_targets.insert(player_id.clone(), target_count / team_target_counts[team]);
    }
    // log::info!("Ms targets:\n{:#?}", ms_targets);
    ms_targets
}
