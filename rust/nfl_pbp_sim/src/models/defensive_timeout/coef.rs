use crate::models::defensive_timeout::DefensiveTimeoutModel;

impl DefensiveTimeoutModel {
pub fn coefs() -> DefensiveTimeoutModel {
    DefensiveTimeoutModel {
            intercept: -4.0643,
            clock_running: 1.6152,
            inv_half_minutes: -0.2815,
            log_inv_half_minutes: 1.6586,
            inv_game_minutes: -0.7377,
            log_inv_game_minutes: 0.5081,
            possession_diff: 0.1279,
            fg_possession_diff: -0.0101,
            possdiff_per_minute: -0.4088,
            fgpossdiff_per_minute: 0.3224,
            clock_runs_pdpm: 0.1078,
            clock_runs_fgpdpm: 0.8117,
            clock_runs_pdpm2: 0.1279,
            clock_runs_fgpdpm2: -0.2000,
            def_timeouts_remaining_0: 0.1122,
            def_timeouts_remaining_1: -2.0356,
            def_timeouts_remaining_2: -1.9136,
            def_timeouts_remaining_3: 3.8422,
            clock_runs_pdpm_def0to: 0.0000,
            clock_runs_pdpm_def1to: 0.0773,
            clock_runs_pdpm_def2to: 0.1284,
            clock_runs_pdpm_def3to: -0.0979,
        }
    }
}