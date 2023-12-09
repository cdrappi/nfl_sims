pub mod coef;

use crate::game_loop::field_goals::ENDZONE_LENGTH;
use crate::util::stats::{random_sigmoid, truncated_negbinom, truncated_poisson};

use crate::models::dropback::{EPSILON_AIR_YARDS, MEAN_AIR_YARDS};
use crate::{
    models::features::{PlaycallFeatures, EPSILON},
    sim::GameSim,
    start::HomeAway,
};

// most a pass can travel is -15 yards
const MIN_AIR_YARDS: u8 = 15;
// no one can throw the ball further than this...
const MAX_AIR_YARDS: u8 = 70;

#[derive(Debug)]
pub struct AirYardsModel {
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
    qb_scramble_rate: f32,
    log_qb_scramble: f32,
    qb_prob_sack_given_hit: f32,
    log_qbps: f32,
    offense_pass_rush_z: f32,
    defense_pass_rush_z: f32,
    off_def_pass_rush_z: f32,
    olpz_qbps: f32,
    dlpz_qbps: f32,
    olpz_scramble: f32,
    dlpz_scramble: f32,
    defense_completion_z: f32,
    defense_interception_z: f32,
    def_comp_scramble: f32,
    def_int_scramble: f32,
    qb_ay_oe: f32,
    qb_ay_std: f32,
    log_qb_mean_ay: f32,
    log_qb_std_ay: f32,
}

impl AirYardsModel {
    pub fn simulate_air_yards(sim: &GameSim) -> i8 {
        let features: AirYardsModel = AirYardsModel::features(sim);
        let pos_ay = AirYardsModel::sim_is_pos_air_yards(&features);

        let air_yards = match pos_ay {
            true => AirYardsModel::sim_pos_air_yards(
                &features,
                sim.game_state.play.yards_for_touchdown() as u8,
            ),
            false => {
                AirYardsModel::sim_neg_air_yards(&features, sim.game_state.play.yards_for_safety())
            }
        };

        // if air_yards == -128 {
        //     log::info!("AIR YARDS: {:?}", pos_ay);
        // }
        air_yards
    }

    fn sim_is_pos_air_yards(features: &AirYardsModel) -> bool {
        let coefs = AirYardsModel::is_pos_air_yards_coef();
        let z = AirYardsModel::get_z(features, &coefs);
        random_sigmoid(z)
    }

    fn sim_neg_air_yards(features: &AirYardsModel, yards_for_safety: i8) -> i8 {
        let coef = AirYardsModel::neg_air_yards_coef();
        let mean = AirYardsModel::get_z(features, &coef)
            .exp()
            .min(MIN_AIR_YARDS as f32);

        // don't allow throwing 1+ yard into own end zone
        let min_ay_flipped = ((-1 * yards_for_safety) as u8 + 1).min(MIN_AIR_YARDS);
        // log::info!(
        //     "NEG AIR YARDS: mean AY = {}, min = {}",
        //     mean,
        //     min_ay_flipped
        // );
        -1 * (truncated_poisson(mean, min_ay_flipped) as i8)
    }

    fn sim_pos_air_yards(features: &AirYardsModel, yards_for_touchdown: u8) -> i8 {
        if yards_for_touchdown == 0 {
            panic!("yards for touchdown is 0 -- should be in end zone");
        }
        let mean_coef = AirYardsModel::pos_air_yards_coef();
        let var_coef = AirYardsModel::pos_air_yards_var_coef();

        let max_ay = (yards_for_touchdown + ENDZONE_LENGTH - 2).min(MAX_AIR_YARDS - 1);
        let mean = AirYardsModel::get_z(&features, &mean_coef)
            .exp()
            .min(max_ay as f32);
        let var = AirYardsModel::get_z(&features, &var_coef).exp();

        // log::info!(
        //     "POS AIR YARDS + mean = {}, var = {}, max_ay: {}",
        //     mean,
        //     var,
        //     max_ay
        // );
        let sampled_ay = 1 + match var > mean {
            true => truncated_negbinom(mean, var, max_ay) as i8,
            false => truncated_poisson(mean, max_ay) as i8,
        };
        let simmed_ay = sampled_ay.min(yards_for_touchdown as i8);
        // if simmed_ay <= 0 {
        //     panic!(
        //         "sampled {} ay {} {} {} {} {}",
        //         yards_for_touchdown, yards_for_touchdown as i8, sampled_ay, max_ay, mean, var
        //     )
        // }
        simmed_ay
    }

