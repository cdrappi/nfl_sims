pub mod coef;

use std::collections::HashMap;

use crate::models::{
    dropback::{EPSILON_AIR_YARDS, MEAN_AIR_YARDS},
    features::{PlaycallFeatures, EPSILON},
    shares::compute_conditional_shares,
};
use crate::params::skill_player::SkillPlayer;
use crate::sim::{
    play_result::{ReceivingYards, TargetOutcome, TargetResult, TurnoverOutcome},
    GameSim,
};
use crate::start::HomeAway;
use crate::state::yards_to_goal::YardsToGoal;
use crate::util::stats::{
    double_truncated_poisson, negbinom_pmf, normal_cdf, poisson_pmf, random_discrete,
    random_sigmoid, sigmoid_prob, truncated_negbinom, truncated_poisson,
};

const PROB_COMPLETION: f32 = 0.60;
const PROB_INTERCEPTION: f32 = 0.025;

#[derive(Debug)]
pub struct TargetModel {
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
    air_yards: f32,
    neg_log_air_yards: f32,
    pos_log_air_yards: f32,
    pos_log_air_yards_sq: f32,
    qb_ay_oe: f32,
    log_qb_mean_ay: f32,
    log_qb_comp_prob: f32,
    log_qb_int_prob: f32,
    qb_def_comp: f32,
    qb_def_int: f32,
    wr_adot: f32,
    log_wr_adot: f32,
    wr_xyac: f32,
    log_wr_xyac: f32,
    wr_yac_oe: f32,
    wr_yac_oe_sq: f32,
    log_wr_catch_prob: f32,
    def_yac_oe: f32,
    qb_wr_lcp: f32,
    clock_runs_after: f32,
}

impl TargetModel {
    pub fn simulate_target(sim: &GameSim, air_yards: i8) -> TargetResult {
        let dtg = sim.game_state.play.expect_downtogo();
        let targeted_receiver_id =
            TargetModel::simulate_receiver(sim, air_yards, dtg.yards_to_goal);
        // log::info!("Simmed receiver {:?}", targeted_receiver_id);
        let mut features = TargetModel::features(sim, air_yards, &targeted_receiver_id);

        // log::debug!("{:?}", features);

        let yards_to_returner_goal = dtg.yards_to_goal.flip().0 as i8 + air_yards;
        if TargetModel::is_interception(&features) {
            // log::info!("interception!");
            let turnover_outcome = match TargetModel::is_int_pick_six(&features) {
                true => TurnoverOutcome::Touchdown,
                false => {
                    let return_yards =
                        TargetModel::sim_int_return_yards(&features, yards_to_returner_goal);
                    // log::info!(
                    //     "ytg: {:?} retyds: {:?}, ay: {:?}",
                    //     dtg.yards_to_goal,
                    //     return_yards,
                    //     air_yards
                    // );
                    let next_yards_to_goal =
                        dtg.yards_to_goal.flip().0 as i8 + (air_yards - return_yards);
                    TurnoverOutcome::YardsToGoal(match next_yards_to_goal >= 100 {
                        // touchback
                        true => YardsToGoal::touchback(),
                        false => YardsToGoal(next_yards_to_goal as u8),
                    })
                }
            };
            return TargetResult {
                targeted_receiver_id,
                outcome: TargetOutcome::Interception(air_yards, turnover_outcome),
            };
        }

        if !TargetModel::is_completion(&features) {
            // log::info!("incomplete.");
            return TargetResult {
                targeted_receiver_id,
                outcome: TargetOutcome::Incomplete(air_yards),
            };
        }

        let outcome: TargetOutcome = TargetModel::sim_completion(&mut features, dtg.yards_to_goal);
        TargetResult {
            targeted_receiver_id,
            outcome,
        }
    }

    fn is_completion(features: &TargetModel) -> bool {
        let coef = TargetModel::prob_completion_coef();
        let z = TargetModel::get_z(features, &coef);
        // let res = random_sigmoid(z);
        // log::info!(
        //     "completion z: {:?}, prob = {:?}, res = {}",
        //     z,
        //     sigmoid_prob(z),
        //     res
        // );
        // res
        random_sigmoid(z)
    }

