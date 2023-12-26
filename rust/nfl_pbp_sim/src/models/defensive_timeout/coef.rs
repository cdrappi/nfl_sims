use crate::models::defensive_timeout::DefensiveTimeoutModel;

impl DefensiveTimeoutModel {
pub fn coefs() -> DefensiveTimeoutModel {
    DefensiveTimeoutModel {
            intercept: -4.0666,
            clock_running: 1.6166,
            inv_half_minutes: -0.2848,
            log_inv_half_minutes: 1.6607,
            inv_game_minutes: -0.7151,
            log_inv_game_minutes: 0.5027,
            possession_diff: 0.1341,
            fg_possession_diff: -0.0165,
            possdiff_per_minute: -0.3974,
            fgpossdiff_per_minute: 0.3112,
            clock_runs_pdpm: 0.0489,
            clock_runs_fgpdpm: 0.8814,
            clock_runs_pdpm2: 0.1524,
            clock_runs_fgpdpm2: -0.2229,
            def_timeouts_remaining_0: 0.1132,
            def_timeouts_remaining_1: -2.0121,
            def_timeouts_remaining_2: -1.8923,
            def_timeouts_remaining_3: 3.8343,
            clock_runs_pdpm_def0to: 0.0000,
            clock_runs_pdpm_def1to: 0.0506,
            clock_runs_pdpm_def2to: 0.0995,
            clock_runs_pdpm_def3to: -0.1012,
        }
    }
}