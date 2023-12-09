use crate::util::stats::{random_sigmoid, truncated_poisson};

use crate::state::yards_to_goal::YardsToGoal;
use crate::{
    models::{
        features::{PlaycallFeatures, EPSILON},
        rushing::RushingModel,
    },
    sim::{
        play_result::{DropbackOutcome, RunResult, SackOutcome, TurnoverOutcome},
        GameSim,
    },
    start::HomeAway,
};

use crate::models::post_pass_penalty::PostPassPenaltyModel;
use crate::models::{air_yards::AirYardsModel, targets::TargetModel};

pub mod coef;

const MAX_SACK_YARDS_LOST: u8 = 20;

pub const MEAN_AIR_YARDS: f32 = 8.0;
pub const EPSILON_AIR_YARDS: f32 = 0.01;

pub struct DropbackModel {
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
    defense_completion_z: f32,
    defense_interception_z: f32,
    def_comp_scramble: f32,
    def_int_scramble: f32,
    olpz_qbps: f32,
    dlpz_qbps: f32,
    olpz_scramble: f32,
    dlpz_scramble: f32,
}

impl DropbackModel {
    pub fn simulate_dropback(sim: &GameSim) -> DropbackOutcome {
        let features = PlaycallFeatures::new(sim);
        let dtg = sim.game_state.play.expect_downtogo();

        if DropbackModel::is_sack(&features) {
            return DropbackOutcome::Sack(DropbackModel::simulate_sack(
                &features,
                dtg.yards_to_goal,
            ));
        }
        if DropbackModel::is_scramble(&features) {
            let offense = match dtg.possession {
                HomeAway::Home => &sim.game_params.home,
                HomeAway::Away => &sim.game_params.away,
            };
            let qb = &offense.quarterback();
            let outcome = RushingModel::simulate_scramble(sim);
            let dbo = DropbackOutcome::QbScramble(RunResult {
                carrier_id: qb.player_id.clone(),
                outcome: outcome.clone(),
                penalty: None,
            });
            let penalty = PostPassPenaltyModel::sample_post_pass_penalty(sim, &dbo);
            return DropbackOutcome::QbScramble(RunResult {
                carrier_id: qb.player_id.clone(),
                outcome,
                penalty,
            });
        }

        if DropbackModel::is_throwaway(&features) {
            return DropbackOutcome::Throwaway;
        }

        let air_yards = AirYardsModel::simulate_air_yards(sim);
        DropbackOutcome::Target(TargetModel::simulate_target(sim, air_yards))
    }

    fn simulate_sack(features: &PlaycallFeatures, yards_to_goal: YardsToGoal) -> SackOutcome {
        if DropbackModel::is_safety(features) {
            return SackOutcome::Safety;
        }

        if DropbackModel::is_fumble_lost(features) {
            return SackOutcome::FumbleLost(
                match DropbackModel::is_fumble_lost_td_return(features) {
                    true => TurnoverOutcome::Touchdown,
                    false => {
                        let yards_lost =
                            DropbackModel::sample_yards_lost(features, yards_to_goal.0);
                        TurnoverOutcome::YardsToGoal((yards_to_goal + yards_lost).flip())
                    }
                },
            );
        }
        let yards_lost = DropbackModel::sample_yards_lost(features, yards_to_goal.0);
        SackOutcome::YardsLost(yards_lost)
    }

    fn is_sack(features: &PlaycallFeatures) -> bool {
        let coef = DropbackModel::prob_sack_coef();
        let z = DropbackModel::get_z(features, &coef);
        random_sigmoid(z)
    }

    fn is_scramble(features: &PlaycallFeatures) -> bool {
        let coef = DropbackModel::prob_scramble_coef();
        let z = DropbackModel::get_z(features, &coef);
        random_sigmoid(z)
    }

    fn is_throwaway(features: &PlaycallFeatures) -> bool {
        let coef = DropbackModel::prob_throwaway_coef();
        let z = DropbackModel::get_z(features, &coef);
        random_sigmoid(z)
    }

    fn is_safety(features: &PlaycallFeatures) -> bool {
        let coef = DropbackModel::prob_sack_safety_coef();
        let z = DropbackModel::get_z(features, &coef);
        random_sigmoid(z)
    }

    fn is_fumble_lost(features: &PlaycallFeatures) -> bool {
        let coef = DropbackModel::prob_sack_fumble_lost_coef();
        let z = DropbackModel::get_z(features, &coef);
        random_sigmoid(z)
    }

    fn is_fumble_lost_td_return(features: &PlaycallFeatures) -> bool {
        let coef = DropbackModel::prob_sack_fumble_lost_td_coef();
        let z = DropbackModel::get_z(features, &coef);
        random_sigmoid(z)
    }

    fn sample_yards_lost(features: &PlaycallFeatures, yards_to_goal: u8) -> u8 {
        let coef = DropbackModel::sack_yards_lost_coef();
        let lambda = DropbackModel::get_z(features, &coef)
            .exp()
            .min(MAX_SACK_YARDS_LOST as f32);
        let max_loss = (100 - yards_to_goal).min(MAX_SACK_YARDS_LOST + 1);
        truncated_poisson(lambda, max_loss)
    }

    fn get_z(features: &PlaycallFeatures, coef: &DropbackModel) -> f32 {
        let log_qb_scramble = (features.qb_scramble_rate + EPSILON).ln();
        let log_qbps = (features.qb_prob_sack_given_hit + EPSILON).ln();

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
        z += coef.log_qb_scramble * log_qb_scramble;
        z += coef.qb_prob_sack_given_hit * features.qb_prob_sack_given_hit;
        z += coef.log_qbps * log_qbps;
        z += coef.offense_pass_rush_z * features.offense_pass_rush_z;
        z += coef.defense_pass_rush_z * features.defense_pass_rush_z;
        z += coef.off_def_pass_rush_z * features.offense_pass_rush_z * features.defense_pass_rush_z;
        z += coef.defense_completion_z * features.defense_completion_z;
        z += coef.defense_interception_z * features.defense_interception_z;
        z += coef.def_comp_scramble * features.defense_completion_z * log_qb_scramble;
        z += coef.def_int_scramble * features.defense_interception_z * log_qb_scramble;
        z += coef.olpz_qbps * features.offense_pass_rush_z * log_qbps;
        z += coef.dlpz_qbps * features.defense_pass_rush_z * log_qbps;
        z += coef.olpz_scramble * features.offense_pass_rush_z * log_qb_scramble;
        z += coef.dlpz_scramble * features.defense_pass_rush_z * log_qb_scramble;
        z
    }
}
