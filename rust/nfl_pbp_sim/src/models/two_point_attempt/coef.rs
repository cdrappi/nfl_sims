use crate::models::two_point_attempt::TwoPointAttemptModel;

impl TwoPointAttemptModel {
    pub fn coefs() -> TwoPointAttemptModel {
        TwoPointAttemptModel {
            intercept: -0.2607,
            inv_half_minutes: 0.3174,
            log_inv_half_minutes: -0.5133,
            inv_game_minutes: -0.7927,
            log_inv_game_minutes: 1.0383,
            possession_diff: 0.2623,
            fg_possession_diff: 0.4644,
            possdiff_per_minute: -0.0216,
            fgpossdiff_per_minute: 0.3026,
            off_timeouts_remaining_0: 0.2912,
            off_timeouts_remaining_1: -0.0583,
            off_timeouts_remaining_2: -0.0592,
            off_timeouts_remaining_3: -0.1745,
            def_timeouts_remaining_0: 0.0536,
            def_timeouts_remaining_1: 0.0131,
            def_timeouts_remaining_2: 0.0997,
            def_timeouts_remaining_3: -0.1672,
            garbage_time_win: -0.6831,
            garbage_time_loss: -0.2220,
            possdiff_plus_1: -0.9132,
            possdiff_p1_per_minute: -0.0230,
            possdiff_plus_2: -0.2477,
            possdiff_p2_per_minute: -0.0277,
        }
    }
}