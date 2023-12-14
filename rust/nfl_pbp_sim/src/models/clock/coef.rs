use crate::models::clock::ClockModel;

impl ClockModel {

    pub fn play_duration_coefs() -> ClockModel {
        ClockModel {
            intercept: 1.7052,
            play_type_field_goal: -0.0060,
            play_type_kickoff: -0.0022,
            play_type_no_play: -0.0026,
            play_type_pass: -0.0057,
            play_type_punt: 0.0405,
            play_type_run: -0.0239,
            yards_gained_pct: 0.0039,
            yards_gained_pct_sq: 0.0021,
            play_type_pass_yards_pct: 0.0033,
            play_type_run_yards_pct: 0.0006,
            play_type_pass_yards_pct_sq: 0.0016,
            play_type_run_yards_pct_sq: 0.0005,
            clock_running: -0.0084,
            inv_half_minutes: -0.0039,
            log_inv_half_minutes: -0.0044,
            inv_game_minutes: -0.0005,
            log_inv_game_minutes: -0.0014,
            possession_diff: -0.0006,
            fg_possession_diff: -0.0017,
            possdiff_per_minute: 0.0007,
            fgpossdiff_per_minute: -0.0002,
            clock_runs_pdpm: 0.0004,
            clock_runs_fgpdpm: 0.0000,
            clock_runs_pdpm2: 0.0002,
            clock_runs_fgpdpm2: 0.0002,
            off_timeouts_remaining_0: -0.0002,
            off_timeouts_remaining_1: -0.0002,
            off_timeouts_remaining_2: 0.0010,
            off_timeouts_remaining_3: -0.0005,
            clock_runs_pdpm_off0to: -0.0000,
            clock_runs_pdpm_off1to: 0.0001,
            clock_runs_pdpm_off2to: 0.0002,
            clock_runs_pdpm_off3to: 0.0002,
            def_timeouts_remaining_0: 0.0003,
            def_timeouts_remaining_1: 0.0001,
            def_timeouts_remaining_2: 0.0005,
            def_timeouts_remaining_3: -0.0009,
            clock_runs_pdpm_def0to: 0.0000,
            clock_runs_pdpm_def1to: -0.0000,
            clock_runs_pdpm_def2to: 0.0001,
            clock_runs_pdpm_def3to: 0.0003,
            play_duration: 0.0,
            pace_z: 0.0,
        }
    }


    pub fn time_to_spot_coefs() -> ClockModel {
        ClockModel {
            intercept: 4.5417,
            play_type_field_goal: 0.0000,
            play_type_kickoff: 0.0000,
            play_type_no_play: 0.0000,
            play_type_pass: 0.1392,
            play_type_punt: 0.0000,
            play_type_run: -0.1392,
            yards_gained_pct: -0.4448,
            yards_gained_pct_sq: 0.4214,
            play_type_pass_yards_pct: 0.2902,
            play_type_run_yards_pct: -0.7350,
            play_type_pass_yards_pct_sq: 0.5102,
            play_type_run_yards_pct_sq: -0.0888,
            clock_running: 0.0107,
            inv_half_minutes: 0.1595,
            log_inv_half_minutes: 0.0518,
            inv_game_minutes: 0.0514,
            log_inv_game_minutes: 0.0452,
            possession_diff: 0.0868,
            fg_possession_diff: -0.0711,
            possdiff_per_minute: -0.2634,
            fgpossdiff_per_minute: -0.2404,
            clock_runs_pdpm: -0.0550,
            clock_runs_fgpdpm: 0.0820,
            clock_runs_pdpm2: -0.2560,
            clock_runs_fgpdpm2: -0.1967,
            off_timeouts_remaining_0: -0.1774,
            off_timeouts_remaining_1: 0.3458,
            off_timeouts_remaining_2: 0.0036,
            off_timeouts_remaining_3: -0.1720,
            clock_runs_pdpm_off0to: 0.1127,
            clock_runs_pdpm_off1to: 0.0385,
            clock_runs_pdpm_off2to: -0.2125,
            clock_runs_pdpm_off3to: 0.0064,
            def_timeouts_remaining_0: 0.2799,
            def_timeouts_remaining_1: 0.0072,
            def_timeouts_remaining_2: -0.1572,
            def_timeouts_remaining_3: -0.1299,
            clock_runs_pdpm_def0to: -0.0280,
            clock_runs_pdpm_def1to: -0.0891,
            clock_runs_pdpm_def2to: -0.1695,
            clock_runs_pdpm_def3to: 0.2317,
            play_duration: 0.0127,
            pace_z: 0.0,
        }
    }


