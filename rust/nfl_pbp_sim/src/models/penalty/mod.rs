use crate::util::stats::{random_negbinom, random_poisson, random_sigmoid};

use crate::sim::{play_result::PlayResult, GameSim};
pub mod coef;
use crate::models::features::PlaycallFeatures;

pub struct PenaltyModel {
    intercept: f32,
    is_offense_home: f32,
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
    offense_penalty_z: f32,
    defense_penalty_z: f32,
    off_def_penalty_z: f32,
}

impl PenaltyModel {
    pub fn sim_offensive_penalty(sim: &GameSim) -> PlayResult {
        let is_loss_of_down = PenaltyModel::is_loss_of_down(sim);
        match is_loss_of_down {
            true => PlayResult::OffensivePenaltyNoPlay(5, true),
            false => PlayResult::OffensivePenaltyNoPlay(
                PenaltyModel::sim_offensive_penalty_yards(sim),
                false,
            ),
        }
    }

    pub fn sim_defensive_penalty(sim: &GameSim) -> PlayResult {
        let is_auto_first = PenaltyModel::is_auto_first_down(sim);
        match is_auto_first {
            false => PlayResult::DefensivePenaltyNoPlay(5, false, false),
            true => {
                let (yards, ignore_half_distance) = PenaltyModel::sim_defensive_penalty_yards(sim);
                PlayResult::DefensivePenaltyNoPlay(yards, true, ignore_half_distance)
            }
        }
    }

    fn is_auto_first_down(sim: &GameSim) -> bool {
        let features = PlaycallFeatures::new(sim);
        let coef = PenaltyModel::defensive_automatic_first_coef();
        let z = PenaltyModel::get_z(&features, &coef);
        random_sigmoid(z)
    }

    fn is_loss_of_down(sim: &GameSim) -> bool {
        let features = PlaycallFeatures::new(sim);
        let coef = PenaltyModel::offensive_loss_of_down_coef();
        let z = PenaltyModel::get_z(&features, &coef);
        random_sigmoid(z)
    }

    fn sim_offensive_penalty_yards(sim: &GameSim) -> u8 {
        // only when there is no loss of down (always 5 otherwise)
        // TODO: this isn't correct as intentional grounding has variable yards
        if PenaltyModel::is_offensive_5yard_penalty(sim) {
            return 5;
        }
        if PenaltyModel::is_offensive_15yard_penalty(sim) {
            return 15;
        }
        return 10;
    }

    fn is_offensive_5yard_penalty(sim: &GameSim) -> bool {
        let features = PlaycallFeatures::new(sim);
        let coef = PenaltyModel::offensive_5_yards_coef();
        let z = PenaltyModel::get_z(&features, &coef);
        random_sigmoid(z)
    }

    fn is_offensive_15yard_penalty(sim: &GameSim) -> bool {
        let features = PlaycallFeatures::new(sim);
        let coef = PenaltyModel::offensive_15_yards_coef();
        let z = PenaltyModel::get_z(&features, &coef);
        random_sigmoid(z)
    }

    fn sim_defensive_penalty_yards(sim: &GameSim) -> (u8, bool) {
        // only when there is an automatic first down (always 5 otherwise)
        if PenaltyModel::is_defensive_5yard_penalty(sim) {
            return (5, false);
        }
        if PenaltyModel::is_defensive_15yard_penalty(sim) {
            return (15, false);
        }
        return PenaltyModel::sample_defensive_variable_penalty_yards(sim);
    }

    fn is_defensive_5yard_penalty(sim: &GameSim) -> bool {
        let features = PlaycallFeatures::new(sim);
        let coef = PenaltyModel::defensive_5_yards_coef();
        let z = PenaltyModel::get_z(&features, &coef);
        random_sigmoid(z)
    }

    fn is_defensive_15yard_penalty(sim: &GameSim) -> bool {
        let features = PlaycallFeatures::new(sim);
        let coef = PenaltyModel::defensive_15_yards_coef();
        let z = PenaltyModel::get_z(&features, &coef);
        random_sigmoid(z)
    }

