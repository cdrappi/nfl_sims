pub mod coef;

use std::collections::HashMap;

use crate::state::yards_to_goal::YardsToGoal;
use crate::util::stats::{random_discrete, random_sigmoid, truncated_negbinom, truncated_poisson};
use crate::{
    models::features::PlaycallFeatures,
    params::RushingParams,
    sim::{
        play_result::{RunResult, RushingOutcome, TurnoverOutcome},
        GameSim,
    },
    start::HomeAway,
    state::down::RushingSituation,
};

use crate::models::post_rush_penalty::PostRushPenaltyModel;

use super::shares::compute_conditional_shares;

const RUSHING_EPSILON: f32 = 0.01;
const YARDS_PER_DESIGNED_RUN: f32 = 4.25;
const YARDS_PER_SCRAMBLE: f32 = 7.35;

pub struct RushingModel {
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
    possdiff_per_minute: f32,
    fgpossdiff_per_minute: f32,
    ol_z: f32,
    dl_z: f32,
    ol_dl_z: f32,
    log_mean_yards: f32,
    log_std_yards: f32,
    yoe_mean: f32,
    yoe_std: f32,
    yoe_var: f32,
    yardline_std: f32,
    yardline_var: f32,
    togo_std: f32,
    togo_var: f32,
    clock_runs_after: f32,
}

impl RushingModel {
    pub fn simulate_scramble(sim: &GameSim) -> RushingOutcome {
        let features = PlaycallFeatures::new(sim);
        let offense = sim.game_state.play.possession();

        let offense_params = match offense {
            HomeAway::Home => &sim.game_params.home,
            HomeAway::Away => &sim.game_params.away,
        };
        let rusher = offense_params.qbs[0].rushing_params();

        let dtg = sim.game_state.play.expect_downtogo();

        if RushingModel::is_scramble_fumble_lost(&features, &rusher) {
            let turnover_outcome = match RushingModel::is_scramble_fl_td(&features, &rusher) {
                true => TurnoverOutcome::Touchdown,
                false => TurnoverOutcome::YardsToGoal(dtg.yards_to_goal.flip()),
            };
            return RushingOutcome::FumbleLost(0, turnover_outcome);
        }

        if RushingModel::is_scramble_touchdown(&features, &rusher, dtg.yards_to_goal) {
            return RushingOutcome::Touchdown;
        }
        // cannot have scrambling safety (is just a sack)
        let clock_runs_after = RushingModel::is_scramble_clock_runs(&features, &rusher);
        let clock_runs_after_f32 = if clock_runs_after { 1.0 } else { 0.0 };
        let yards = RushingModel::sample_scrambling_yards(
            &features,
            &rusher,
            clock_runs_after_f32,
            dtg.yards_to_goal.0,
        );
        // log::info!("scrambling yards = {} from {}", yards, dtg.yards_to_goal);
        RushingOutcome::Yards(yards, !clock_runs_after)
    }

    pub fn sim_designed_run(sim: &GameSim) -> RunResult {
        let carrier_id = RushingModel::sim_rusher(sim);
        let outcome = RushingModel::sim_designed_run_outcome(sim, &carrier_id);
        let penalty = PostRushPenaltyModel::sample_post_rush_penalty(sim, &outcome);
        RunResult {
            carrier_id,
            outcome,
            penalty,
        }
    }

    fn sim_rusher(sim: &GameSim) -> String {
        let probs = RushingModel::rusher_probs(sim);
        match random_discrete(probs.clone()) {
            Ok(rusher_id) => rusher_id,
            Err(_we) => {
                panic!(
                    "Could not sample rusher\nInjuries = {:?}\nProbs: {:?}",
                    sim.offense_params().injuries,
                    &probs
                )
            }
        }
    }

