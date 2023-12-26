use crate::models::punt::PuntModel;

impl PuntModel {

    pub fn punt_block_coef() -> PuntModel {
        PuntModel {
            intercept: -5.187,
            is_offense_home: 0.182,
            yardline_pct: 0.061,
            log_yardline_pct: 0.107,
            touchback_goodness: 0.086,
            log_touchback_goodness: 0.406,
            yardline_tbg: 0.074,
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
            intercept: -16.256,
            is_offense_home: -0.202,
            yardline_pct: -0.009,
            log_yardline_pct: -0.001,
            touchback_goodness: -0.071,
            log_touchback_goodness: 0.089,
            yardline_tbg: -0.068,
            kick_distance_pct: 0.224,
            log_kick_distance_pct: 0.473,
            punted_to_pct: -0.232,
            log_punted_to_pct: -3.969,
        }
    }

    pub fn fair_catch_coef() -> PuntModel {
        PuntModel {
            intercept: -2.058,
            is_offense_home: -0.007,
            yardline_pct: -0.176,
            log_yardline_pct: -0.184,
            touchback_goodness: -0.182,
            log_touchback_goodness: -0.531,
            yardline_tbg: -0.193,
            kick_distance_pct: -1.534,
            log_kick_distance_pct: -2.882,
            punted_to_pct: 1.358,
            log_punted_to_pct: -0.408,
        }
    }

    pub fn fumble_lost_coef() -> PuntModel {
        PuntModel {
            intercept: -3.332,
            is_offense_home: 0.039,
            yardline_pct: -0.174,
            log_yardline_pct: -0.239,
            touchback_goodness: -0.284,
            log_touchback_goodness: -0.433,
            yardline_tbg: -0.337,
            kick_distance_pct: -0.262,
            log_kick_distance_pct: -0.582,
            punted_to_pct: 0.088,
            log_punted_to_pct: 0.108,
        }
    }

    pub fn fumble_lost_return_td_coef() -> PuntModel {
        PuntModel {
            intercept: -3.749,
            is_offense_home: -0.088,
            yardline_pct: -0.036,
            log_yardline_pct: -0.057,
            touchback_goodness: -0.088,
            log_touchback_goodness: -0.194,
            yardline_tbg: -0.074,
            kick_distance_pct: 0.006,
            log_kick_distance_pct: 0.018,
            punted_to_pct: -0.042,
            log_punted_to_pct: -0.244,
        }
    }

    pub fn punt_return_td_coef() -> PuntModel {
        PuntModel {
            intercept: -5.100,
            is_offense_home: 0.199,
            yardline_pct: 0.066,
            log_yardline_pct: 0.105,
            touchback_goodness: 0.197,
            log_touchback_goodness: 0.302,
            yardline_tbg: 0.172,
            kick_distance_pct: 0.032,
            log_kick_distance_pct: 0.069,
            punted_to_pct: 0.034,
            log_punted_to_pct: -0.004,
        }
    }

    pub fn is_pos_punt_return_yards_coef() -> PuntModel {
        PuntModel {
            intercept: 1.643,
            is_offense_home: 0.079,
            yardline_pct: 0.206,
            log_yardline_pct: 0.229,
            touchback_goodness: 0.119,
            log_touchback_goodness: -0.108,
            yardline_tbg: 0.336,
            kick_distance_pct: 0.246,
            log_kick_distance_pct: 0.282,
            punted_to_pct: -0.040,
            log_punted_to_pct: -0.590,
        }
    }

    pub fn pos_punt_return_yards_coef() -> PuntModel {
        PuntModel {
            intercept: 2.210,
            is_offense_home: -0.001,
            yardline_pct: 0.006,
            log_yardline_pct: 0.009,
            touchback_goodness: 0.012,
            log_touchback_goodness: 0.021,
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