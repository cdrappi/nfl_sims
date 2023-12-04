use crate::{
    game_loop::field_goals::ENDZONE_LENGTH,
    sim::{play_result::KickingResult, GameSim},
    state::yards_to_goal::YardsToGoal,
    util::stats::{random_bool, random_sigmoid, truncated_negbinom, truncated_poisson},
};

use crate::models::features::PlaycallFeatures;

pub mod coef;

const MIN_KICKOFF_DISTANCE: u8 = 25;
const ONSIDE_RECOVER_PROB: f32 = 0.11;
const KICKOFF_FROM: u8 = 35;

pub struct KickoffModel {
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
    kicked_from_pct: f32,
    log_kicked_from_pct: f32,
    kicked_to_pct: f32,
    log_kicked_to_pct: f32,
}

impl KickoffModel {
    pub fn get_result(sim: &GameSim) -> KickingResult {
        let mut features = KickoffModel::features(sim, 0.0);
        if KickoffModel::is_onside_kick(&features) {
            return match random_bool(ONSIDE_RECOVER_PROB) {
                true => KickingResult::OnsideRecovery(YardsToGoal(60)),
                false => KickingResult::ReturnedForYards(YardsToGoal(40)),
            };
        }
        if KickoffModel::is_out_of_bounds_kick(&features) {
            return KickingResult::ReturnedForYards(YardsToGoal(60));
        }
        if KickoffModel::is_touchback(&features) {
            return KickingResult::KickoffTouchback;
        }

        let kickoff_distance = KickoffModel::sample_kickoff_distance(&features);
        features = KickoffModel::features(sim, kickoff_distance as f32);

        if KickoffModel::is_kickoff_return_td(&features) {
            return KickingResult::ReturnedForTouchdown;
        }

        let kicked_to = KICKOFF_FROM + kickoff_distance;
        let return_yards = KickoffModel::sample_kickoff_return_yards(&features, kicked_to);
        let returned_to = kicked_to - return_yards;
        let ytg = match returned_to {
            // NOTE: formally should be 99, but probably better to treat 95-99 as touchback
            0..=99 => YardsToGoal::new(returned_to),
            _ => YardsToGoal::kickoff_touchback(),
        };
        // log::info!(
        //     "Kickoff returned to {} - ryds: {} kt: {}",
        //     returned_to,
        //     return_yards,
        //     kicked_to
        // );
        KickingResult::ReturnedForYards(ytg)
    }

    fn is_onside_kick(features: &KickoffModel) -> bool {
        let coef = KickoffModel::onside_kick_coef();
        let z = KickoffModel::get_z(&coef, features);
        random_sigmoid(z)
    }

    fn is_out_of_bounds_kick(features: &KickoffModel) -> bool {
        let coef = KickoffModel::out_of_bounds_kick_coef();
        let z = KickoffModel::get_z(&coef, features);
        random_sigmoid(z)
    }

    fn is_touchback(features: &KickoffModel) -> bool {
        let coef = KickoffModel::touchback_coef();
        let z = KickoffModel::get_z(&coef, features);
        random_sigmoid(z)
    }

    fn sample_kickoff_distance(features: &KickoffModel) -> u8 {
        let coef = KickoffModel::kickoff_distance_coef();
        let offset_mean = KickoffModel::get_z(&coef, features).exp();
        let max_exclusive = (100 - KICKOFF_FROM) + ENDZONE_LENGTH - MIN_KICKOFF_DISTANCE;
        MIN_KICKOFF_DISTANCE + truncated_poisson(offset_mean, max_exclusive) as u8
    }

    fn is_kickoff_return_td(features: &KickoffModel) -> bool {
        let coef = KickoffModel::kickoff_return_td_coef();
        let z = KickoffModel::get_z(&coef, features);
        random_sigmoid(z)
    }

    fn sample_kickoff_return_yards(features: &KickoffModel, kicked_to: u8) -> u8 {
        let mean_coef = KickoffModel::kickoff_return_yards_coef();
        let var_coef = KickoffModel::kickoff_return_yards_var_coef();
        let mean = KickoffModel::get_z(&mean_coef, features).exp();
        let var = KickoffModel::get_z(&var_coef, features).exp();
        match var > mean {
            true => truncated_negbinom(mean, var, kicked_to) as u8,
            false => truncated_poisson(mean, kicked_to) as u8,
        }
    }

    fn features(sim: &GameSim, kickoff_distance: f32) -> KickoffModel {
        let game_minutes_remaining = sim.game_state.clock.game_minutes_remaining();
        let f = PlaycallFeatures::new(sim);

        let possession = sim.game_state.play.possession();
        let possdiff_plus_1 = sim.game_state.score.possdiff_score_n(possession, 1) as f32;
        let possdiff_plus_2 = sim.game_state.score.possdiff_score_n(possession, 2) as f32;

        let kicked_to_pct = (KICKOFF_FROM as f32 + kickoff_distance) / 100.0;
        let kicked_from_pct = KICKOFF_FROM as f32 / 100.0;

        KickoffModel {
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
            kicked_from_pct,
            log_kicked_from_pct: kicked_from_pct.max(0.01).ln(),
            kicked_to_pct,
            log_kicked_to_pct: kicked_to_pct.max(0.01).ln(),
        }
    }

    fn get_z(c: &KickoffModel, f: &KickoffModel) -> f32 {
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
        z += c.kicked_from_pct * f.kicked_from_pct;
        z += c.log_kicked_from_pct * f.log_kicked_from_pct;
        z += c.kicked_to_pct * f.kicked_to_pct;
        z += c.log_kicked_to_pct * f.log_kicked_to_pct;
        z
    }
}
