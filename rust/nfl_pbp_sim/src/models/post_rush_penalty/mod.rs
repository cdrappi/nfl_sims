use crate::{
    sim::{
        play_result::{
            DefensivePenaltyOutcome, OffensivePenaltyOutcome, PenaltyType, RushingOutcome,
        },
        GameSim,
    },
    util::stats::random_sigmoid,
};

use crate::models::features::PlaycallFeatures;
pub mod coef;

pub struct PostRushPenaltyModel {
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
    yardline_fgsig_4th: f32,
    yardline_puntsig_4th: f32,
    yardline_pct: f32,
    yardline_pct_sq: f32,
    log_yardline_pct: f32,
    fg_sigmoid: f32,
    punt_sigmoid: f32,
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
    rushing_yards_div10: f32,
    rushing_yards_div10_sq: f32,
}

impl PostRushPenaltyModel {
    pub fn sample_post_rush_penalty(
        sim: &GameSim,
        outcome: &RushingOutcome,
    ) -> Option<PenaltyType> {
        if !outcome.can_have_post_play_penalty() {
            return None;
        }

        let rushing_yards = outcome.yards_gained(sim.game_state.play.yards_for_touchdown() as u8);
        if PostRushPenaltyModel::is_off_postrush_penalty(sim, rushing_yards) {
            let penalty_yards = PostRushPenaltyModel::sim_off_penaty_yards(sim, rushing_yards);
            return Some(PenaltyType::Offensive(OffensivePenaltyOutcome {
                yards: penalty_yards,
                loss_of_down: false,
            }));
        }

        if PostRushPenaltyModel::is_def_postrush_penalty(sim, rushing_yards) {
            let penalty_yards = PostRushPenaltyModel::sim_def_penalty_yards(sim, rushing_yards);
            return Some(PenaltyType::Defensive(DefensivePenaltyOutcome {
                yards: penalty_yards,
                automatic_first: true,
                ignore_half_distance: false,
            }));
        }

        None
    }

    fn is_off_postrush_penalty(sim: &GameSim, rushing_yards: f32) -> bool {
        let coef = PostRushPenaltyModel::is_postrush_off_penalty_coef();
        let f = PostRushPenaltyModel::get_features(sim, rushing_yards);
        let z = PostRushPenaltyModel::get_z(&coef, &f);
        random_sigmoid(z)
    }

    fn is_def_postrush_penalty(sim: &GameSim, rushing_yards: f32) -> bool {
        let coef = PostRushPenaltyModel::is_postrush_def_penalty_coef();
        let f = PostRushPenaltyModel::get_features(sim, rushing_yards);
        let z = PostRushPenaltyModel::get_z(&coef, &f);
        random_sigmoid(z)
    }

    fn sim_def_penalty_yards(sim: &GameSim, rushing_yards: f32) -> u8 {
        // 5 or 15
        let is_5_coef = PostRushPenaltyModel::is_5_postrush_def_penalty_yards_coef();
        let f = PostRushPenaltyModel::get_features(sim, rushing_yards);
        let z = PostRushPenaltyModel::get_z(&is_5_coef, &f);
        match random_sigmoid(z) {
            true => 5,
            false => 15,
        }
    }

    fn sim_off_penaty_yards(_sim: &GameSim, _rushing_yards: f32) -> u8 {
        // 5, 10 or 15
        let f = PostRushPenaltyModel::get_features(_sim, _rushing_yards);
        let is_5_coef = PostRushPenaltyModel::is_5_postrush_off_penalty_yards_coef();
        if random_sigmoid(PostRushPenaltyModel::get_z(&is_5_coef, &f)) {
            return 5;
        }
        let is_10_coef = PostRushPenaltyModel::is_10_postrush_off_penalty_yards_coef();
        if random_sigmoid(PostRushPenaltyModel::get_z(&is_10_coef, &f)) {
            return 10;
        }
        return 15;
    }

    fn get_z(c: &PostRushPenaltyModel, f: &PostRushPenaltyModel) -> f32 {
        let mut z = c.intercept;

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
        z += c.yardline_fgsig_4th * f.yardline_fgsig_4th;
        z += c.yardline_puntsig_4th * f.yardline_puntsig_4th;
        z += c.yardline_pct * f.yardline_pct;
        z += c.yardline_pct_sq * f.yardline_pct_sq;
        z += c.log_yardline_pct * f.log_yardline_pct;
        z += c.fg_sigmoid * f.fg_sigmoid;
        z += c.punt_sigmoid * f.punt_sigmoid;
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
        z += c.offense_penalty_z * f.offense_penalty_z;
        z += c.defense_penalty_z * f.defense_penalty_z;
        z += c.off_def_penalty_z * f.off_def_penalty_z;
        z += c.rushing_yards_div10 * f.rushing_yards_div10;
        z += c.rushing_yards_div10_sq * f.rushing_yards_div10_sq;

        z
    }