    fn get_z(features: &AirYardsModel, coef: &AirYardsModel) -> f32 {
        let mut z = coef.intercept;
        z += coef.clock_running * features.clock_running;
        z += coef.inv_half_minutes * features.inv_half_minutes;
        z += coef.log_inv_half_minutes * features.log_inv_half_minutes;
        z += coef.inv_game_minutes * features.inv_game_minutes;
        z += coef.log_inv_game_minutes * features.log_inv_game_minutes;
        z += coef.possession_diff * features.possession_diff;
        z += coef.fg_possession_diff * features.fg_possession_diff;
        z += coef.possdiff_per_minute * features.possdiff_per_minute;
        z += coef.fgpossdiff_per_minute * features.fgpossdiff_per_minute;
        z += coef.clock_runs_pdpm * features.clock_runs_pdpm;
        z += coef.clock_runs_fgpdpm * features.clock_runs_fgpdpm;
        z += coef.clock_runs_pdpm2 * features.clock_runs_pdpm2;
        z += coef.clock_runs_fgpdpm2 * features.clock_runs_fgpdpm2;
        z += coef.is_offense_home * features.is_offense_home;
        z += coef.offense_log_pass_prob * features.offense_log_pass_prob;
        z += coef.defense_log_pass_prob * features.defense_log_pass_prob;
        z += coef.off_def_lpp * features.off_def_lpp;
        z += coef.off_lpp_rz * features.off_lpp_rz;
        z += coef.def_lpp_rz * features.def_lpp_rz;
        z += coef.off_def_lpp_rz * features.off_def_lpp_rz;
        z += coef.off_lpp_outside_rz * features.off_lpp_outside_rz;
        z += coef.off_lpp_inside_rz * features.off_lpp_inside_rz;
        z += coef.def_lpp_outside_rz * features.def_lpp_outside_rz;
        z += coef.def_lpp_inside_rz * features.def_lpp_inside_rz;
        z += coef.off_lpp_pdpm * features.off_lpp_pdpm;
        z += coef.def_lpp_pdpm * features.def_lpp_pdpm;
        z += coef.off_lpp_rz_pdpm * features.off_lpp_rz_pdpm;
        z += coef.def_lpp_rz_pdpm * features.def_lpp_rz_pdpm;
        z += coef.down_1 * features.down_1;
        z += coef.down_2 * features.down_2;
        z += coef.down_3 * features.down_3;
        z += coef.down_4 * features.down_4;
        z += coef.goal_to_go * features.goal_to_go;
        z += coef.z_ydstogo * features.z_ydstogo;
        z += coef.ydstogo_pct * features.ydstogo_pct;
        z += coef.log_ydstogo_pct * features.log_ydstogo_pct;
        z += coef.to_go_1st * features.to_go_1st;
        z += coef.to_go_2nd * features.to_go_2nd;
        z += coef.to_go_3rd * features.to_go_3rd;
        z += coef.to_go_4th * features.to_go_4th;
        z += coef.log_to_go_1st * features.log_to_go_1st;
        z += coef.log_to_go_2nd * features.log_to_go_2nd;
        z += coef.log_to_go_3rd * features.log_to_go_3rd;
        z += coef.log_to_go_4th * features.log_to_go_4th;
        z += coef.fp_1st * features.fp_1st;
        z += coef.fp_2nd * features.fp_2nd;
        z += coef.fp_3rd * features.fp_3rd;
        z += coef.fp_4th * features.fp_4th;
        z += coef.fg_sigmoid * features.fg_sigmoid;
        z += coef.punt_sigmoid * features.punt_sigmoid;
        z += coef.yardline_pct * features.yardline_pct;
        z += coef.yardline_pct_sq * features.yardline_pct_sq;
        z += coef.log_yardline_pct * features.log_yardline_pct;
        z += coef.yardline_fgsig_4th * features.yardline_fgsig_4th;
        z += coef.yardline_puntsig_4th * features.yardline_puntsig_4th;
        z += coef.goal_to_go_yardline * features.goal_to_go_yardline;
        z += coef.log_goal_to_go_yardline * features.log_goal_to_go_yardline;
        z += coef.yards_to_go_yardline * features.yards_to_go_yardline;
        z += coef.log_yards_to_go_yardline * features.log_yards_to_go_yardline;
        z += coef.yardline_4th * features.yardline_4th;
        z += coef.log_yardline_4th * features.log_yardline_4th;
        z += coef.yardline_not_4th * features.yardline_not_4th;
        z += coef.log_yardline_not_4th * features.log_yardline_not_4th;
        z += coef.inside_2m_warning * features.inside_2m_warning;
        z += coef.garbage_time_win * features.garbage_time_win;
        z += coef.garbage_time_loss * features.garbage_time_loss;
        z += coef.qb_scramble_rate * features.qb_scramble_rate;
        z += coef.log_qb_scramble * features.log_qb_scramble;
        z += coef.qb_prob_sack_given_hit * features.qb_prob_sack_given_hit;
        z += coef.log_qbps * features.log_qbps;
        z += coef.offense_pass_rush_z * features.offense_pass_rush_z;
        z += coef.defense_pass_rush_z * features.defense_pass_rush_z;
        z += coef.off_def_pass_rush_z * features.off_def_pass_rush_z;
        z += coef.olpz_qbps * features.olpz_qbps;
        z += coef.dlpz_qbps * features.dlpz_qbps;
        z += coef.olpz_scramble * features.olpz_scramble;
        z += coef.dlpz_scramble * features.dlpz_scramble;
        z += coef.defense_completion_z * features.defense_completion_z;
        z += coef.defense_interception_z * features.defense_interception_z;
        z += coef.def_comp_scramble * features.def_comp_scramble;
        z += coef.def_int_scramble * features.def_int_scramble;
        z += coef.qb_ay_oe * features.qb_ay_oe;
        z += coef.qb_ay_std * features.qb_ay_std;
        z += coef.log_qb_mean_ay * features.log_qb_mean_ay;
        z += coef.log_qb_std_ay * features.log_qb_std_ay;
        z
    }