    fn sim_completion(features: &mut TargetModel, yards_to_goal: YardsToGoal) -> TargetOutcome {
        let air_yards = features.air_yards as i8;
        // log::info!(
        //     "completed pass, ytg = {:?}, ay = {:?}",
        //     yards_to_goal,
        //     air_yards
        // );
        if air_yards >= yards_to_goal.0 as i8 {
            // completion in the endzone => automatic touchdown
            return TargetOutcome::Touchdown(ReceivingYards {
                air_yards,
                yards_after_catch: 0,
            });
        }

        match TargetModel::is_td_reception(&features) {
            true => TargetOutcome::Touchdown(ReceivingYards {
                air_yards,
                yards_after_catch: (yards_to_goal.0 as i8 - air_yards).max(0),
            }),
            false => {
                let clock_runs_after = TargetModel::sim_clock_runs_after(&features);
                if clock_runs_after {
                    features.clock_runs_after = 1.0;
                }

                let yards_after_catch =
                    TargetModel::sim_yac(&features, yards_to_goal.0 as i8 - air_yards);
                // log::info!(
                //     "YTG = {:?}, air yards = {:?}, YAC = {:?}, ",
                //     yards_to_goal,
                //     air_yards,
                //     yards_after_catch
                // );
                let receiving_yards = ReceivingYards {
                    air_yards,
                    yards_after_catch,
                };
                TargetOutcome::Yards(receiving_yards, !clock_runs_after)
            }
        }
    }

    fn is_td_reception(features: &TargetModel) -> bool {
        let coef = TargetModel::prob_catch_td_coef();
        let z = TargetModel::get_z(features, &coef);
        random_sigmoid(z)
    }

    fn sim_clock_runs_after(features: &TargetModel) -> bool {
        let coef = TargetModel::clock_runs_after_coef();
        let z = TargetModel::get_z(features, &coef);
        random_sigmoid(z)
    }

    fn sim_yac(features: &TargetModel, caught_yards_to_goal: i8) -> i8 {
        // already handled the case where the pass is caught in the endzone
        // if caught_yards_to_goal <= 0 {
        //     return 0;
        // }
        match TargetModel::is_yac_positive(&features, caught_yards_to_goal) {
            true => TargetModel::sim_pos_yac(&features, caught_yards_to_goal),
            false => TargetModel::sim_neg_yac(&features, caught_yards_to_goal),
        }
    }

    fn is_yac_positive(features: &TargetModel, caught_yards_to_goal: i8) -> bool {
        if caught_yards_to_goal <= 1 {
            // would be a TD
            return false;
        }
        if caught_yards_to_goal >= 99 {
            // would be a safety
            return true;
        }
        let coef = TargetModel::is_pos_yac_coef();
        let z = TargetModel::get_z(&features, &coef);
        random_sigmoid(z)
    }

    fn sim_neg_yac(features: &TargetModel, caught_yards_to_goal: i8) -> i8 {
        let coef = TargetModel::neg_yac_coef();
        let lambda = TargetModel::get_z(&features, &coef).exp();
        // log::info!(
        //     "neg yac poisson: {:?}. caught ytg = {:?}",
        //     lambda,
        //     caught_yards_to_goal
        // );

        let caught_yards_to_safety = 100 - caught_yards_to_goal as u8;
        -1 * truncated_poisson(lambda, caught_yards_to_safety) as i8
    }

    fn sim_pos_yac(features: &TargetModel, caught_yards_to_goal: i8) -> i8 {
        let coef_mean = TargetModel::pos_yac_coef();
        let coef_var = TargetModel::pos_yac_var_coef();
        let mean = TargetModel::get_z(&features, &coef_mean).exp();
        let var = TargetModel::get_z(&features, &coef_var).exp();
        let max_exclusive = (caught_yards_to_goal - 1) as u8;
        1 + match var > mean {
            true => truncated_negbinom(mean, var, max_exclusive) as i8,
            false => truncated_poisson(mean, max_exclusive) as i8,
        }
    }

    fn is_interception(features: &TargetModel) -> bool {
        let coef = TargetModel::prob_int_coef();
        let z = TargetModel::get_z(&features, &coef);
        random_sigmoid(z)
    }

    fn sim_int_return_yards(features: &TargetModel, yards_to_returner_goal: i8) -> i8 {
        // log::info!("int return. yards to goal = {:?}", yards_to_returner_goal);
        match TargetModel::is_int_yards_positive(&features, yards_to_returner_goal) {
            true => TargetModel::sim_pos_int_return_yards(features, yards_to_returner_goal),
            false => TargetModel::sim_neg_int_return_yards(features, yards_to_returner_goal),
        }
    }

    fn is_int_yards_positive(features: &TargetModel, yards_to_returner_goal: i8) -> bool {
        if yards_to_returner_goal <= 1 {
            // would be a TD
            return false;
        }
        if yards_to_returner_goal >= 99 {
            // would be a touchback/safety
            return true;
        }
        let coef = TargetModel::is_int_return_yards_pos_coef();
        let z = TargetModel::get_z(&features, &coef);
        random_sigmoid(z)
    }