    fn get_features(sim: &GameSim, rushing_yards: f32) -> PostRushPenaltyModel {
        let f = PlaycallFeatures::new(sim);
        let rushing_yards_div10 = rushing_yards as f32 / 10.0;

        PostRushPenaltyModel {
            intercept: 1.0,
            is_offense_home: f.is_offense_home,
            offense_log_pass_prob: f.offense_log_pass_prob,
            defense_log_pass_prob: f.defense_log_pass_prob,
            off_def_lpp: f.off_def_lpp,
            off_lpp_rz: f.off_lpp_rz,
            def_lpp_rz: f.def_lpp_rz,
            off_def_lpp_rz: f.off_def_lpp_rz,
            off_lpp_outside_rz: f.off_lpp_outside_rz,
            off_lpp_inside_rz: f.off_lpp_inside_rz,
            def_lpp_outside_rz: f.def_lpp_outside_rz,
            def_lpp_inside_rz: f.def_lpp_inside_rz,
            off_lpp_pdpm: f.off_lpp_pdpm,
            def_lpp_pdpm: f.def_lpp_pdpm,
            off_lpp_rz_pdpm: f.off_lpp_rz_pdpm,
            def_lpp_rz_pdpm: f.def_lpp_rz_pdpm,
            down_1: f.down_1,
            down_2: f.down_2,
            down_3: f.down_3,
            down_4: f.down_1,
            goal_to_go: f.goal_to_go,
            z_ydstogo: f.z_ydstogo,
            ydstogo_pct: f.ydstogo_pct,
            log_ydstogo_pct: f.log_ydstogo_pct,
            to_go_1st: f.to_go_1st,
            to_go_2nd: f.to_go_2nd,
            to_go_3rd: f.to_go_3rd,
            to_go_4th: f.to_go_4th,
            log_to_go_1st: f.log_to_go_1st,
            log_to_go_2nd: f.log_to_go_2nd,
            log_to_go_3rd: f.log_to_go_3rd,
            log_to_go_4th: f.log_to_go_4th,
            fp_1st: f.fp_1st,
            fp_2nd: f.fp_2nd,
            fp_3rd: f.fp_3rd,
            fp_4th: f.fp_4th,
            yardline_fgsig_4th: f.yardline_fgsig_4th,
            yardline_puntsig_4th: f.yardline_puntsig_4th,
            yardline_pct: f.yardline_pct,
            yardline_pct_sq: f.yardline_pct_sq,
            log_yardline_pct: f.log_yardline_pct,
            fg_sigmoid: f.fg_sigmoid,
            punt_sigmoid: f.punt_sigmoid,
            goal_to_go_yardline: f.goal_to_go_yardline,
            log_goal_to_go_yardline: f.log_goal_to_go_yardline,
            yards_to_go_yardline: f.yards_to_go_yardline,
            log_yards_to_go_yardline: f.log_yards_to_go_yardline,
            yardline_4th: f.yardline_4th,
            log_yardline_4th: f.log_yardline_4th,
            yardline_not_4th: f.yardline_not_4th,
            log_yardline_not_4th: f.log_yardline_not_4th,
            inside_2m_warning: f.inside_2m_warning,
            garbage_time_win: f.garbage_time_win,
            garbage_time_loss: f.garbage_time_loss,
            clock_running: f.clock_running,
            inv_half_minutes: f.inv_half_minutes,
            log_inv_half_minutes: f.log_inv_half_minutes,
            inv_game_minutes: f.inv_game_minutes,
            log_inv_game_minutes: f.log_inv_game_minutes,
            possession_diff: f.possession_diff,
            fg_possession_diff: f.fg_possession_diff,
            possdiff_per_minute: f.possdiff_per_minute,
            fgpossdiff_per_minute: f.fgpossdiff_per_minute,
            clock_runs_pdpm: f.clock_runs_pdpm,
            clock_runs_fgpdpm: f.clock_runs_fgpdpm,
            clock_runs_pdpm2: f.clock_runs_pdpm2,
            clock_runs_fgpdpm2: f.clock_runs_fgpdpm2,
            off_timeouts_remaining_0: f.off_timeouts_remaining_0,
            off_timeouts_remaining_1: f.off_timeouts_remaining_1,
            off_timeouts_remaining_2: f.off_timeouts_remaining_2,
            off_timeouts_remaining_3: f.off_timeouts_remaining_3,
            clock_runs_pdpm_off0to: f.clock_runs_pdpm_off0to,
            clock_runs_pdpm_off1to: f.clock_runs_pdpm_off1to,
            clock_runs_pdpm_off2to: f.clock_runs_pdpm_off2to,
            clock_runs_pdpm_off3to: f.clock_runs_pdpm_off3to,
            def_timeouts_remaining_0: f.def_timeouts_remaining_0,
            def_timeouts_remaining_1: f.def_timeouts_remaining_1,
            def_timeouts_remaining_2: f.def_timeouts_remaining_2,
            def_timeouts_remaining_3: f.def_timeouts_remaining_3,
            clock_runs_pdpm_def0to: f.clock_runs_pdpm_def0to,
            clock_runs_pdpm_def1to: f.clock_runs_pdpm_def1to,
            clock_runs_pdpm_def2to: f.clock_runs_pdpm_def2to,
            clock_runs_pdpm_def3to: f.clock_runs_pdpm_def3to,
            offense_penalty_z: f.offense_penalty_z,
            defense_penalty_z: f.defense_penalty_z,
            off_def_penalty_z: f.offense_penalty_z * f.defense_penalty_z,
            rushing_yards_div10,
            rushing_yards_div10_sq: rushing_yards_div10.powi(2),
        }
    }
}
