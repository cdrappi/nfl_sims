extern crate env_logger;
extern crate nfl_pbp_sim;

use std::env;

use nfl_pbp_sim::{
    params::{burn_in::burn_in_params, GameParams},
    projections::{math::accumulate_projections, writer::ProjectionsWriter},
    sim::run::sim_box_scores_rayon,
};

const SLATE_ID: &str = "2023-12-11";
const N_SIMS: u32 = 50_000;
const SIM_INJURIES: bool = true;

fn main() {
    let slates_dir = get_slates_dir();
    run_slate(format!("{}/{}", slates_dir, SLATE_ID));
}

fn get_slates_dir() -> String {
    let nfl_sims_path =
        env::var("NFL_SIMS_PATH").expect("could not find environment variable NFL_SIMS_PATH");
    format!("{}/data/slates", nfl_sims_path)
}

fn run_slate(slate_dir: String) {
    env_logger::init();
    let game_params_vec = GameParams::load(format!("{}/params", &slate_dir));
    let player_meta = GameParams::player_meta(&game_params_vec);

    let game_params_vec = burn_in_params(5_000, &game_params_vec, &player_meta);

    let opponents: std::collections::HashMap<String, String> =
        GameParams::opponents_map(&game_params_vec);

    let box_scores = sim_box_scores_rayon(N_SIMS, &game_params_vec, SIM_INJURIES, "Sims");
    let projections = accumulate_projections(&box_scores, false);
    let proj_writer = ProjectionsWriter::new(&slate_dir, &projections, &player_meta, &opponents);
    proj_writer.write_projections();
}