    pub fn paused_next_play_clock_coefs() -> ClockModel {
        ClockModel {
            intercept: 6.3175,
            play_type_field_goal: 0.0000,
            play_type_kickoff: 0.0000,
            play_type_no_play: 0.0000,
            play_type_pass: 0.8470,
            play_type_punt: 0.0000,
            play_type_run: -0.8470,
            yards_gained_pct: 0.6833,
            yards_gained_pct_sq: -3.1526,
            play_type_pass_yards_pct: 0.4800,
            play_type_run_yards_pct: 0.2034,
            play_type_pass_yards_pct_sq: -2.0423,
            play_type_run_yards_pct_sq: -1.1103,
            clock_running: -0.5424,
            inv_half_minutes: 9.3082,
            log_inv_half_minutes: -0.6647,
            inv_game_minutes: 1.4666,
            log_inv_game_minutes: -0.1844,
            possession_diff: -0.4674,
            fg_possession_diff: 0.5106,
            possdiff_per_minute: -5.1136,
            fgpossdiff_per_minute: -3.5456,
            clock_runs_pdpm: -0.0742,
            clock_runs_fgpdpm: 0.0460,
            clock_runs_pdpm2: 0.2387,
            clock_runs_fgpdpm2: 0.7184,
            off_timeouts_remaining_0: 0.0031,
            off_timeouts_remaining_1: -0.4488,
            off_timeouts_remaining_2: 0.0526,
            off_timeouts_remaining_3: 0.3932,
            clock_runs_pdpm_off0to: 0.0522,
            clock_runs_pdpm_off1to: -0.1918,
            clock_runs_pdpm_off2to: 0.7010,
            clock_runs_pdpm_off3to: -0.6356,
            def_timeouts_remaining_0: 0.6857,
            def_timeouts_remaining_1: 0.6495,
            def_timeouts_remaining_2: -0.4584,
            def_timeouts_remaining_3: -0.8768,
            clock_runs_pdpm_def0to: 0.2010,
            clock_runs_pdpm_def1to: -0.5981,
            clock_runs_pdpm_def2to: 0.3086,
            clock_runs_pdpm_def3to: 0.0143,
            play_duration: -0.1272,
            pace_z: 2.2119,
        }
    }


    pub fn running_next_play_clock_coefs() -> ClockModel {
        ClockModel {
            intercept: 14.5647,
            play_type_field_goal: 0.0000,
            play_type_kickoff: 0.0000,
            play_type_no_play: 0.0000,
            play_type_pass: 0.5747,
            play_type_punt: 0.0000,
            play_type_run: -0.5747,
            yards_gained_pct: 8.6846,
            yards_gained_pct_sq: -14.0173,
            play_type_pass_yards_pct: 2.6098,
            play_type_run_yards_pct: 6.0748,
            play_type_pass_yards_pct_sq: -7.2462,
            play_type_run_yards_pct_sq: -6.7711,
            clock_running: -1.0372,
            inv_half_minutes: -0.2490,
            log_inv_half_minutes: 1.2352,
            inv_game_minutes: -1.6732,
            log_inv_game_minutes: 0.1466,
            possession_diff: -0.4867,
            fg_possession_diff: -0.0151,
            possdiff_per_minute: -1.0870,
            fgpossdiff_per_minute: -0.8840,
            clock_runs_pdpm: -2.4251,
            clock_runs_fgpdpm: 1.2625,
            clock_runs_pdpm2: 0.5904,
            clock_runs_fgpdpm2: 0.0563,
            off_timeouts_remaining_0: 3.1770,
            off_timeouts_remaining_1: -0.2131,
            off_timeouts_remaining_2: -1.6578,
            off_timeouts_remaining_3: -1.3061,
            clock_runs_pdpm_off0to: -0.5797,
            clock_runs_pdpm_off1to: -1.1171,
            clock_runs_pdpm_off2to: -0.4550,
            clock_runs_pdpm_off3to: -0.2734,
            def_timeouts_remaining_0: -1.2486,
            def_timeouts_remaining_1: 0.7454,
            def_timeouts_remaining_2: 0.1774,
            def_timeouts_remaining_3: 0.3257,
            clock_runs_pdpm_def0to: 0.2125,
            clock_runs_pdpm_def1to: -0.7308,
            clock_runs_pdpm_def2to: -0.6597,
            clock_runs_pdpm_def3to: -1.2472,
            play_duration: -0.0607,
            pace_z: 2.4783,
        }
    }

}