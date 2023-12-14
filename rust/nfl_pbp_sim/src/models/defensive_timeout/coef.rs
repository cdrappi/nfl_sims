use crate::models::defensive_timeout::DefensiveTimeoutModel;

impl DefensiveTimeoutModel {
pub fn coefs() -> DefensiveTimeoutModel {
    DefensiveTimeoutModel {
            intercept: -4.0352,
            clock_running: 1.6163,
            inv_half_minutes: -0.2866,
            log_inv_half_minutes: 1.6615,
            inv_game_minutes: -0.7124,
            log_inv_game_minutes: 0.5053,
            possession_diff: 0.1266,
            fg_possession_diff: -0.0076,
            possdiff_per_minute: -0.3902,
            fgpossdiff_per_minute: 0.3042,
            clock_runs_pdpm: 0.0515,
            clock_runs_fgpdpm: 0.8709,
            clock_runs_pdpm2: 0.1501,
            clock_runs_fgpdpm2: -0.2198,
            def_timeouts_remaining_0: 0.1090,
            def_timeouts_remaining_1: -2.0026,
            def_timeouts_remaining_2: -1.9061,
            def_timeouts_remaining_3: 3.8089,
            clock_runs_pdpm_def0to: 0.0000,
            clock_runs_pdpm_def1to: 0.0478,
            clock_runs_pdpm_def2to: 0.0971,
            clock_runs_pdpm_def3to: -0.0934,
        }
    }
}