pub mod coef;

use crate::util::stats::random_sigmoid;

use crate::{models::features::PlaycallFeatures, sim::GameSim};

pub struct OffensiveTimeoutModel {
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
    off_timeouts_remaining_0: f32,
    off_timeouts_remaining_1: f32,
    off_timeouts_remaining_2: f32,
    off_timeouts_remaining_3: f32,
    clock_runs_pdpm_off0to: f32,
    clock_runs_pdpm_off1to: f32,
    clock_runs_pdpm_off2to: f32,
    clock_runs_pdpm_off3to: f32,
}

impl OffensiveTimeoutModel {
    pub fn calls_timeout(sim: &GameSim) -> bool {
        let game_state = &sim.game_state;
        let poss = game_state.play.possession();
        if !game_state.timeouts_used.can_use_timeout(poss) {
            return false;
        }
        let coefs = OffensiveTimeoutModel::coefs();
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
        z += coefs.off_timeouts_remaining_0 * f.off_timeouts_remaining_0;
        z += coefs.off_timeouts_remaining_1 * f.off_timeouts_remaining_1;
        z += coefs.off_timeouts_remaining_2 * f.off_timeouts_remaining_2;
        z += coefs.off_timeouts_remaining_3 * f.off_timeouts_remaining_3;
        z += coefs.clock_runs_pdpm_off0to * f.clock_runs_pdpm_off0to;
        z += coefs.clock_runs_pdpm_off1to * f.clock_runs_pdpm_off1to;
        z += coefs.clock_runs_pdpm_off2to * f.clock_runs_pdpm_off2to;
        z += coefs.clock_runs_pdpm_off3to * f.clock_runs_pdpm_off3to;
        random_sigmoid(z)
    }
}
