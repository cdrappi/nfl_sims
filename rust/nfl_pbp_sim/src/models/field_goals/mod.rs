use crate::util::stats::random_sigmoid;
pub mod coef;
use crate::state::yards_to_goal::YardsToGoal;
use crate::{
    game_loop::field_goals::fg_distance,
    params::weather::StadiumType,
    sim::{
        play_result::{FieldGoalResult, TurnoverOutcome},
        GameSim,
    },
    start::HomeAway,
};

pub struct FgModel {
    intercept: f32,
    is_offense_home: f32,
    distance_pct: f32,
    distance_2: f32,
    distance_3: f32,
    short_fg_z: f32,
    long_fg_z: f32,
    short_z_dist: f32,
    long_z_dist: f32,
    short_z_dist_2: f32,
    long_z_dist_2: f32,
    short_z_dist_3: f32,
    long_z_dist_3: f32,
    roof_dome: f32,
    roof_open: f32,
    roof_outdoors: f32,
    roof_dome_dist: f32,
    roof_open_dist: f32,
    roof_outdoors_dist: f32,
}

impl FgModel {
    pub fn get_result(sim: &GameSim) -> FieldGoalResult {
        match FgModel::is_fg_blocked(sim) {
            true => FieldGoalResult::Blocked(match FgModel::is_fg_block_returned_for_td(sim) {
                true => TurnoverOutcome::Touchdown,
                false => {
                    let kicked_from = fg_distance(sim.expect_downtogo().yards_to_goal.0) - 10;
                    // log::info!("kicked from: {:?}", kicked_from);
                    TurnoverOutcome::YardsToGoal(YardsToGoal::new(kicked_from).flip())
                }
            }),
            false => FieldGoalResult::AttemptedFg(FgModel::is_good(sim)),
        }
    }

    fn is_fg_blocked(sim: &GameSim) -> bool {
        let coefs = FgModel::is_fg_blocked_coef();
        let features = FgModel::features(sim);
        let z = FgModel::get_z(coefs, features);
        random_sigmoid(z)
    }

    fn is_fg_block_returned_for_td(sim: &GameSim) -> bool {
        let coefs = FgModel::is_fg_block_returned_for_td_coef();
        let features = FgModel::features(sim);
        let z = FgModel::get_z(coefs, features);
        random_sigmoid(z)
    }

    fn is_good(sim: &GameSim) -> bool {
        let coefs = FgModel::is_fg_good_coef();
        let features = FgModel::features(sim);
        let z = FgModel::get_z(coefs, features);
        random_sigmoid(z)
    }

    fn get_z(coefs: FgModel, f: FgModel) -> f32 {
        let mut z = coefs.intercept;
        z += coefs.is_offense_home * f.is_offense_home;
        z += coefs.distance_pct * f.distance_pct;
        z += coefs.distance_2 * f.distance_2;
        z += coefs.distance_3 * f.distance_3;
        z += coefs.short_fg_z * f.short_fg_z;
        z += coefs.long_fg_z * f.long_fg_z;
        z += coefs.short_z_dist * f.short_z_dist;
        z += coefs.long_z_dist * f.long_z_dist;
        z += coefs.short_z_dist_2 * f.short_z_dist_2;
        z += coefs.long_z_dist_2 * f.long_z_dist_2;
        z += coefs.short_z_dist_3 * f.short_z_dist_3;
        z += coefs.long_z_dist_3 * f.long_z_dist_3;
        z += coefs.roof_dome * f.roof_dome;
        z += coefs.roof_open * f.roof_open;
        z += coefs.roof_outdoors * f.roof_outdoors;
        z += coefs.roof_dome_dist * f.roof_dome_dist;
        z += coefs.roof_open_dist * f.roof_open_dist;
        z += coefs.roof_outdoors_dist * f.roof_outdoors_dist;
        z
    }

    fn features(sim: &GameSim) -> FgModel {
        let possession = sim.game_state.play.possession();
        let offense = sim.game_params.get_team(possession);
        let is_offense_home = match possession {
            HomeAway::Home => 1.0,
            HomeAway::Away => 0.0,
        };
        let short_fg_z = offense.team.short_fg_z;
        let long_fg_z = offense.team.long_fg_z;

        let fg_distance = sim.game_state.play.field_goal_distance();
        let distance_pct = fg_distance as f32 / 100.0;
        let distance_2 = distance_pct * distance_pct;
        let distance_3 = distance_2 * distance_pct;

        let short_z_dist = distance_pct * short_fg_z;
        let long_z_dist = distance_pct * long_fg_z;

        let short_z_dist_2 = distance_2 * short_fg_z;
        let long_z_dist_2 = distance_2 * long_fg_z;

        let short_z_dist_3 = distance_3 * short_fg_z;
        let long_z_dist_3 = distance_3 * long_fg_z;

        let (roof_dome, roof_open, roof_outdoors) = match sim.game_params.weather.stadium_type {
            StadiumType::Dome => (1.0, 0.0, 0.0),
            StadiumType::Open => (0.0, 1.0, 0.0),
            StadiumType::Outdoor => (0.0, 0.0, 1.0),
        };

        FgModel {
            intercept: 1.0,
            is_offense_home,
            distance_pct,
            distance_2,
            distance_3,
            short_fg_z,
            long_fg_z,
            short_z_dist,
            long_z_dist,
            short_z_dist_2,
            long_z_dist_2,
            short_z_dist_3,
            long_z_dist_3,
            roof_dome,
            roof_open,
            roof_outdoors,
            roof_dome_dist: roof_dome * distance_pct,
            roof_open_dist: roof_open * distance_pct,
            roof_outdoors_dist: roof_outdoors * distance_pct,
        }
    }
}
