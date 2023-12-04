use crate::models::defensive_timeout::DefensiveTimeoutModel;

impl DefensiveTimeoutModel {
pub fn coefs() -> DefensiveTimeoutModel {
    DefensiveTimeoutModel {
            intercept: -4.0196,
            clock_running: 1.6116,
            inv_half_minutes: -0.2828,
            log_inv_half_minutes: 1.6571,
            inv_game_minutes: -0.7152,
            log_inv_game_minutes: 0.5072,
            possession_diff: 0.1253,
            fg_possession_diff: -0.0057,
            possdiff_per_minute: -0.3870,
            fgpossdiff_per_minute: 0.3013,
            clock_runs_pdpm: 0.0517,
            clock_runs_fgpdpm: 0.8667,
            clock_runs_pdpm2: 0.1494,
            clock_runs_fgpdpm2: -0.2188,
            def_timeouts_remaining_0: 0.1098,
            def_timeouts_remaining_1: -2.0018,
            def_timeouts_remaining_2: -1.8987,
            def_timeouts_remaining_3: 3.7943,
            clock_runs_pdpm_def0to: 0.0000,
            clock_runs_pdpm_def1to: 0.0486,
            clock_runs_pdpm_def2to: 0.0960,
            clock_runs_pdpm_def3to: -0.0929,
        }
    }
}