    fn sim_neg_int_return_yards(features: &TargetModel, yards_to_returner_goal: i8) -> i8 {
        // log::info!("neg int return yards poisson: {:?}", lambda);
        if yards_to_returner_goal >= 100 {
            // assume a touchback if it's non-positive yardage in the endzone
            return 0;
        }
        let yards_to_safety = (100 - yards_to_returner_goal) as u8;
        let coef = TargetModel::int_neg_return_yards_coef();
        let lambda = TargetModel::get_z(&features, &coef)
            .exp()
            .min(yards_to_safety as f32);
        // log::info!(
        //     "simming return yards. lambda = {}, YTG = {}",
        //     lambda,
        //     yards_to_returner_goal
        // );
        // if we're at this point, it's not a pick 6, so if they intercept it at the goal line,
        // then assume they have negative return yardage. dumb, but most correct thing to do
        let min_inclusive = (1 - yards_to_returner_goal).max(0) as u8;
        -1 * double_truncated_poisson(lambda, min_inclusive, yards_to_safety) as i8
    }

    fn sim_pos_int_return_yards(features: &TargetModel, yards_to_returner_goal: i8) -> i8 {
        let coef_mean = TargetModel::int_pos_return_yards_coef();
        let coef_var = TargetModel::int_pos_return_yards_var_coef();
        let mean = TargetModel::get_z(&features, &coef_mean)
            .exp()
            .min(yards_to_returner_goal as f32);
        let var = TargetModel::get_z(&features, &coef_var).exp();
        let max_exclusive = (yards_to_returner_goal - 1).max(0) as u8;
        1 + match var > mean {
            true => truncated_negbinom(mean, var, max_exclusive) as i8,
            false => truncated_poisson(mean, max_exclusive) as i8,
        }
        // log::info!(
        //     "pos int return yards NB: {:?}, {:?}, ytg = {:?}",
        //     mean,
        //     var,
        //     yards_to_returner_goal
        // );
    }

    fn is_int_pick_six(features: &TargetModel) -> bool {
        let coef = TargetModel::prob_pick_six_coef();
        let z = TargetModel::get_z(&features, &coef);
        random_sigmoid(z)
    }

    fn simulate_receiver(sim: &GameSim, air_yards: i8, yards_to_goal: YardsToGoal) -> String {
        let ytg_conditional_shares = TargetModel::target_shares_by_location(sim, yards_to_goal);
        let ay_conditional_shares =
            TargetModel::adjust_target_shares_by_air_yards(sim, &ytg_conditional_shares, air_yards);

        match random_discrete(ay_conditional_shares.clone()) {
            Ok(receiver_id) => receiver_id,
            Err(_we) => {
                log::info!(
                    "air yards = {}, YTG = {}\nYTG conditional = {:?}\nAY conditional = {:?}",
                    air_yards,
                    yards_to_goal,
                    ytg_conditional_shares,
                    ay_conditional_shares
                );
                panic!(
                    "Failed to sim receiver. Injuries = {:?}",
                    sim.offense_params().injuries
                );
            }
        }
    }

    fn adjust_target_shares_by_air_yards(
        sim: &GameSim,
        ytg_conditional_shares: &Vec<(String, f32)>,
        air_yards: i8,
    ) -> Vec<(String, f32)> {
        let offense = sim.offense_params();
        let mut id_shares = vec![];

        let mut cumsum = 0.0;
        for (pid, loc_share) in ytg_conditional_shares.iter() {
            let player = offense.skill_players.get(pid).unwrap();
            let prob_pos_ay = PositiveAirYardsTargetModel::prob_pos_ay(player);
            let ay_prob = match air_yards > 0 {
                true => {
                    let pos_cond_mean = ConditionalAirYardsModel::pos_ay_cond_mean(player);
                    let pos_cond_var = ConditionalAirYardsModel::pos_ay_cond_var(player);
                    let pmf = match pos_cond_var > pos_cond_mean {
                        true => negbinom_pmf(pos_cond_mean, pos_cond_var, air_yards as u8),
                        false => poisson_pmf(pos_cond_mean, air_yards as u8),
                    };
                    // log::info!(
                    //     "++ {} ... PMF = {:?} adot = {:?}, atdot_stdev = {:?}, P(posAy) = {:?}, E[ay|pos] = {:?}, Stdev[ay|pos] = {:?}",
                    //     player.name,
                    //     pmf,
                    //     player.adot,
                    //     player.adot_std,
                    //     prob_pos_ay,
                    //     pos_cond_mean,
                    //     pos_cond_var.sqrt(),
                    // );
                    pmf * prob_pos_ay
                }
                false => {
                    let neg_cond_mean = ConditionalAirYardsModel::neg_ay_cond_mean_abs(player);
                    let pmf = poisson_pmf(neg_cond_mean, (-1 * air_yards) as u8);
                    // log::info!(
                    //     "-- {} ... PMF = {:?} adot = {:?}, atdot_stdev = {:?}, P(negAY) = {:?}, E[ay|neg] = {:?}",
                    //     player.name,
                    //     pmf,
                    //     player.adot,
                    //     player.adot_std,
                    //     1.0 - prob_pos_ay,
                    //     neg_cond_mean,
                    // );
                    pmf * (1.0 - prob_pos_ay)
                }
            };

            let cond_prob = ay_prob * loc_share;
            cumsum += cond_prob;

            id_shares.push((pid.clone(), cond_prob));
        }

        // if cumsum == 0.0 {
        //     log::info!("adj air yards cumsum = 0.0. {}", air_yards);
        // }

        for (_, cond_prob) in id_shares.iter_mut() {
            *cond_prob /= cumsum;
        }
        id_shares
    }

