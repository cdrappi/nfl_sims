pub mod coef;

use crate::models::features::PlaycallFeatures;
use crate::{sim::GameSim, util::stats::random_sigmoid};

pub struct TwoPointAttemptModel {
    intercept: f32,
    inv_half_minutes: f32,
    log_inv_half_minutes: f32,
    inv_game_minutes: f32,
    log_inv_game_minutes: f32,
    possession_diff: f32,
    fg_possession_diff: f32,
    possdiff_per_minute: f32,
    fgpossdiff_per_minute: f32,
    off_timeouts_remaining_0: f32,
    off_timeouts_remaining_1: f32,
    off_timeouts_remaining_2: f32,
    off_timeouts_remaining_3: f32,
    def_timeouts_remaining_0: f32,
    def_timeouts_remaining_1: f32,
    def_timeouts_remaining_2: f32,
    def_timeouts_remaining_3: f32,
    garbage_time_win: f32,
    garbage_time_loss: f32,
    possdiff_plus_1: f32,
    possdiff_p1_per_minute: f32,
    possdiff_plus_2: f32,
    possdiff_p2_per_minute: f32,
}

impl TwoPointAttemptModel {
    pub fn goes_for_2(sim: &GameSim) -> bool {
        let coef = TwoPointAttemptModel::coefs();
        let features = TwoPointAttemptModel::features(sim);
        let z = TwoPointAttemptModel::get_z(&coef, &features);
        random_sigmoid(z)
    }

    fn features(sim: &GameSim) -> TwoPointAttemptModel {
        let game_minutes_remaining = sim.game_state.clock.game_minutes_remaining();
        let f = PlaycallFeatures::new(sim);

        let possession = sim.game_state.play.possession();
        let possdiff_plus_1 = sim.game_state.score.possdiff_score_n(possession, 1) as f32;
        let possdiff_plus_2 = sim.game_state.score.possdiff_score_n(possession, 2) as f32;

        TwoPointAttemptModel {
            intercept: 1.0,
            inv_half_minutes: f.inv_half_minutes,
            log_inv_half_minutes: f.log_inv_half_minutes,
            inv_game_minutes: f.inv_game_minutes,
            log_inv_game_minutes: f.log_inv_game_minutes,
            possession_diff: f.possession_diff,
            fg_possession_diff: f.fg_possession_diff,
            possdiff_per_minute: f.possdiff_per_minute,
            fgpossdiff_per_minute: f.fgpossdiff_per_minute,
            off_timeouts_remaining_0: f.off_timeouts_remaining_0,
            off_timeouts_remaining_1: f.off_timeouts_remaining_1,
            off_timeouts_remaining_2: f.off_timeouts_remaining_2,
            off_timeouts_remaining_3: f.off_timeouts_remaining_3,
            def_timeouts_remaining_0: f.def_timeouts_remaining_0,
            def_timeouts_remaining_1: f.def_timeouts_remaining_1,
            def_timeouts_remaining_2: f.def_timeouts_remaining_2,
            def_timeouts_remaining_3: f.def_timeouts_remaining_3,
            garbage_time_win: f.garbage_time_win,
            garbage_time_loss: f.garbage_time_loss,
            possdiff_plus_1: possdiff_plus_1,
            possdiff_p1_per_minute: possdiff_plus_1 / game_minutes_remaining,
            possdiff_plus_2: possdiff_plus_2,
            possdiff_p2_per_minute: possdiff_plus_2 / game_minutes_remaining,
        }
    }

    fn get_z(c: &TwoPointAttemptModel, f: &TwoPointAttemptModel) -> f32 {
        let mut z = c.intercept;
        z += c.inv_half_minutes * f.inv_half_minutes;
        z += c.log_inv_half_minutes * f.log_inv_half_minutes;
        z += c.inv_game_minutes * f.inv_game_minutes;
        z += c.log_inv_game_minutes * f.log_inv_game_minutes;
        z += c.possession_diff * f.possession_diff;
        z += c.fg_possession_diff * f.fg_possession_diff;
        z += c.possdiff_per_minute * f.possdiff_per_minute;
        z += c.fgpossdiff_per_minute * f.fgpossdiff_per_minute;
        z += c.off_timeouts_remaining_0 * f.off_timeouts_remaining_0;
        z += c.off_timeouts_remaining_1 * f.off_timeouts_remaining_1;
        z += c.off_timeouts_remaining_2 * f.off_timeouts_remaining_2;
        z += c.off_timeouts_remaining_3 * f.off_timeouts_remaining_3;
        z += c.def_timeouts_remaining_0 * f.def_timeouts_remaining_0;
        z += c.def_timeouts_remaining_1 * f.def_timeouts_remaining_1;
        z += c.def_timeouts_remaining_2 * f.def_timeouts_remaining_2;
        z += c.def_timeouts_remaining_3 * f.def_timeouts_remaining_3;
        z += c.garbage_time_win * f.garbage_time_win;
        z += c.garbage_time_loss * f.garbage_time_loss;
        z += c.possdiff_plus_1 * f.possdiff_plus_1;
        z += c.possdiff_p1_per_minute * f.possdiff_p1_per_minute;
        z += c.possdiff_plus_2 * f.possdiff_plus_2;
        z += c.possdiff_p2_per_minute * f.possdiff_p2_per_minute;
        z
    }
}
