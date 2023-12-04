use crate::game_loop::field_goals::ENDZONE_LENGTH;
use crate::util::stats::{double_truncated_poisson, random_sigmoid, truncated_poisson};

use crate::state::yards_to_goal::YardsToGoal;
use crate::{
    sim::{
        play_result::{KickingResult, TurnoverOutcome},
        GameSim,
    },
    start::HomeAway,
};

pub mod coef;

pub struct PuntModel {
    intercept: f32,
    is_offense_home: f32,
    yardline_pct: f32,
    log_yardline_pct: f32,
    touchback_goodness: f32,
    log_touchback_goodness: f32,
    yardline_tbg: f32,
    kick_distance_pct: f32,
    log_kick_distance_pct: f32,
    punted_to_pct: f32,
    log_punted_to_pct: f32,
}

impl PuntModel {
    pub fn punt_result(sim: &GameSim) -> KickingResult {
        let down_togo = sim.expect_downtogo();
        if PuntModel::is_punt_blocked(sim) {
            return KickingResult::Blocked(TurnoverOutcome::YardsToGoal(
                down_togo.yards_to_goal.flip(),
            ));
        }
        let distance = PuntModel::sim_punt_distance(sim, down_togo.yards_to_goal.0);
        let features = PuntModel::features(sim, distance);

        if PuntModel::is_touchback(&features) {
            // log::info!("touchback");
            return KickingResult::PuntTouchback;
        }

        // in terms of receiving team's yards to goal
        let punt_lands_at = down_togo.yards_to_goal.flip().0 as i8 + distance as i8;
        if PuntModel::is_fair_catch(&features) {
            let return_team_yards_to_goal = match punt_lands_at >= 100 {
                true => 80, // touchback
                false => punt_lands_at as u8,
            };
            return KickingResult::ReturnedForYards(YardsToGoal(return_team_yards_to_goal));
        }

        if PuntModel::is_fumble_lost(&features) {
            return KickingResult::FumbleLost(
                match PuntModel::is_fumble_lost_return_td(&features) {
                    true => TurnoverOutcome::Touchdown,
                    // assume no fumbles that aren't returned for a touchdown
                    // are returned at the spot of the fumble. seems fine
                    false => {
                        let new_ytg_u8 = match punt_lands_at >= 95 {
                            true => 5,
                            false => (100 - punt_lands_at) as u8,
                        };
                        TurnoverOutcome::YardsToGoal(YardsToGoal::new(new_ytg_u8))
                    }
                },
            );
        }
        if PuntModel::is_punt_return_td(&features) {
            return KickingResult::ReturnedForTouchdown;
        }
        let return_yards = PuntModel::punt_return_yards(&features, punt_lands_at);
        let returned_to_raw = (punt_lands_at - return_yards) as u8;
        let returned_to = YardsToGoal::new(returned_to_raw);
        // log::info!(
        //     "distance = {}, from = {}, lands at = {}, retyds = {}, returned to = {}",
        //     distance,
        //     down_togo.yards_to_goal,
        //     punt_lands_at,
        //     return_yards,
        //     returned_to_raw
        // );
        // log::info!("punt returned to {}", returned_to_raw);
        return KickingResult::ReturnedForYards(returned_to);
    }

    fn is_punt_blocked(sim: &GameSim) -> bool {
        let features = PuntModel::features(sim, 0.0);
        let coefs = PuntModel::punt_block_coef();
        let z = PuntModel::get_z(coefs, &features);
        random_sigmoid(z)
    }

    fn is_touchback(features: &PuntModel) -> bool {
        let coefs = PuntModel::touchback_coef();
        let z = PuntModel::get_z(coefs, features);
        random_sigmoid(z)
    }

    fn is_fair_catch(features: &PuntModel) -> bool {
        let coefs = PuntModel::fair_catch_coef();
        let z = PuntModel::get_z(coefs, features);
        random_sigmoid(z)
    }

    fn is_fumble_lost(features: &PuntModel) -> bool {
        let coefs = PuntModel::fumble_lost_coef();
        let z = PuntModel::get_z(coefs, features);
        random_sigmoid(z)
    }

    fn is_fumble_lost_return_td(features: &PuntModel) -> bool {
        let coefs = PuntModel::fumble_lost_return_td_coef();
        let z = PuntModel::get_z(coefs, features);
        random_sigmoid(z)
    }

    fn is_punt_return_td(features: &PuntModel) -> bool {
        let coefs = PuntModel::punt_return_td_coef();
        let z = PuntModel::get_z(coefs, features);
        random_sigmoid(z)
    }

    fn punt_return_yards(features: &PuntModel, caught_yards_to_goal: i8) -> i8 {
        match PuntModel::is_pos_punt_return_yards(features, caught_yards_to_goal) {
            true => PuntModel::sim_pos_punt_return_yards(features, caught_yards_to_goal),
            false => PuntModel::sim_neg_punt_return_yards(features, caught_yards_to_goal),
        }
    }

