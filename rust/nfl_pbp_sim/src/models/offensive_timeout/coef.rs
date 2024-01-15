use crate::models::offensive_timeout::OffensiveTimeoutModel;

impl OffensiveTimeoutModel {
    pub fn coefs() -> OffensiveTimeoutModel {
        OffensiveTimeoutModel {
            intercept: -5.6885,
            clock_running: 0.9210,
            inv_half_minutes: 0.0924,
            log_inv_half_minutes: 1.0657,
            inv_game_minutes: -0.1004,
            log_inv_game_minutes: -0.0599,
            possession_diff: 0.0757,
            fg_possession_diff: 0.0067,
            possdiff_per_minute: 0.0529,
            fgpossdiff_per_minute: -0.0804,
            clock_runs_pdpm: -0.3250,
            clock_runs_fgpdpm: 0.3741,
            clock_runs_pdpm2: 0.0157,
            clock_runs_fgpdpm2: -0.0111,
            off_timeouts_remaining_0: 0.0000,
            off_timeouts_remaining_1: -1.6492,
            off_timeouts_remaining_2: -1.7495,
            off_timeouts_remaining_3: 3.3516,
            clock_runs_pdpm_off0to: 0.0000,
            clock_runs_pdpm_off1to: -0.0268,
            clock_runs_pdpm_off2to: -0.1441,
            clock_runs_pdpm_off3to: -0.1541,
        }
    }
}