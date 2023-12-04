pub mod coef;

use crate::game_loop::field_goals::fg_distance;
use crate::util::stats::random_sigmoid;
use crate::{
    models::features::PlaycallFeatures,
    sim::{play_result::PlaycallResult, GameSim},
};

pub struct PlaycallModel {
    intercept: f32,
    is_offense_home: f32,
    offense_log_pass_prob: f32,
    defense_log_pass_prob: f32,
    off_def_lpp: f32,
    off_lpp_rz: f32,
    def_lpp_rz: f32,
    off_def_lpp_rz: f32,
    off_lpp_outside_rz: f32,
    off_lpp_inside_rz: f32,
    def_lpp_outside_rz: f32,
    def_lpp_inside_rz: f32,
    off_lpp_pdpm: f32,
    def_lpp_pdpm: f32,
    off_lpp_rz_pdpm: f32,
    def_lpp_rz_pdpm: f32,
    down_1: f32,
    down_2: f32,
    down_3: f32,
    down_4: f32,
    goal_to_go: f32,
    z_ydstogo: f32,
    ydstogo_pct: f32,
    log_ydstogo_pct: f32,
    to_go_1st: f32,
    to_go_2nd: f32,
    to_go_3rd: f32,
    to_go_4th: f32,
    log_to_go_1st: f32,
    log_to_go_2nd: f32,
    log_to_go_3rd: f32,
    log_to_go_4th: f32,
    fp_1st: f32,
    fp_2nd: f32,
    fp_3rd: f32,
    fp_4th: f32,
    fg_sigmoid: f32,
    punt_sigmoid: f32,
    yardline_pct: f32,
    yardline_pct_sq: f32,
    log_yardline_pct: f32,
    yardline_fgsig_4th: f32,
    yardline_puntsig_4th: f32,
    goal_to_go_yardline: f32,
    log_goal_to_go_yardline: f32,
    yards_to_go_yardline: f32,
    log_yards_to_go_yardline: f32,
    yardline_4th: f32,
    log_yardline_4th: f32,
    yardline_not_4th: f32,
    log_yardline_not_4th: f32,
    inside_2m_warning: f32,
    garbage_time_win: f32,
    garbage_time_loss: f32,
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
    def_timeouts_remaining_0: f32,
    def_timeouts_remaining_1: f32,
    def_timeouts_remaining_2: f32,
    def_timeouts_remaining_3: f32,
    clock_runs_pdpm_def0to: f32,
    clock_runs_pdpm_def1to: f32,
    clock_runs_pdpm_def2to: f32,
    clock_runs_pdpm_def3to: f32,
    offense_penalty_z: f32,
    defense_penalty_z: f32,
    off_def_penalty_z: f32,
}

impl PlaycallModel {
    pub fn sample_playcall(sim: &GameSim) -> PlaycallResult {
        let f = PlaycallFeatures::new(sim);
        // log::debug!("{:?}", f);

        if PlaycallModel::is_offensive_penalty(&f) {
            return PlaycallResult::OffensivePenalty;
        }
        if PlaycallModel::is_defensive_penalty(&f) {
            return PlaycallResult::DefensivePenalty;
        }
        if PlaycallModel::is_qb_kneel(&f, sim) {
            return PlaycallResult::QbKneel;
        }
        if PlaycallModel::is_qb_spike(&f, sim) {
            return PlaycallResult::QbSpike;
        }
        if PlaycallModel::is_field_goal_attempt(&f, sim) {
            return PlaycallResult::FieldGoalAttempt;
        }
        if PlaycallModel::is_punt(&f) {
            return PlaycallResult::Punt;
        }
        if PlaycallModel::is_qb_dropback(&f) {
            return PlaycallResult::QbDropback;
        }
        return PlaycallResult::DesignedRush;
    }

    fn is_offensive_penalty(f: &PlaycallFeatures) -> bool {
        let coefs = PlaycallModel::offensive_penalty_coefs();
        PlaycallModel::sample_event(f, coefs)
    }

    fn is_defensive_penalty(f: &PlaycallFeatures) -> bool {
        let coefs = PlaycallModel::defensive_penalty_coefs();
        PlaycallModel::sample_event(f, coefs)
    }