    fn rusher_probs(sim: &GameSim) -> Vec<(String, f32)> {
        let offense = sim.offense_params();

        let mut team_loc_probs = HashMap::new();
        team_loc_probs.insert(
            RushingSituation::OneYardToGo,
            offense.team.prob_1ytg_given_carry,
        );
        team_loc_probs.insert(
            RushingSituation::GreenZone,
            offense.team.prob_gz_given_carry,
        );

        let mut marginal_shares = HashMap::new();
        let mut player_loc_probs = HashMap::new();
        for (pid, player) in offense.skill_players.iter() {
            marginal_shares.insert(pid.clone(), player.ms_carries_live);
            let mut player_rz_probs = HashMap::new();
            player_rz_probs.insert(RushingSituation::OneYardToGo, player.prob_1ytg_given_carry);

            player_rz_probs.insert(RushingSituation::GreenZone, player.prob_gz_given_carry);
            player_loc_probs.insert(pid.clone(), player_rz_probs);
        }

        let cond_shares = compute_conditional_shares(
            marginal_shares,
            player_loc_probs,
            team_loc_probs,
            RushingSituation::Normal,
        );
        let situation = sim.game_state.play.expect_downtogo().rushing_situation();
        cond_shares[&situation].clone()
    }

    fn sim_designed_run_outcome(sim: &GameSim, rusher_id: &String) -> RushingOutcome {
        let features = PlaycallFeatures::new(sim);
        let offense = sim.game_state.play.possession();

        let offense_params = match offense {
            HomeAway::Home => &sim.game_params.home,
            HomeAway::Away => &sim.game_params.away,
        };
        let rusher = offense_params
            .skill_players
            .get(rusher_id)
            .expect(&format!("Could not find {} in skill players", rusher_id))
            .rushing_params();

        let dtg = sim.game_state.play.expect_downtogo();
        if RushingModel::is_designed_run_fumble_lost(&features, &rusher) {
            let turnover_outcome = match RushingModel::is_designed_run_fl_td(&features, &rusher) {
                true => TurnoverOutcome::Touchdown,
                false => TurnoverOutcome::YardsToGoal(dtg.yards_to_goal.flip()),
            };
            return RushingOutcome::FumbleLost(0, turnover_outcome);
        }

        if RushingModel::is_designed_run_touchdown(&features, &rusher) {
            return RushingOutcome::Touchdown;
        }

        if RushingModel::is_designed_run_safety(&features, &rusher) {
            return RushingOutcome::Safety;
        }

        let clock_runs_after = RushingModel::is_designed_run_clock_runs(&features, &rusher);
        let clock_runs_after_f32 = if clock_runs_after { 1.0 } else { 0.0 };
        let yards = RushingModel::sample_designed_run_yards(
            &features,
            &rusher,
            clock_runs_after_f32,
            dtg.yards_to_goal,
        );
        // log::info!("designed run yards = {} from {}", yards, dtg.yards_to_goal);
        RushingOutcome::Yards(yards, !clock_runs_after)
    }

    fn is_designed_run_fumble_lost(features: &PlaycallFeatures, rusher: &RushingParams) -> bool {
        let coefs = RushingModel::designed_run_fumble_lost_coef();
        let z = RushingModel::get_z(&features, rusher, &coefs, YARDS_PER_DESIGNED_RUN, 0.0);
        random_sigmoid(z)
    }

    fn is_designed_run_fl_td(features: &PlaycallFeatures, rusher: &RushingParams) -> bool {
        let coefs = RushingModel::designed_run_prob_fl_td_coef();
        let z = RushingModel::get_z(&features, rusher, &coefs, YARDS_PER_DESIGNED_RUN, 0.0);
        random_sigmoid(z)
    }

    fn is_designed_run_touchdown(features: &PlaycallFeatures, rusher: &RushingParams) -> bool {
        let coefs = RushingModel::designed_run_rush_td_coef();
        let z = RushingModel::get_z(&features, rusher, &coefs, YARDS_PER_DESIGNED_RUN, 0.0);
        // log::info!(
        //     "YTG = {:.0} prob rush td: {:.2}%",
        //     100.0 * features.yardline_pct,
        //     100.0 * sigmoid_prob(z)
        // );
        random_sigmoid(z)
    }