    fn is_pos_punt_return_yards(features: &PuntModel, caught_yards_to_goal: i8) -> bool {
        if caught_yards_to_goal >= 99 {
            // return must be positive if caught at own goal line
            return true;
        }
        if caught_yards_to_goal <= 0 {
            // return must be negative if caught at opponents' goal line
            // (never going to happen, but this prevents theoretical panic)
            return false;
        }
        let coefs = PuntModel::is_pos_punt_return_yards_coef();
        let z = PuntModel::get_z(coefs, features);
        random_sigmoid(z)
    }

    fn sim_pos_punt_return_yards(features: &PuntModel, caught_yards_to_goal: i8) -> i8 {
        let coefs = PuntModel::pos_punt_return_yards_coef();
        let z = PuntModel::get_z(coefs, features).exp();
        // it's not a safety, so the minimum return would be to our own 1 yard line
        // (NOTE: we +1 at the end, so it's -100.max(0) not -99.max(1))
        let min_exclusive = (caught_yards_to_goal - 100).max(0) as u8;
        // it's not a touchdown, so the maximum return would be to the opponents' 1 yard line
        let max_exclusive = (caught_yards_to_goal - 1) as u8;
        // log::info!(
        //     "caught YTG = {}, min = {}, max = {}",
        //     caught_yards_to_goal,
        //     min_exclusive,
        //     max_exclusive
        // );
        1 + double_truncated_poisson(z, min_exclusive, max_exclusive) as i8
    }

    fn sim_neg_punt_return_yards(features: &PuntModel, caught_yards_to_goal: i8) -> i8 {
        let coefs = PuntModel::neg_punt_return_yards_coef();
        let z = PuntModel::get_z(coefs, features).exp();
        let yards_backwards_for_safety = (100 - caught_yards_to_goal) as u8;
        // log::info!(
        //     "caught ytg = {} ybfs = {}",
        //     caught_yards_to_goal,
        //     yards_backwards_for_safety
        // );
        -1 + -1 * truncated_poisson(z, yards_backwards_for_safety - 1) as i8
    }

    fn sim_punt_distance(sim: &GameSim, yards_to_goal: u8) -> f32 {
        // can't punt it more than 9 yards into the endzone
        let max_distance_exclusive = yards_to_goal + ENDZONE_LENGTH;

        let distance_features = PuntModel::features(sim, 0.0);
        let distance_coef = PuntModel::punt_distance_coef();
        let z = PuntModel::get_z(distance_coef, &distance_features).exp();
        // log::info!("punt distance z: {:?}", z);
        truncated_poisson(z, max_distance_exclusive) as f32
    }

    fn get_z(coef: PuntModel, features: &PuntModel) -> f32 {
        let mut z = coef.intercept;
        z += coef.is_offense_home * features.is_offense_home;
        z += coef.yardline_pct * features.yardline_pct;
        z += coef.log_yardline_pct * features.log_yardline_pct;
        z += coef.touchback_goodness * features.touchback_goodness;
        z += coef.log_touchback_goodness * features.log_touchback_goodness;
        z += coef.yardline_tbg * features.yardline_tbg;
        z += coef.kick_distance_pct * features.kick_distance_pct;
        z += coef.log_kick_distance_pct * features.log_kick_distance_pct;
        z += coef.punted_to_pct * features.punted_to_pct;
        z += coef.log_punted_to_pct * features.log_punted_to_pct;
        z
    }

    fn features(sim: &GameSim, kick_distance: f32) -> PuntModel {
        let down_togo = sim.expect_downtogo();
        let is_offense_home = match down_togo.possession {
            HomeAway::Home => 1.0,
            HomeAway::Away => 0.0,
        };

        let yardline_100 = down_togo.yards_to_goal.0 as f32;
        let yardline_pct = yardline_100 / 100.0;

        let kick_distance_pct = kick_distance / 100.0;
        // in terms of punting team
        let punted_to_pct = (yardline_100 - kick_distance) / 100.0;

        let touchback_goodness = 1.0 / (1.0 + (-1.0 * (yardline_100 - 60.0) / 8.0).exp());
        let yardline_tbg = yardline_pct * touchback_goodness;

        PuntModel {
            intercept: 0.0,
            is_offense_home,
            yardline_pct,
            log_yardline_pct: yardline_pct.max(0.01).ln(),
            touchback_goodness,
            log_touchback_goodness: touchback_goodness.max(0.01).ln(),
            yardline_tbg,
            kick_distance_pct,
            log_kick_distance_pct: kick_distance_pct.max(0.01).ln(),
            punted_to_pct,
            log_punted_to_pct: punted_to_pct.max(0.01).ln(),
        }
    }
}
