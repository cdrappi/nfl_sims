use crate::models::two_point_attempt::TwoPointAttemptModel;

impl TwoPointAttemptModel {
    pub fn coefs() -> TwoPointAttemptModel {
        TwoPointAttemptModel {
            intercept: -0.1778,
            inv_half_minutes: 0.2912,
            log_inv_half_minutes: -0.4957,
            inv_game_minutes: -0.8242,
            log_inv_game_minutes: 1.0452,
            possession_diff: 0.2924,
            fg_possession_diff: 0.4691,
            possdiff_per_minute: -0.0313,
            fgpossdiff_per_minute: 0.3191,
            off_timeouts_remaining_0: 0.3621,
            off_timeouts_remaining_1: -0.0945,
            off_timeouts_remaining_2: -0.0672,
            off_timeouts_remaining_3: -0.2011,
            def_timeouts_remaining_0: 0.0179,
            def_timeouts_remaining_1: 0.0526,
            def_timeouts_remaining_2: 0.1021,
            def_timeouts_remaining_3: -0.1735,
            garbage_time_win: -0.7281,
            garbage_time_loss: -0.2556,
            possdiff_plus_1: -0.9431,
            possdiff_p1_per_minute: -0.0228,
            possdiff_plus_2: -0.2412,
            possdiff_p2_per_minute: -0.0268,
        }
    }
}