    fn target_shares_by_location(sim: &GameSim, yards_to_goal: YardsToGoal) -> Vec<(String, f32)> {
        let offense = sim.offense_params();

        let mut team_loc_probs = HashMap::new();
        team_loc_probs.insert(true, offense.team.prob_rz_given_target);

        let mut marginal_shares = HashMap::new();
        let mut player_loc_probs = HashMap::new();
        for (pid, player) in offense.skill_players.iter() {
            marginal_shares.insert(pid.clone(), player.ms_targets_live);
            let mut player_rz_probs = HashMap::new();
            player_rz_probs.insert(true, player.prob_rz_given_target);
            player_loc_probs.insert(pid.clone(), player_rz_probs);
        }

        let cond_shares =
            compute_conditional_shares(marginal_shares, player_loc_probs, team_loc_probs, false);
        cond_shares[&yards_to_goal.is_redzone()].clone()
    }

    fn get_z(features: &TargetModel, coef: &TargetModel) -> f32 {
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
        z += coef.log_qb_comp_prob * features.log_qb_comp_prob;
        z += coef.log_qb_int_prob * features.log_qb_int_prob;
        z += coef.qb_def_comp * features.qb_def_comp;
        z += coef.qb_def_int * features.qb_def_int;
        z += coef.air_yards * features.air_yards;
        z += coef.neg_log_air_yards * features.neg_log_air_yards;
        z += coef.pos_log_air_yards * features.pos_log_air_yards;
        z += coef.pos_log_air_yards_sq * features.pos_log_air_yards_sq;
        z += coef.qb_ay_oe * features.qb_ay_oe;
        z += coef.log_qb_mean_ay * features.log_qb_mean_ay;
        z += coef.wr_adot * features.wr_adot;
        z += coef.log_wr_adot * features.log_wr_adot;
        z += coef.log_wr_catch_prob * features.log_wr_catch_prob;
        z += coef.wr_xyac * features.wr_xyac;
        z += coef.log_wr_xyac * features.log_wr_xyac;
        z += coef.wr_yac_oe * features.wr_yac_oe;
        z += coef.wr_yac_oe_sq * features.wr_yac_oe_sq;
        z += coef.qb_wr_lcp * features.qb_wr_lcp;
        z += coef.def_yac_oe * features.def_yac_oe;
        z
    }