    fn is_designed_run_safety(features: &PlaycallFeatures, rusher: &RushingParams) -> bool {
        let coefs = RushingModel::designed_run_safety_coef();
        let z = RushingModel::get_z(&features, rusher, &coefs, YARDS_PER_DESIGNED_RUN, 0.0);
        random_sigmoid(z)
    }

    fn is_designed_run_clock_runs(features: &PlaycallFeatures, rusher: &RushingParams) -> bool {
        let coefs = RushingModel::designed_run_clock_runs_coef();
        let z = RushingModel::get_z(&features, rusher, &coefs, YARDS_PER_DESIGNED_RUN, 0.0);
        random_sigmoid(z)
    }

    fn sample_designed_run_yards(
        features: &PlaycallFeatures,
        rusher: &RushingParams,
        clock_runs_after: f32,
        yards_to_goal: YardsToGoal,
    ) -> i8 {
        // cannot have positive rush from 1
        let pos_sign = RushingModel::is_designed_run_pos_sign(
            yards_to_goal,
            features,
            rusher,
            clock_runs_after,
        );
        match pos_sign {
            false => RushingModel::sample_designed_run_neg_yards(
                features,
                rusher,
                clock_runs_after,
                yards_to_goal.flip().0,
            ),
            true => RushingModel::sample_designed_run_pos_yards(
                features,
                rusher,
                clock_runs_after,
                yards_to_goal.0,
            ),
        }
    }

    fn is_designed_run_pos_sign(
        yards_to_goal: YardsToGoal,
        features: &PlaycallFeatures,
        rusher: &RushingParams,
        clock_runs_after: f32,
    ) -> bool {
        if let Some(is_pos) = yards_to_goal.forced_yards_sign_pos() {
            return is_pos;
        }
        let coefs = RushingModel::designed_run_is_yards_pos_sign_coef();
        let z = RushingModel::get_z(
            &features,
            rusher,
            &coefs,
            YARDS_PER_SCRAMBLE,
            clock_runs_after,
        );
        // log::info!("prob positive rush: {:.1}%", 100.0 * sigmoid_prob(z));
        random_sigmoid(z)
    }

    fn sample_designed_run_pos_yards(
        features: &PlaycallFeatures,
        rusher: &RushingParams,
        clock_runs_after: f32,
        yards_to_goal: u8,
    ) -> i8 {
        // log::info!("FEATURES \n\n{:?}\n\n", features);
        let mean_coefs = RushingModel::designed_run_pos_yards_coef();
        let var_coefs = RushingModel::designed_run_pos_yards_var_coef();

        let mean = RushingModel::get_z(
            &features,
            rusher,
            &mean_coefs,
            YARDS_PER_SCRAMBLE,
            clock_runs_after,
        )
        .exp();
        let var = RushingModel::get_z(
            &features,
            rusher,
            &var_coefs,
            YARDS_PER_SCRAMBLE,
            clock_runs_after,
        )
        .exp();
        1 + match var > mean {
            true => truncated_negbinom(mean, var, yards_to_goal - 1) as i8,
            false => truncated_poisson(mean, yards_to_goal - 1) as i8,
        }
    }

    fn sample_designed_run_neg_yards(
        features: &PlaycallFeatures,
        rusher: &RushingParams,
        clock_runs_after: f32,
        yards_to_safety: u8,
    ) -> i8 {
        let coefs = RushingModel::designed_run_neg_yards_coef();
        let lambda = RushingModel::get_z(
            &features,
            rusher,
            &coefs,
            YARDS_PER_SCRAMBLE,
            clock_runs_after,
        )
        .exp();
        -1 * truncated_poisson(lambda, yards_to_safety) as i8
    }

