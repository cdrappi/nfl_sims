use crate::util::stats::random_sigmoid;
use crate::{models::features::PlaycallFeatures, sim::GameSim};

pub mod coef;

pub struct DefensiveTimeoutModel {
    intercept: f32,
    clock_running: f32,
    inv_half_minutes: f32,
    log_inv_half_minutes: f32,
    inv_game_minutes: f32,
    log_inv_game_minutes: f32,
    possession_diff: f32,
    fg_possession_diff: f32,
    possdiff_per_minute: f32,
    fgpossdiff_per_minute: f32,
    clock_runs_pdpm: f32,
    clock_runs_fgpdpm: f32,
    clock_runs_pdpm2: f32,
    clock_runs_fgpdpm2: f32,
    def_timeouts_remaining_0: f32,
    def_timeouts_remaining_1: f32,
    def_timeouts_remaining_2: f32,
    def_timeouts_remaining_3: f32,
    clock_runs_pdpm_def0to: f32,
    clock_runs_pdpm_def1to: f32,
    clock_runs_pdpm_def2to: f32,
    clock_runs_pdpm_def3to: f32,
}

impl DefensiveTimeoutModel {
    pub fn calls_timeout(sim: &GameSim) -> bool {
        let game_state = &sim.game_state;
        let def = game_state.play.possession().flip();
        if !game_state.timeouts_used.can_use_timeout(def) {
            return false;
        }

        let coefs = DefensiveTimeoutModel::coefs();
        let f = PlaycallFeatures::new(sim);

        let mut z = coefs.intercept;
        z += coefs.clock_running * f.clock_running;
        z += coefs.inv_half_minutes * f.inv_half_minutes;
        z += coefs.log_inv_half_minutes * f.log_inv_half_minutes;
        z += coefs.inv_game_minutes * f.inv_game_minutes;
        z += coefs.log_inv_game_minutes * f.log_inv_game_minutes;
        z += coefs.possession_diff * f.possession_diff;
        z += coefs.fg_possession_diff * f.fg_possession_diff;
        z += coefs.possdiff_per_minute * f.possdiff_per_minute;
        z += coefs.fgpossdiff_per_minute * f.fgpossdiff_per_minute;
        z += coefs.clock_runs_pdpm * f.clock_runs_pdpm;
        z += coefs.clock_runs_fgpdpm * f.clock_runs_fgpdpm;
        z += coefs.clock_runs_pdpm2 * f.clock_runs_pdpm2;
        z += coefs.clock_runs_fgpdpm2 * f.clock_runs_fgpdpm2;
        z += coefs.def_timeouts_remaining_0 * f.def_timeouts_remaining_0;
        z += coefs.def_timeouts_remaining_1 * f.def_timeouts_remaining_1;
        z += coefs.def_timeouts_remaining_2 * f.def_timeouts_remaining_2;
        z += coefs.def_timeouts_remaining_3 * f.def_timeouts_remaining_3;
        z += coefs.clock_runs_pdpm_def0to * f.clock_runs_pdpm_def0to;
        z += coefs.clock_runs_pdpm_def1to * f.clock_runs_pdpm_def1to;
        z += coefs.clock_runs_pdpm_def2to * f.clock_runs_pdpm_def2to;
        z += coefs.clock_runs_pdpm_def3to * f.clock_runs_pdpm_def3to;

        random_sigmoid(z)
    }
}
