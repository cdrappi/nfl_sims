use crate::models::punt::PuntModel;

impl PuntModel {

    pub fn punt_block_coef() -> PuntModel {
        PuntModel {
            intercept: -5.202,
            is_offense_home: 0.196,
            yardline_pct: 0.061,
            log_yardline_pct: 0.107,
            touchback_goodness: 0.091,
            log_touchback_goodness: 0.411,
            yardline_tbg: 0.077,
            kick_distance_pct: 0.0,
            log_kick_distance_pct: 0.0,
            punted_to_pct: 0.0,
            log_punted_to_pct: 0.0,
        }
    }


    pub fn punt_distance_coef() -> PuntModel {
        PuntModel {
            intercept: 3.863,
            is_offense_home: 0.003,
            yardline_pct: 0.010,
            log_yardline_pct: 0.019,
            touchback_goodness: 0.025,
            log_touchback_goodness: 0.087,
            yardline_tbg: 0.016,
            kick_distance_pct: 0.0,
            log_kick_distance_pct: 0.0,
            punted_to_pct: 0.0,
            log_punted_to_pct: 0.0,
        }
    }

    pub fn touchback_coef() -> PuntModel {
        PuntModel {
            intercept: -16.356,
            is_offense_home: -0.186,
            yardline_pct: -0.006,
            log_yardline_pct: 0.001,
            touchback_goodness: -0.060,
            log_touchback_goodness: 0.096,
            yardline_tbg: -0.060,
            kick_distance_pct: 0.227,
            log_kick_distance_pct: 0.477,
            punted_to_pct: -0.233,
            log_punted_to_pct: -3.991,
        }
    }

    pub fn fair_catch_coef() -> PuntModel {
        PuntModel {
            intercept: -2.065,
            is_offense_home: -0.006,
            yardline_pct: -0.188,
            log_yardline_pct: -0.198,
            touchback_goodness: -0.167,
            log_touchback_goodness: -0.523,
            yardline_tbg: -0.198,
            kick_distance_pct: -1.547,
            log_kick_distance_pct: -2.900,
            punted_to_pct: 1.360,
            log_punted_to_pct: -0.405,
        }
    }

    pub fn fumble_lost_coef() -> PuntModel {
        PuntModel {
            intercept: -3.340,
            is_offense_home: 0.046,
            yardline_pct: -0.174,
            log_yardline_pct: -0.240,
            touchback_goodness: -0.278,
            log_touchback_goodness: -0.433,
            yardline_tbg: -0.334,
            kick_distance_pct: -0.260,
            log_kick_distance_pct: -0.580,
            punted_to_pct: 0.086,
            log_punted_to_pct: 0.115,
        }
    }

    pub fn fumble_lost_return_td_coef() -> PuntModel {
        PuntModel {
            intercept: -3.755,
            is_offense_home: -0.089,
            yardline_pct: -0.036,
            log_yardline_pct: -0.058,
            touchback_goodness: -0.089,
            log_touchback_goodness: -0.195,
            yardline_tbg: -0.074,
            kick_distance_pct: 0.006,
            log_kick_distance_pct: 0.018,
            punted_to_pct: -0.042,
            log_punted_to_pct: -0.245,
        }
    }

    pub fn punt_return_td_coef() -> PuntModel {
        PuntModel {
            intercept: -5.228,
            is_offense_home: 0.193,
            yardline_pct: 0.065,
            log_yardline_pct: 0.104,
            touchback_goodness: 0.191,
            log_touchback_goodness: 0.304,
            yardline_tbg: 0.167,
            kick_distance_pct: 0.037,
            log_kick_distance_pct: 0.078,
            punted_to_pct: 0.028,
            log_punted_to_pct: -0.114,
        }
    }

    pub fn is_pos_punt_return_yards_coef() -> PuntModel {
        PuntModel {
            intercept: 1.686,
            is_offense_home: 0.088,
            yardline_pct: 0.200,
            log_yardline_pct: 0.222,
            touchback_goodness: 0.111,
            log_touchback_goodness: -0.114,
            yardline_tbg: 0.325,
            kick_distance_pct: 0.238,
            log_kick_distance_pct: 0.280,
            punted_to_pct: -0.038,
            log_punted_to_pct: -0.578,
        }
    }

    pub fn pos_punt_return_yards_coef() -> PuntModel {
        PuntModel {
            intercept: 2.210,
            is_offense_home: -0.001,
            yardline_pct: 0.006,
            log_yardline_pct: 0.009,
            touchback_goodness: 0.011,
            log_touchback_goodness: 0.020,
            yardline_tbg: 0.012,
            kick_distance_pct: 0.010,
            log_kick_distance_pct: 0.021,
            punted_to_pct: -0.004,
            log_punted_to_pct: -0.023,
        }
    }

    pub fn neg_punt_return_yards_coef() -> PuntModel {
        PuntModel {
            intercept: 1.262,
            is_offense_home: -0.005,
            yardline_pct: -0.001,
            log_yardline_pct: -0.002,
            touchback_goodness: -0.004,
            log_touchback_goodness: -0.008,
            yardline_tbg: -0.003,
            kick_distance_pct: 0.000,
            log_kick_distance_pct: 0.000,
            punted_to_pct: -0.001,
            log_punted_to_pct: -0.008,
        }
    }
}