    fn is_scramble_touchdown(
        features: &PlaycallFeatures,
        rusher: &RushingParams,
        yards_to_goal: YardsToGoal,
    ) -> bool {
        if yards_to_goal.cannot_move_positive() {
            // if we are scrambling and it can't go forward without being a touchdown,
            // then assume it's a touchdown
            return true;
        }
        let coefs = RushingModel::scrambling_rush_td_coef();
        let z = RushingModel::get_z(&features, rusher, &coefs, YARDS_PER_SCRAMBLE, 0.0);
        random_sigmoid(z)
    }

    fn is_scramble_clock_runs(features: &PlaycallFeatures, rusher: &RushingParams) -> bool {
        let coefs = RushingModel::scrambling_clock_runs_coef();
        let z = RushingModel::get_z(&features, rusher, &coefs, YARDS_PER_SCRAMBLE, 0.0);
        random_sigmoid(z)
    }

    fn sample_scrambling_yards(
        features: &PlaycallFeatures,
        rusher: &RushingParams,
        clock_runs_after: f32,
        yards_to_goal: u8,
    ) -> i8 {
        let mean_coefs = RushingModel::scrambling_pos_yards_coef();
        let var_coefs = RushingModel::scrambling_pos_yards_var_coef();

        let mean = RushingModel::get_z(
            &features,
            rusher,
            &mean_coefs,
            YARDS_PER_SCRAMBLE,
            clock_runs_after,
        )
        .exp();
        let var = RushingModel::get_z(
            features,
            rusher,
            &var_coefs,
            YARDS_PER_SCRAMBLE,
            clock_runs_after,
        )
        .exp();

        // log::info!(
        //     "scrambling negbinom. mean = {:?}, stdev = {:?}",
        //     mean,
        //     var.sqrt()
        // );
        1 + match var > mean {
            true => truncated_negbinom(mean, var, yards_to_goal - 1) as i8,
            false => truncated_poisson(mean, yards_to_goal - 1) as i8,
        }
    }

