use crate::models::field_goals::FgModel;

impl FgModel {

    pub fn is_fg_blocked_coef() -> FgModel {
        FgModel {
            intercept: -4.913,
            is_offense_home: 0.278,
            distance_pct: 0.602,
            distance_2: 0.490,
            distance_3: 0.316,
            roof_dome: -0.162,
            roof_open: -0.073,
            roof_outdoors: 0.342,
            roof_dome_dist: 0.103,
            roof_open_dist: -0.011,
            roof_outdoors_dist: 0.552,
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
            intercept: -3.423,
            is_offense_home: 0.148,
            distance_pct: -0.012,
            distance_2: -0.006,
            distance_3: -0.002,
            roof_dome: 0.119,
            roof_open: -0.004,
            roof_outdoors: -0.116,
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
            intercept: 3.402,
            is_offense_home: 0.076,
            distance_pct: -2.699,
            distance_2: -2.131,
            distance_3: -1.319,
            roof_dome: 0.015,
            roof_open: 0.056,
            roof_outdoors: 0.248,
            roof_dome_dist: -0.754,
            roof_open_dist: -0.014,
            roof_outdoors_dist: -1.744,
            short_fg_z: 0.748,
            long_fg_z: 0.956,
            short_z_dist: -0.498,
            long_z_dist: -0.102,
            short_z_dist_2: -0.532,
            long_z_dist_2: -0.224,
            short_z_dist_3: -0.369,
            long_z_dist_3: -0.171,
        }
    }

}