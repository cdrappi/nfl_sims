use crate::models::offensive_timeout::OffensiveTimeoutModel;

impl OffensiveTimeoutModel {
    pub fn coefs() -> OffensiveTimeoutModel {
        OffensiveTimeoutModel {
            intercept: -5.6785,
            clock_running: 0.9224,
            inv_half_minutes: 0.0918,
            log_inv_half_minutes: 1.0639,
            inv_game_minutes: -0.1059,
            log_inv_game_minutes: -0.0608,
            possession_diff: 0.0792,
            fg_possession_diff: 0.0028,
            possdiff_per_minute: 0.0412,
            fgpossdiff_per_minute: -0.0716,
            clock_runs_pdpm: -0.3195,
            clock_runs_fgpdpm: 0.3666,
            clock_runs_pdpm2: 0.0152,
            clock_runs_fgpdpm2: -0.0100,
            off_timeouts_remaining_0: 0.0000,
            off_timeouts_remaining_1: -1.6328,
            off_timeouts_remaining_2: -1.7063,
            off_timeouts_remaining_3: 3.3467,
            clock_runs_pdpm_off0to: 0.0000,
            clock_runs_pdpm_off1to: -0.0229,
            clock_runs_pdpm_off2to: -0.1345,
            clock_runs_pdpm_off3to: -0.1621,
        }
    }
}