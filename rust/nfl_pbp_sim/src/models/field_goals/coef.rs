use crate::models::field_goals::FgModel;

impl FgModel {

    pub fn is_fg_blocked_coef() -> FgModel {
        FgModel {
            intercept: -4.940,
            is_offense_home: 0.323,
            distance_pct: 0.606,
            distance_2: 0.494,
            distance_3: 0.318,
            roof_dome: -0.149,
            roof_open: -0.073,
            roof_outdoors: 0.329,
            roof_dome_dist: 0.108,
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
            intercept: 3.379,
            is_offense_home: 0.086,
            distance_pct: -2.676,
            distance_2: -2.115,
            distance_3: -1.310,
            roof_dome: 0.010,
            roof_open: 0.057,
            roof_outdoors: 0.251,
            roof_dome_dist: -0.747,
            roof_open_dist: -0.014,
            roof_outdoors_dist: -1.728,
            short_fg_z: 0.751,
            long_fg_z: 0.951,
            short_z_dist: -0.482,
            long_z_dist: -0.107,
            short_z_dist_2: -0.519,
            long_z_dist_2: -0.227,
            short_z_dist_3: -0.360,
            long_z_dist_3: -0.173,
        }
    }

}