    fn get_z(
        features: &PlaycallFeatures,
        rusher: &RushingParams,
        coefs: &RushingModel,
        mean_yards: f32,
        clock_runs_after: f32,
    ) -> f32 {
        let mut z = coefs.intercept;

        let log_mean_yards = (mean_yards + rusher.yoe_mean).max(RUSHING_EPSILON).ln();
        let log_std_yards = rusher.yoe_std.max(RUSHING_EPSILON).ln();
        let yoe_var = rusher.yoe_std * rusher.yoe_std;

        z += coefs.clock_running * features.clock_running;
        z += coefs.possdiff_per_minute * features.possdiff_per_minute;
        z += coefs.fgpossdiff_per_minute * features.fgpossdiff_per_minute;
        z += coefs.is_offense_home * features.is_offense_home;
        z += coefs.offense_log_pass_prob * features.offense_log_pass_prob;
        z += coefs.defense_log_pass_prob * features.defense_log_pass_prob;
        z += coefs.off_def_lpp * features.off_def_lpp;
        z += coefs.off_lpp_rz * features.off_lpp_rz;
        z += coefs.def_lpp_rz * features.def_lpp_rz;
        z += coefs.off_def_lpp_rz * features.off_def_lpp_rz;
        z += coefs.off_lpp_outside_rz * features.off_lpp_outside_rz;
        z += coefs.off_lpp_inside_rz * features.off_lpp_inside_rz;
        z += coefs.def_lpp_outside_rz * features.def_lpp_outside_rz;
        z += coefs.def_lpp_inside_rz * features.def_lpp_inside_rz;
        z += coefs.off_lpp_pdpm * features.off_lpp_pdpm;
        z += coefs.def_lpp_pdpm * features.def_lpp_pdpm;
        z += coefs.off_lpp_rz_pdpm * features.off_lpp_rz_pdpm;
        z += coefs.def_lpp_rz_pdpm * features.def_lpp_rz_pdpm;
        z += coefs.down_1 * features.down_1;
        z += coefs.down_2 * features.down_2;
        z += coefs.down_3 * features.down_3;
        z += coefs.down_4 * features.down_4;
        z += coefs.goal_to_go * features.goal_to_go;
        z += coefs.z_ydstogo * features.z_ydstogo;
        z += coefs.ydstogo_pct * features.ydstogo_pct;
        z += coefs.log_ydstogo_pct * features.log_ydstogo_pct;
        z += coefs.to_go_1st * features.to_go_1st;
        z += coefs.to_go_2nd * features.to_go_2nd;
        z += coefs.to_go_3rd * features.to_go_3rd;
        z += coefs.to_go_4th * features.to_go_4th;
        z += coefs.log_to_go_1st * features.log_to_go_1st;
        z += coefs.log_to_go_2nd * features.log_to_go_2nd;
        z += coefs.log_to_go_3rd * features.log_to_go_3rd;
        z += coefs.log_to_go_4th * features.log_to_go_4th;
        z += coefs.fp_1st * features.fp_1st;
        z += coefs.fp_2nd * features.fp_2nd;
        z += coefs.fp_3rd * features.fp_3rd;
        z += coefs.fp_4th * features.fp_4th;
        z += coefs.fg_sigmoid * features.fg_sigmoid;
        z += coefs.punt_sigmoid * features.punt_sigmoid;
        z += coefs.yardline_pct * features.yardline_pct;
        z += coefs.yardline_pct_sq * features.yardline_pct_sq;
        z += coefs.log_yardline_pct * features.log_yardline_pct;
        z += coefs.yardline_fgsig_4th * features.yardline_fgsig_4th;
        z += coefs.yardline_puntsig_4th * features.yardline_puntsig_4th;
        z += coefs.goal_to_go_yardline * features.goal_to_go_yardline;
        z += coefs.log_goal_to_go_yardline * features.log_goal_to_go_yardline;
        z += coefs.yards_to_go_yardline * features.yards_to_go_yardline;
        z += coefs.log_yards_to_go_yardline * features.log_yards_to_go_yardline;
        z += coefs.yardline_4th * features.yardline_4th;
        z += coefs.log_yardline_4th * features.log_yardline_4th;
        z += coefs.yardline_not_4th * features.yardline_not_4th;
        z += coefs.log_yardline_not_4th * features.log_yardline_not_4th;
        z += coefs.inside_2m_warning * features.inside_2m_warning;
        z += coefs.garbage_time_win * features.garbage_time_win;
        z += coefs.garbage_time_loss * features.garbage_time_loss;
        z += coefs.ol_z * features.oline_rushing_z;
        z += coefs.dl_z * features.dline_rushing_z;
        z += coefs.ol_dl_z * features.oline_rushing_z * features.dline_rushing_z;
        z += coefs.log_mean_yards * log_mean_yards;
        z += coefs.log_std_yards * log_std_yards;
        z += coefs.yoe_mean * rusher.yoe_mean;
        z += coefs.yoe_std * rusher.yoe_std;
        z += coefs.yoe_var * yoe_var;
        z += coefs.yardline_std * features.yardline_pct * rusher.yoe_std;
        z += coefs.yardline_var * features.yardline_pct * yoe_var;
        z += coefs.togo_std * features.z_ydstogo * rusher.yoe_std;
        z += coefs.togo_var * features.z_ydstogo * yoe_var;
        z += coefs.clock_runs_after * clock_runs_after;
        z
    }

    fn is_scramble_fumble_lost(features: &PlaycallFeatures, rusher: &RushingParams) -> bool {
        let coefs = RushingModel::scrambling_fumble_lost_coef();
        let z = RushingModel::get_z(&features, rusher, &coefs, YARDS_PER_SCRAMBLE, 0.0);
        random_sigmoid(z)
    }

    fn is_scramble_fl_td(features: &PlaycallFeatures, rusher: &RushingParams) -> bool {
        let coefs = RushingModel::scrambling_prob_fl_td_coef();
        let z = RushingModel::get_z(&features, rusher, &coefs, YARDS_PER_SCRAMBLE, 0.0);
        random_sigmoid(z)
    }
}