    fn features(sim: &GameSim, air_yards: i8, receiver_id: &String) -> TargetModel {
        let f = PlaycallFeatures::new(sim);

        let dtg = sim.game_state.play.expect_downtogo();

        let (offense, defense) = match dtg.possession {
            HomeAway::Home => (&sim.game_params.home, &sim.game_params.away),
            HomeAway::Away => (&sim.game_params.away, &sim.game_params.home),
        };

        let qb = &offense.quarterback();
        let receiver = offense.skill_players.get(receiver_id).unwrap();

        let log_qb_scramble = (qb.scramble_rate + EPSILON).ln();
        let log_qbps = (qb.prob_sack_given_hit + EPSILON).ln();

        let log_qb_comp_prob = (PROB_COMPLETION + qb.cpoe).ln();
        let log_qb_int_prob = (PROB_INTERCEPTION - qb.int_ue).max(EPSILON).ln();
        let log_wr_catch_prob = (PROB_COMPLETION + receiver.prob_catch_oe).ln();

        let pos_log_air_yards = (air_yards.max(1) as f32).ln();

        TargetModel {
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
            log_qbps,
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
            air_yards: air_yards as f32,
            neg_log_air_yards: ((-1 * air_yards).max(1) as f32).ln(),
            pos_log_air_yards,
            pos_log_air_yards_sq: pos_log_air_yards.powi(2),
            qb_ay_oe: qb.ayoe,
            log_qb_mean_ay: (MEAN_AIR_YARDS + qb.ayoe).max(EPSILON_AIR_YARDS).ln(),
            log_qb_comp_prob,
            log_qb_int_prob,
            qb_def_comp: log_qb_comp_prob * defense.team.defense_completion_z,
            qb_def_int: log_qb_int_prob * defense.team.defense_interception_z,
            wr_adot: receiver.adot,
            log_wr_adot: receiver.adot.max(EPSILON_AIR_YARDS).ln(),
            log_wr_catch_prob,
            wr_xyac: receiver.xyac,
            log_wr_xyac: receiver.xyac.max(EPSILON_AIR_YARDS).ln(),
            wr_yac_oe: receiver.yac_oe,
            wr_yac_oe_sq: receiver.yac_oe.powi(2),
            def_yac_oe: defense.team.defense_yac_oe,
            qb_wr_lcp: log_qb_comp_prob * log_wr_catch_prob,
            // filled in later
            clock_runs_after: 0.0,
        }
    }
}

pub struct PositiveAirYardsTargetModel {
    intercept: f32,
    zero_sigma: f32,
    neg1_sigma: f32,
    neg2_sigma: f32,
    xyac: f32,
    yac_oe: f32,
}

impl PositiveAirYardsTargetModel {
    fn prob_pos_ay(player: &SkillPlayer) -> f32 {
        let coef = PositiveAirYardsTargetModel::is_positive_ay_target_coef();
        let features = PositiveAirYardsTargetModel::features(player);
        let mut z = coef.intercept;
        z += coef.zero_sigma * features.zero_sigma;
        z += coef.neg1_sigma * features.neg1_sigma;
        z += coef.neg2_sigma * features.neg2_sigma;
        z += coef.xyac * features.xyac;
        z += coef.yac_oe * features.yac_oe;

        sigmoid_prob(z)
    }

    fn features(player: &SkillPlayer) -> PositiveAirYardsTargetModel {
        PositiveAirYardsTargetModel {
            intercept: 1.0,
            zero_sigma: normal_cdf(player.adot, player.adot_std, 0.0),
            neg1_sigma: normal_cdf(player.adot - player.adot_std, player.adot_std, 0.0),
            neg2_sigma: normal_cdf(player.adot - 2.0 * player.adot_std, player.adot_std, 0.0),
            xyac: player.xyac,
            yac_oe: player.yac_oe,
        }
    }
}
pub struct ConditionalAirYardsModel {
    intercept: f32,
    adot: f32,
    adot_std: f32,
    xyac: f32,
    yac_oe: f32,
}

impl ConditionalAirYardsModel {
    fn neg_ay_cond_mean_abs(player: &SkillPlayer) -> f32 {
        let coef = ConditionalAirYardsModel::neg_ay_target_mean_coef();
        let mut z = coef.intercept;
        z += coef.adot * player.adot;
        z += coef.adot_std * player.adot_std;
        z += coef.xyac * player.xyac;
        z += coef.yac_oe * player.yac_oe;
        z.exp()
    }

    fn pos_ay_cond_mean(player: &SkillPlayer) -> f32 {
        /*
        aDot = (1 - P(pos)) * E[ay|neg] + P(pos) * E[ay|pos]
        P(pos) * E[ay|pos] = (1 - P(pos)) * E[ay|neg] - aDot
        E[ay|pos] = (aDot - (1 - P(pos)) * E[ay|neg])) / P(pos))
        */
        let prob_pos = PositiveAirYardsTargetModel::prob_pos_ay(player);
        let neg_cond_mean = -1.0 * ConditionalAirYardsModel::neg_ay_cond_mean_abs(player);
        (player.adot - (1.0 - prob_pos) * neg_cond_mean) / prob_pos
    }

    fn pos_ay_cond_var(player: &SkillPlayer) -> f32 {
        let coef = ConditionalAirYardsModel::pos_ay_target_variance_coef();
        let mut z = coef.intercept;
        z += coef.adot * player.adot;
        z += coef.adot_std * player.adot_std;
        z += coef.xyac * player.xyac;
        z += coef.yac_oe * player.yac_oe;
        z.exp()
    }
}