    fn is_field_goal_attempt(f: &PlaycallFeatures, sim: &GameSim) -> bool {
        if fg_distance(sim.expect_downtogo().yards_to_goal.0) > 70 {
            // never attempt a field goal from beyond 70 yards
            return false;
        }
        let coefs = PlaycallModel::fg_attempt_coefs();
        PlaycallModel::sample_event(f, coefs)
    }

    fn is_punt(f: &PlaycallFeatures) -> bool {
        let coefs = PlaycallModel::punt_coefs();
        PlaycallModel::sample_event(f, coefs)
    }

    fn is_qb_spike(f: &PlaycallFeatures, sim: &GameSim) -> bool {
        if !sim.game_state.clock.running {
            // no reason to spike if clock is running
            return false;
        }
        let coefs = PlaycallModel::qb_spike_coefs();
        PlaycallModel::sample_event(f, coefs)
    }

    fn is_qb_kneel(f: &PlaycallFeatures, sim: &GameSim) -> bool {
        if sim.game_state.clock.half_minutes_remaining() > 2.0 {
            return false;
        }
        if sim.game_state.play.yards_for_safety() >= -5 {
            // if losing 5+ yards would mean a safety, never kneel
            return false;
        }
        let coefs = PlaycallModel::qb_kneel_coefs();
        PlaycallModel::sample_event(f, coefs)
    }

    pub fn is_qb_dropback(f: &PlaycallFeatures) -> bool {
        let coefs = PlaycallModel::qb_dropback_coefs();
        let z = PlaycallModel::get_z(f, &coefs);
        random_sigmoid(z)
    }

    fn sample_event(f: &PlaycallFeatures, coefs: PlaycallModel) -> bool {
        let z = PlaycallModel::get_z(f, &coefs);
        random_sigmoid(z)
    }

