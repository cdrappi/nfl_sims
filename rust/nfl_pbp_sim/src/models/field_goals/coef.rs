use crate::models::field_goals::FgModel;

impl FgModel {

    pub fn is_fg_blocked_coef() -> FgModel {
        FgModel {
            intercept: -4.916,
            is_offense_home: 0.318,
            distance_pct: 0.603,
            distance_2: 0.493,
            distance_3: 0.318,
            roof_dome: -0.145,
            roof_open: -0.072,
            roof_outdoors: 0.326,
            roof_dome_dist: 0.110,
            roof_open_dist: -0.011,
            roof_outdoors_dist: 0.547,
            short_fg_z: 0.0,
            long_fg_z: 0.0,
            short_z_dist: 0.0,
            long_z_dist: 0.0,
            short_z_dist_2: 0.0,
            long_z_dist_2: 0.0,
            short_z_dist_3: 0.0,
            long_z_dist_3: 0.0,
        }
    }


    pub fn is_fg_block_returned_for_td_coef() -> FgModel {
        FgModel {
            intercept: -3.392,
            is_offense_home: 0.144,
            distance_pct: -0.012,
            distance_2: -0.007,
            distance_3: -0.003,
            roof_dome: 0.117,
            roof_open: -0.004,
            roof_outdoors: -0.114,
            roof_dome_dist: 0.063,
            roof_open_dist: -0.002,
            roof_outdoors_dist: -0.074,
            short_fg_z: 0.0,
            long_fg_z: 0.0,
            short_z_dist: 0.0,
            long_z_dist: 0.0,
            short_z_dist_2: 0.0,
            long_z_dist_2: 0.0,
            short_z_dist_3: 0.0,
            long_z_dist_3: 0.0,
        }
    }


    pub fn is_fg_good_coef() -> FgModel {
        FgModel {
            intercept: 3.334,
            is_offense_home: 0.093,
            distance_pct: -2.650,
            distance_2: -2.102,
            distance_3: -1.307,
            roof_dome: 0.022,
            roof_open: 0.047,
            roof_outdoors: 0.249,
            roof_dome_dist: -0.728,
            roof_open_dist: -0.020,
            roof_outdoors_dist: -1.715,
            short_fg_z: 0.753,
            long_fg_z: 0.983,
            short_z_dist: -0.476,
            long_z_dist: -0.143,
            short_z_dist_2: -0.514,
            long_z_dist_2: -0.263,
            short_z_dist_3: -0.357,
            long_z_dist_3: -0.198,
        }
    }

}