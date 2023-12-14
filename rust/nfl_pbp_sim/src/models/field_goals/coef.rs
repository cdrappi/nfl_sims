use crate::models::field_goals::FgModel;

impl FgModel {

    pub fn is_fg_blocked_coef() -> FgModel {
        FgModel {
            intercept: -4.925,
            is_offense_home: 0.328,
            distance_pct: 0.604,
            distance_2: 0.493,
            distance_3: 0.318,
            roof_dome: -0.149,
            roof_open: -0.072,
            roof_outdoors: 0.329,
            roof_dome_dist: 0.107,
            roof_open_dist: -0.010,
            roof_outdoors_dist: 0.551,
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
            intercept: -3.399,
            is_offense_home: 0.143,
            distance_pct: -0.012,
            distance_2: -0.007,
            distance_3: -0.003,
            roof_dome: 0.118,
            roof_open: -0.004,
            roof_outdoors: -0.114,
            roof_dome_dist: 0.064,
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
            intercept: 3.337,
            is_offense_home: 0.092,
            distance_pct: -2.653,
            distance_2: -2.101,
            distance_3: -1.304,
            roof_dome: 0.011,
            roof_open: 0.047,
            roof_outdoors: 0.253,
            roof_dome_dist: -0.725,
            roof_open_dist: -0.021,
            roof_outdoors_dist: -1.723,
            short_fg_z: 0.757,
            long_fg_z: 0.982,
            short_z_dist: -0.491,
            long_z_dist: -0.126,
            short_z_dist_2: -0.525,
            long_z_dist_2: -0.246,
            short_z_dist_3: -0.362,
            long_z_dist_3: -0.186,
        }
    }

}