    fn features(sim: &GameSim) -> AirYardsModel {
        let f = PlaycallFeatures::new(sim);

        let dtg = sim.game_state.play.expect_downtogo();

        let (offense, defense) = match dtg.possession {
            HomeAway::Home => (&sim.game_params.home, &sim.game_params.away),
            HomeAway::Away => (&sim.game_params.away, &sim.game_params.home),
        };

        let qb = &offense.quarterback();

        let log_qb_scramble = (f.qb_scramble_rate + EPSILON).ln();
        let log_qbps = (f.qb_prob_sack_given_hit + EPSILON).ln();

        AirYardsModel {
            intercept: 1.0,
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
            down_4: f.down_4,
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
            fg_sigmoid: f.fg_sigmoid,
            punt_sigmoid: f.punt_sigmoid,
            yardline_pct: f.yardline_pct,
            yardline_pct_sq: f.yardline_pct_sq,
            log_yardline_pct: f.log_yardline_pct,
            yardline_fgsig_4th: f.yardline_fgsig_4th,
            yardline_puntsig_4th: f.yardline_puntsig_4th,
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
            qb_scramble_rate: f.qb_scramble_rate,
            log_qb_scramble,
            qb_prob_sack_given_hit: f.qb_prob_sack_given_hit,
            log_qbps: log_qbps,
            offense_pass_rush_z: f.offense_pass_rush_z,
            defense_pass_rush_z: f.defense_pass_rush_z,
            off_def_pass_rush_z: f.offense_pass_rush_z * f.defense_pass_rush_z,
            olpz_qbps: f.offense_pass_rush_z * log_qbps,
            dlpz_qbps: f.defense_pass_rush_z * log_qbps,
            olpz_scramble: f.offense_pass_rush_z * log_qb_scramble,
            dlpz_scramble: f.defense_pass_rush_z * log_qb_scramble,
            defense_completion_z: defense.team.defense_completion_z,
            defense_interception_z: defense.team.defense_interception_z,
            def_comp_scramble: defense.team.defense_completion_z * log_qb_scramble,
            def_int_scramble: defense.team.defense_interception_z * log_qb_scramble,
            qb_ay_oe: qb.ayoe,
            qb_ay_std: qb.ay_std,
            log_qb_mean_ay: (MEAN_AIR_YARDS + qb.ayoe).max(EPSILON_AIR_YARDS).ln(),
            log_qb_std_ay: qb.ay_std.ln(),
        }
    }
}
