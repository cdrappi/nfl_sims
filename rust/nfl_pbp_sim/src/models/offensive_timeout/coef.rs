use crate::models::offensive_timeout::OffensiveTimeoutModel;

impl OffensiveTimeoutModel {
    pub fn coefs() -> OffensiveTimeoutModel {
        OffensiveTimeoutModel {
            intercept: -5.6847,
            clock_running: 0.9240,
            inv_half_minutes: 0.0947,
            log_inv_half_minutes: 1.0627,
            inv_game_minutes: -0.0984,
            log_inv_game_minutes: -0.0611,
            possession_diff: 0.0759,
            fg_possession_diff: 0.0060,
            possdiff_per_minute: 0.0508,
            fgpossdiff_per_minute: -0.0789,
            clock_runs_pdpm: -0.3176,
            clock_runs_fgpdpm: 0.3612,
            clock_runs_pdpm2: 0.0145,
            clock_runs_fgpdpm2: -0.0095,
            off_timeouts_remaining_0: 0.0000,
            off_timeouts_remaining_1: -1.6529,
            off_timeouts_remaining_2: -1.7488,
            off_timeouts_remaining_3: 3.3410,
            clock_runs_pdpm_off0to: 0.0000,
            clock_runs_pdpm_off1to: -0.0349,
            clock_runs_pdpm_off2to: -0.1279,
            clock_runs_pdpm_off3to: -0.1548,
        }
    }
}