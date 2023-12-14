use crate::models::offensive_timeout::OffensiveTimeoutModel;

impl OffensiveTimeoutModel {
    pub fn coefs() -> OffensiveTimeoutModel {
        OffensiveTimeoutModel {
            intercept: -5.6556,
            clock_running: 0.9233,
            inv_half_minutes: 0.0935,
            log_inv_half_minutes: 1.0637,
            inv_game_minutes: -0.1043,
            log_inv_game_minutes: -0.0572,
            possession_diff: 0.0784,
            fg_possession_diff: 0.0034,
            possdiff_per_minute: 0.0441,
            fgpossdiff_per_minute: -0.0736,
            clock_runs_pdpm: -0.3192,
            clock_runs_fgpdpm: 0.3612,
            clock_runs_pdpm2: 0.0153,
            clock_runs_fgpdpm2: -0.0101,
            off_timeouts_remaining_0: 0.0000,
            off_timeouts_remaining_1: -1.6604,
            off_timeouts_remaining_2: -1.7316,
            off_timeouts_remaining_3: 3.3304,
            clock_runs_pdpm_off0to: 0.0000,
            clock_runs_pdpm_off1to: -0.0207,
            clock_runs_pdpm_off2to: -0.1400,
            clock_runs_pdpm_off3to: -0.1585,
        }
    }
}