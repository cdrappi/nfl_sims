extern crate csv;
extern crate env_logger;
extern crate lazy_static;
extern crate log;
extern crate progress_bar;
extern crate rust_decimal;
extern crate serde;
extern crate serde_json;

pub mod box_score;
pub mod game_loop;
pub mod models;
pub mod params;
pub mod sim;
pub mod start;
pub mod state;
pub mod util;

// use params::GameParams;
// use crate::sim::sim_game;

// const SLATES_DIR: &str = "/Users/christiandrappi/code/etr/data/slates";
// const SLATE: &str = "10-04-SHOWDOWN";

/*
fn main() {``
    env_logger::init();

    let path = format!("{}/{}/params", SLATES_DIR, SLATE);
    let game_params: Vec<GameParams> = GameParams::load(path);

    log::info!("loaded {} games", game_params.len());
    // log::info!("first game: {:?}", game_params[0]);

    let bs = sim_game(&game_params[0]);
    let fp = bs.fantasy_points();
    // log::info!("BOX SCORE\n{:?}", bs);
}
*/
