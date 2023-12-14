use progress_bar::*;
use rayon::prelude::*;

use crate::{
    params::GameParamsDistribution,
    sim::{box_score::BoxScore, sim_game},
    util::clock::mins_secs,
};

pub const SIM_CHUNK_SIZE: usize = 200;

pub fn sim_box_scores_rayon(
    n: u32,
    game_params: &Vec<GameParamsDistribution>,
    sim_injuries: bool,
    progress_name: &str,
) -> Vec<Vec<BoxScore>> {
    init_progress_bar(n as usize);
    enable_eta();
    set_progress_bar_action(progress_name, Color::Green, Style::Normal);
    let start_time = std::time::Instant::now();

    let box_scores = (0..n)
        .into_par_iter()
        .chunks(SIM_CHUNK_SIZE)
        .flat_map(|chunk| {
            chunk.into_par_iter().map(|_ii| {
                let gp = game_params
                    .iter()
                    .map(|gp| sim_game(gp, sim_injuries))
                    .collect::<Vec<BoxScore>>();
                inc_progress_bar();
                gp
            })
        })
        .collect::<Vec<Vec<BoxScore>>>();

    finalize_progress_bar();

    let end_sim_time = std::time::Instant::now();
    let (mins, secs) = mins_secs(end_sim_time - start_time);
    log::info!(
        "Simmed {} slates in {:.0}m {:.0}s",
        box_scores.len(),
        mins,
        secs
    );
    box_scores
}

pub fn sim_many(
    n_sims: u32,
    game_params: &Vec<GameParamsDistribution>,
    sim_injuries: bool,
) -> Vec<Vec<BoxScore>> {
    let mut all_box_scores = vec![];
    for _ in 0..n_sims {
        let mut box_scores = vec![];
        for gp in game_params {
            box_scores.push(sim_game(gp, sim_injuries));
        }
        all_box_scores.push(box_scores)
    }
    all_box_scores
}
