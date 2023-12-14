use crate::models::two_point_attempt::TwoPointAttemptModel;

impl TwoPointAttemptModel {
    pub fn coefs() -> TwoPointAttemptModel {
        TwoPointAttemptModel {
            intercept: -0.2105,
            inv_half_minutes: 0.3066,
            log_inv_half_minutes: -0.4984,
            inv_game_minutes: -0.8008,
            log_inv_game_minutes: 1.0410,
            possession_diff: 0.2771,
            fg_possession_diff: 0.4594,
            possdiff_per_minute: -0.0206,
            fgpossdiff_per_minute: 0.3031,
            off_timeouts_remaining_0: 0.2959,
            off_timeouts_remaining_1: -0.0788,
            off_timeouts_remaining_2: -0.0494,
            off_timeouts_remaining_3: -0.1688,
            def_timeouts_remaining_0: 0.0326,
            def_timeouts_remaining_1: 0.0398,
            def_timeouts_remaining_2: 0.1009,
            def_timeouts_remaining_3: -0.1744,
            garbage_time_win: -0.6986,
            garbage_time_loss: -0.2323,
            possdiff_plus_1: -0.9180,
            possdiff_p1_per_minute: -0.0238,
            possdiff_plus_2: -0.2524,
            possdiff_p2_per_minute: -0.0266,
        }
    }
}