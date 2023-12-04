use crate::models::punt::PuntModel;

impl PuntModel {

    pub fn punt_block_coef() -> PuntModel {
        PuntModel {
            intercept: -5.167,
            is_offense_home: 0.182,
            yardline_pct: 0.052,
            log_yardline_pct: 0.095,
            touchback_goodness: 0.075,
            log_touchback_goodness: 0.400,
            yardline_tbg: 0.055,
            kick_distance_pct: 0.0,
            log_kick_distance_pct: 0.0,
            punted_to_pct: 0.0,
            log_punted_to_pct: 0.0,
        }
    }


    pub fn punt_distance_coef() -> PuntModel {
        PuntModel {
            intercept: 3.862,
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
            intercept: -16.125,
            is_offense_home: -0.212,
            yardline_pct: -0.014,
            log_yardline_pct: -0.006,
            touchback_goodness: -0.080,
            log_touchback_goodness: 0.100,
            yardline_tbg: -0.082,
            kick_distance_pct: 0.216,
            log_kick_distance_pct: 0.465,
            punted_to_pct: -0.230,
            log_punted_to_pct: -3.943,
        }
    }

    pub fn fair_catch_coef() -> PuntModel {
        PuntModel {
            intercept: -2.119,
            is_offense_home: -0.001,
            yardline_pct: -0.197,
            log_yardline_pct: -0.229,
            touchback_goodness: -0.109,
            log_touchback_goodness: -0.581,
            yardline_tbg: -0.172,
            kick_distance_pct: -1.503,
            log_kick_distance_pct: -2.873,
            punted_to_pct: 1.306,
            log_punted_to_pct: -0.394,
        }
    }

    pub fn fumble_lost_coef() -> PuntModel {
        PuntModel {
            intercept: -3.387,
            is_offense_home: -0.024,
            yardline_pct: -0.169,
            log_yardline_pct: -0.240,
            touchback_goodness: -0.252,
            log_touchback_goodness: -0.468,
            yardline_tbg: -0.307,
            kick_distance_pct: -0.260,
            log_kick_distance_pct: -0.596,
            punted_to_pct: 0.091,
            log_punted_to_pct: 0.107,
        }
    }

    pub fn fumble_lost_return_td_coef() -> PuntModel {
        PuntModel {
            intercept: -3.704,
            is_offense_home: -0.078,
            yardline_pct: -0.036,
            log_yardline_pct: -0.058,
            touchback_goodness: -0.090,
            log_touchback_goodness: -0.195,
            yardline_tbg: -0.075,
            kick_distance_pct: 0.006,
            log_kick_distance_pct: 0.018,
            punted_to_pct: -0.042,
            log_punted_to_pct: -0.244,
        }
    }

    pub fn punt_return_td_coef() -> PuntModel {
        PuntModel {
            intercept: -5.121,
            is_offense_home: 0.224,
            yardline_pct: 0.067,
            log_yardline_pct: 0.106,
            touchback_goodness: 0.194,
            log_touchback_goodness: 0.296,
            yardline_tbg: 0.172,
            kick_distance_pct: 0.033,
            log_kick_distance_pct: 0.070,
            punted_to_pct: 0.034,
            log_punted_to_pct: -0.012,
        }
    }

    pub fn is_pos_punt_return_yards_coef() -> PuntModel {
        PuntModel {
            intercept: 1.635,
            is_offense_home: 0.097,
            yardline_pct: 0.202,
            log_yardline_pct: 0.228,
            touchback_goodness: 0.111,
            log_touchback_goodness: -0.090,
            yardline_tbg: 0.325,
            kick_distance_pct: 0.244,
            log_kick_distance_pct: 0.282,
            punted_to_pct: -0.041,
            log_punted_to_pct: -0.590,
        }
    }

    pub fn pos_punt_return_yards_coef() -> PuntModel {
        PuntModel {
            intercept: 2.208,
            is_offense_home: -0.001,
            yardline_pct: 0.006,
            log_yardline_pct: 0.009,
            touchback_goodness: 0.011,
            log_touchback_goodness: 0.020,
            yardline_tbg: 0.012,
            kick_distance_pct: 0.010,
            log_kick_distance_pct: 0.021,
            punted_to_pct: -0.004,
            log_punted_to_pct: -0.022,
        }
    }

    pub fn neg_punt_return_yards_coef() -> PuntModel {
        PuntModel {
            intercept: 1.263,
            is_offense_home: -0.005,
            yardline_pct: -0.001,
            log_yardline_pct: -0.002,
            touchback_goodness: -0.003,
            log_touchback_goodness: -0.007,
            yardline_tbg: -0.003,
            kick_distance_pct: 0.000,
            log_kick_distance_pct: 0.000,
            punted_to_pct: -0.001,
            log_punted_to_pct: -0.009,
        }
    }
}