    fn get_z(f: &PlaycallFeatures, c: &PlaycallModel) -> f32 {
        let mut z = c.intercept;
        z += c.clock_running * f.clock_running;
        z += c.inv_half_minutes * f.inv_half_minutes;
        z += c.log_inv_half_minutes * f.log_inv_half_minutes;
        z += c.inv_game_minutes * f.inv_game_minutes;
        z += c.log_inv_game_minutes * f.log_inv_game_minutes;
        z += c.possession_diff * f.possession_diff;
        z += c.fg_possession_diff * f.fg_possession_diff;
        z += c.possdiff_per_minute * f.possdiff_per_minute;
        z += c.fgpossdiff_per_minute * f.fgpossdiff_per_minute;
        z += c.clock_runs_pdpm * f.clock_runs_pdpm;
        z += c.clock_runs_fgpdpm * f.clock_runs_fgpdpm;
        z += c.clock_runs_pdpm2 * f.clock_runs_pdpm2;
        z += c.clock_runs_fgpdpm2 * f.clock_runs_fgpdpm2;
        z += c.off_timeouts_remaining_0 * f.off_timeouts_remaining_0;
        z += c.off_timeouts_remaining_1 * f.off_timeouts_remaining_1;
        z += c.off_timeouts_remaining_2 * f.off_timeouts_remaining_2;
        z += c.off_timeouts_remaining_3 * f.off_timeouts_remaining_3;
        z += c.clock_runs_pdpm_off0to * f.clock_runs_pdpm_off0to;
        z += c.clock_runs_pdpm_off1to * f.clock_runs_pdpm_off1to;
        z += c.clock_runs_pdpm_off2to * f.clock_runs_pdpm_off2to;
        z += c.clock_runs_pdpm_off3to * f.clock_runs_pdpm_off3to;
        z += c.def_timeouts_remaining_0 * f.def_timeouts_remaining_0;
        z += c.def_timeouts_remaining_1 * f.def_timeouts_remaining_1;
        z += c.def_timeouts_remaining_2 * f.def_timeouts_remaining_2;
        z += c.def_timeouts_remaining_3 * f.def_timeouts_remaining_3;
        z += c.clock_runs_pdpm_def0to * f.clock_runs_pdpm_def0to;
        z += c.clock_runs_pdpm_def1to * f.clock_runs_pdpm_def1to;
        z += c.clock_runs_pdpm_def2to * f.clock_runs_pdpm_def2to;
        z += c.clock_runs_pdpm_def3to * f.clock_runs_pdpm_def3to;
        z += c.is_offense_home * f.is_offense_home;
        z += c.offense_log_pass_prob * f.offense_log_pass_prob;
        z += c.defense_log_pass_prob * f.defense_log_pass_prob;
        z += c.off_def_lpp * f.off_def_lpp;
        z += c.off_lpp_rz * f.off_lpp_rz;
        z += c.def_lpp_rz * f.def_lpp_rz;
        z += c.off_def_lpp_rz * f.off_def_lpp_rz;
        z += c.off_lpp_outside_rz * f.off_lpp_outside_rz;
        z += c.off_lpp_inside_rz * f.off_lpp_inside_rz;
        z += c.def_lpp_outside_rz * f.def_lpp_outside_rz;
        z += c.def_lpp_inside_rz * f.def_lpp_inside_rz;
        z += c.off_lpp_pdpm * f.off_lpp_pdpm;
        z += c.def_lpp_pdpm * f.def_lpp_pdpm;
        z += c.off_lpp_rz_pdpm * f.off_lpp_rz_pdpm;
        z += c.def_lpp_rz_pdpm * f.def_lpp_rz_pdpm;
        z += c.down_1 * f.down_1;
        z += c.down_2 * f.down_2;
        z += c.down_3 * f.down_3;
        z += c.down_4 * f.down_4;
        z += c.goal_to_go * f.goal_to_go;
        z += c.z_ydstogo * f.z_ydstogo;
        z += c.ydstogo_pct * f.ydstogo_pct;
        z += c.log_ydstogo_pct * f.log_ydstogo_pct;
        z += c.to_go_1st * f.to_go_1st;
        z += c.to_go_2nd * f.to_go_2nd;
        z += c.to_go_3rd * f.to_go_3rd;
        z += c.to_go_4th * f.to_go_4th;
        z += c.log_to_go_1st * f.log_to_go_1st;
        z += c.log_to_go_2nd * f.log_to_go_2nd;
        z += c.log_to_go_3rd * f.log_to_go_3rd;
        z += c.log_to_go_4th * f.log_to_go_4th;
        z += c.fp_1st * f.fp_1st;
        z += c.fp_2nd * f.fp_2nd;
        z += c.fp_3rd * f.fp_3rd;
        z += c.fp_4th * f.fp_4th;
        z += c.fg_sigmoid * f.fg_sigmoid;
        z += c.punt_sigmoid * f.punt_sigmoid;
        z += c.yardline_pct * f.yardline_pct;
        z += c.yardline_pct_sq * f.yardline_pct_sq;
        z += c.log_yardline_pct * f.log_yardline_pct;
        z += c.yardline_fgsig_4th * f.yardline_fgsig_4th;
        z += c.yardline_puntsig_4th * f.yardline_puntsig_4th;
        z += c.goal_to_go_yardline * f.goal_to_go_yardline;
        z += c.log_goal_to_go_yardline * f.log_goal_to_go_yardline;
        z += c.yards_to_go_yardline * f.yards_to_go_yardline;
        z += c.log_yards_to_go_yardline * f.log_yards_to_go_yardline;
        z += c.yardline_4th * f.yardline_4th;
        z += c.log_yardline_4th * f.log_yardline_4th;
        z += c.yardline_not_4th * f.yardline_not_4th;
        z += c.log_yardline_not_4th * f.log_yardline_not_4th;
        z += c.inside_2m_warning * f.inside_2m_warning;
        z += c.garbage_time_win * f.garbage_time_win;
        z += c.garbage_time_loss * f.garbage_time_loss;
        z += c.offense_penalty_z * f.offense_penalty_z;
        z += c.defense_penalty_z * f.defense_penalty_z;
        z += c.off_def_penalty_z * f.offense_penalty_z * f.defense_penalty_z;

        z
    }
}