    fn sample_defensive_variable_penalty_yards(sim: &GameSim) -> (u8, bool) {
        let features = PlaycallFeatures::new(sim);
        let to_1_coef = PenaltyModel::defensive_yards_to_1_coef();
        let to_1_z = PenaltyModel::get_z(&features, &to_1_coef).exp();
        match random_sigmoid(to_1_z) {
            true => {
                // modeling DPI in the endzone
                let ytg = sim.expect_downtogo().yards_to_goal.0;
                (ytg - 1, true)
            }
            false => {
                let mean_coef = PenaltyModel::defensive_yards_vary_coef();
                let mean = PenaltyModel::get_z(&features, &mean_coef).exp();

                let var_coef = PenaltyModel::defensive_yards_vary_var_coef();
                let var = PenaltyModel::get_z(&features, &var_coef).exp();

                let yards_minus_1 = match var > mean {
                    // NOTE: downstream, we slim this down based on yards to goal/safety per NFL's
                    // "half distance to goal" rules, so no need to truncate here.
                    true => random_negbinom(mean, var) as u8,
                    false => random_poisson(mean) as u8,
                };
                (1 + yards_minus_1, false)
            }
        }
    }

    fn get_z(f: &PlaycallFeatures, coef: &PenaltyModel) -> f32 {
        let mut z = coef.intercept;
        z += coef.clock_running * f.clock_running;
        z += coef.inv_half_minutes * f.inv_half_minutes;
        z += coef.log_inv_half_minutes * f.log_inv_half_minutes;
        z += coef.inv_game_minutes * f.inv_game_minutes;
        z += coef.log_inv_game_minutes * f.log_inv_game_minutes;
        z += coef.possession_diff * f.possession_diff;
        z += coef.fg_possession_diff * f.fg_possession_diff;
        z += coef.possdiff_per_minute * f.possdiff_per_minute;
        z += coef.fgpossdiff_per_minute * f.fgpossdiff_per_minute;
        z += coef.clock_runs_pdpm * f.clock_runs_pdpm;
        z += coef.clock_runs_fgpdpm * f.clock_runs_fgpdpm;
        z += coef.clock_runs_pdpm2 * f.clock_runs_pdpm2;
        z += coef.clock_runs_fgpdpm2 * f.clock_runs_fgpdpm2;
        z += coef.off_timeouts_remaining_0 * f.off_timeouts_remaining_0;
        z += coef.off_timeouts_remaining_1 * f.off_timeouts_remaining_1;
        z += coef.off_timeouts_remaining_2 * f.off_timeouts_remaining_2;
        z += coef.off_timeouts_remaining_3 * f.off_timeouts_remaining_3;
        z += coef.clock_runs_pdpm_off0to * f.clock_runs_pdpm_off0to;
        z += coef.clock_runs_pdpm_off1to * f.clock_runs_pdpm_off1to;
        z += coef.clock_runs_pdpm_off2to * f.clock_runs_pdpm_off2to;
        z += coef.clock_runs_pdpm_off3to * f.clock_runs_pdpm_off3to;
        z += coef.def_timeouts_remaining_0 * f.def_timeouts_remaining_0;
        z += coef.def_timeouts_remaining_1 * f.def_timeouts_remaining_1;
        z += coef.def_timeouts_remaining_2 * f.def_timeouts_remaining_2;
        z += coef.def_timeouts_remaining_3 * f.def_timeouts_remaining_3;
        z += coef.clock_runs_pdpm_def0to * f.clock_runs_pdpm_def0to;
        z += coef.clock_runs_pdpm_def1to * f.clock_runs_pdpm_def1to;
        z += coef.clock_runs_pdpm_def2to * f.clock_runs_pdpm_def2to;
        z += coef.clock_runs_pdpm_def3to * f.clock_runs_pdpm_def3to;
        z += coef.is_offense_home * f.is_offense_home;
        z += coef.down_1 * f.down_1;
        z += coef.down_2 * f.down_2;
        z += coef.down_3 * f.down_3;
        z += coef.down_4 * f.down_4;
        z += coef.goal_to_go * f.goal_to_go;
        z += coef.z_ydstogo * f.z_ydstogo;
        z += coef.ydstogo_pct * f.ydstogo_pct;
        z += coef.log_ydstogo_pct * f.log_ydstogo_pct;
        z += coef.to_go_1st * f.to_go_1st;
        z += coef.to_go_2nd * f.to_go_2nd;
        z += coef.to_go_3rd * f.to_go_3rd;
        z += coef.to_go_4th * f.to_go_4th;
        z += coef.log_to_go_1st * f.log_to_go_1st;
        z += coef.log_to_go_2nd * f.log_to_go_2nd;
        z += coef.log_to_go_3rd * f.log_to_go_3rd;
        z += coef.log_to_go_4th * f.log_to_go_4th;
        z += coef.fp_1st * f.fp_1st;
        z += coef.fp_2nd * f.fp_2nd;
        z += coef.fp_3rd * f.fp_3rd;
        z += coef.fp_4th * f.fp_4th;
        z += coef.fg_sigmoid * f.fg_sigmoid;
        z += coef.punt_sigmoid * f.punt_sigmoid;
        z += coef.yardline_pct * f.yardline_pct;
        z += coef.yardline_pct_sq * f.yardline_pct_sq;
        z += coef.log_yardline_pct * f.log_yardline_pct;
        z += coef.yardline_fgsig_4th * f.yardline_fgsig_4th;
        z += coef.yardline_puntsig_4th * f.yardline_puntsig_4th;
        z += coef.goal_to_go_yardline * f.goal_to_go_yardline;
        z += coef.log_goal_to_go_yardline * f.log_goal_to_go_yardline;
        z += coef.yards_to_go_yardline * f.yards_to_go_yardline;
        z += coef.log_yards_to_go_yardline * f.log_yards_to_go_yardline;
        z += coef.yardline_4th * f.yardline_4th;
        z += coef.log_yardline_4th * f.log_yardline_4th;
        z += coef.yardline_not_4th * f.yardline_not_4th;
        z += coef.log_yardline_not_4th * f.log_yardline_not_4th;
        z += coef.inside_2m_warning * f.inside_2m_warning;
        z += coef.garbage_time_win * f.garbage_time_win;
        z += coef.garbage_time_loss * f.garbage_time_loss;
        z += coef.offense_log_pass_prob * f.offense_log_pass_prob;
        z += coef.defense_log_pass_prob * f.defense_log_pass_prob;
        z += coef.off_def_lpp * f.off_def_lpp;
        z += coef.off_lpp_rz * f.off_lpp_rz;
        z += coef.def_lpp_rz * f.def_lpp_rz;
        z += coef.off_def_lpp_rz * f.off_def_lpp_rz;
        z += coef.off_lpp_outside_rz * f.off_lpp_outside_rz;
        z += coef.off_lpp_inside_rz * f.off_lpp_inside_rz;
        z += coef.def_lpp_outside_rz * f.def_lpp_outside_rz;
        z += coef.def_lpp_inside_rz * f.def_lpp_inside_rz;
        z += coef.off_lpp_pdpm * f.off_lpp_pdpm;
        z += coef.def_lpp_pdpm * f.def_lpp_pdpm;
        z += coef.off_lpp_rz_pdpm * f.off_lpp_rz_pdpm;
        z += coef.def_lpp_rz_pdpm * f.def_lpp_rz_pdpm;
        z += coef.offense_penalty_z * f.offense_penalty_z;
        z += coef.defense_penalty_z * f.defense_penalty_z;
        z += coef.off_def_penalty_z * f.offense_penalty_z * f.defense_penalty_z;
        z
    }
}
