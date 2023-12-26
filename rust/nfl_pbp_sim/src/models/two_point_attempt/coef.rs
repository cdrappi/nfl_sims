use crate::models::two_point_attempt::TwoPointAttemptModel;

impl TwoPointAttemptModel {
    pub fn coefs() -> TwoPointAttemptModel {
        TwoPointAttemptModel {
            intercept: -0.2037,
            inv_half_minutes: 0.2957,
            log_inv_half_minutes: -0.4916,
            inv_game_minutes: -0.8025,
            log_inv_game_minutes: 1.0323,
            possession_diff: 0.3066,
            fg_possession_diff: 0.4565,
            possdiff_per_minute: -0.0262,
            fgpossdiff_per_minute: 0.3078,
            off_timeouts_remaining_0: 0.3197,
            off_timeouts_remaining_1: -0.0876,
            off_timeouts_remaining_2: -0.0568,
            off_timeouts_remaining_3: -0.1771,
            def_timeouts_remaining_0: 0.0327,
            def_timeouts_remaining_1: 0.0402,
            def_timeouts_remaining_2: 0.1032,
            def_timeouts_remaining_3: -0.1780,
            garbage_time_win: -0.7112,
            garbage_time_loss: -0.2418,
            possdiff_plus_1: -0.9369,
            possdiff_p1_per_minute: -0.0239,
            possdiff_plus_2: -0.2477,
            possdiff_p2_per_minute: -0.0261,
        }
    }
}