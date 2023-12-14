use crate::models::punt::PuntModel;

impl PuntModel {

    pub fn punt_block_coef() -> PuntModel {
        PuntModel {
            intercept: -5.179,
            is_offense_home: 0.183,
            yardline_pct: 0.052,
            log_yardline_pct: 0.095,
            touchback_goodness: 0.075,
            log_touchback_goodness: 0.400,
            yardline_tbg: 0.056,
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
            intercept: -16.179,
            is_offense_home: -0.211,
            yardline_pct: -0.011,
            log_yardline_pct: -0.003,
            touchback_goodness: -0.075,
            log_touchback_goodness: 0.097,
            yardline_tbg: -0.074,
            kick_distance_pct: 0.220,
            log_kick_distance_pct: 0.469,
            punted_to_pct: -0.231,
            log_punted_to_pct: -3.956,
        }
    }

    pub fn fair_catch_coef() -> PuntModel {
        PuntModel {
            intercept: -2.032,
            is_offense_home: -0.004,
            yardline_pct: -0.187,
            log_yardline_pct: -0.201,
            touchback_goodness: -0.167,
            log_touchback_goodness: -0.538,
            yardline_tbg: -0.198,
            kick_distance_pct: -1.517,
            log_kick_distance_pct: -2.856,
            punted_to_pct: 1.330,
            log_punted_to_pct: -0.395,
        }
    }

    pub fn fumble_lost_coef() -> PuntModel {
        PuntModel {
            intercept: -3.341,
            is_offense_home: -0.001,
            yardline_pct: -0.176,
            log_yardline_pct: -0.242,
            touchback_goodness: -0.279,
            log_touchback_goodness: -0.437,
            yardline_tbg: -0.336,
            kick_distance_pct: -0.265,
            log_kick_distance_pct: -0.590,
            punted_to_pct: 0.090,
            log_punted_to_pct: 0.100,
        }
    }

    pub fn fumble_lost_return_td_coef() -> PuntModel {
        PuntModel {
            intercept: -3.726,
            is_offense_home: -0.082,
            yardline_pct: -0.035,
            log_yardline_pct: -0.057,
            touchback_goodness: -0.087,
            log_touchback_goodness: -0.191,
            yardline_tbg: -0.073,
            kick_distance_pct: 0.006,
            log_kick_distance_pct: 0.019,
            punted_to_pct: -0.042,
            log_punted_to_pct: -0.242,
        }
    }

    pub fn punt_return_td_coef() -> PuntModel {
        PuntModel {
            intercept: -5.082,
            is_offense_home: 0.198,
            yardline_pct: 0.066,
            log_yardline_pct: 0.105,
            touchback_goodness: 0.197,
            log_touchback_goodness: 0.303,
            yardline_tbg: 0.172,
            kick_distance_pct: 0.033,
            log_kick_distance_pct: 0.070,
            punted_to_pct: 0.033,
            log_punted_to_pct: -0.005,
        }
    }

    pub fn is_pos_punt_return_yards_coef() -> PuntModel {
        PuntModel {
            intercept: 1.633,
            is_offense_home: 0.096,
            yardline_pct: 0.202,
            log_yardline_pct: 0.225,
            touchback_goodness: 0.115,
            log_touchback_goodness: -0.104,
            yardline_tbg: 0.328,
            kick_distance_pct: 0.245,
            log_kick_distance_pct: 0.281,
            punted_to_pct: -0.043,
            log_punted_to_pct: -0.593,
        }
    }

    pub fn pos_punt_return_yards_coef() -> PuntModel {
        PuntModel {
            intercept: 2.209,
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