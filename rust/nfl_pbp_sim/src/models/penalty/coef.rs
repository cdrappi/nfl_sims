use crate::models::penalty::PenaltyModel;

impl PenaltyModel {

    pub fn offensive_loss_of_down_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: -9.485,
            is_offense_home: 0.096,
            offense_log_pass_prob: -0.017,
            defense_log_pass_prob: 0.007,
            off_def_lpp: 0.004,
            off_lpp_rz: -0.022,
            def_lpp_rz: 0.006,
            off_def_lpp_rz: 0.008,
            off_lpp_outside_rz: -0.050,
            off_lpp_inside_rz: 0.035,
            def_lpp_outside_rz: -0.027,
            def_lpp_inside_rz: 0.035,
            off_lpp_pdpm: -0.006,
            def_lpp_pdpm: -0.007,
            off_lpp_rz_pdpm: -0.006,
            def_lpp_rz_pdpm: -0.006,
            down_1: -0.076,
            down_2: 0.034,
            down_3: 0.050,
            down_4: -0.008,
            goal_to_go: -0.020,
            z_ydstogo: -0.021,
            ydstogo_pct: -0.011,
            log_ydstogo_pct: 0.022,
            to_go_1st: -0.030,
            to_go_2nd: 0.016,
            to_go_3rd: 0.017,
            to_go_4th: -0.005,
            log_to_go_1st: 0.007,
            log_to_go_2nd: -0.005,
            log_to_go_3rd: 0.012,
            log_to_go_4th: 0.008,
            fp_1st: -0.027,
            fp_2nd: 0.014,
            fp_3rd: 0.022,
            fp_4th: -0.002,
            yardline_fgsig_4th: 0.012,
            yardline_puntsig_4th: 0.091,
            yardline_pct: 0.012,
            yardline_pct_sq: 0.003,
            log_yardline_pct: 0.081,
            fg_sigmoid: 0.079,
            punt_sigmoid: -0.206,
            goal_to_go_yardline: -0.001,
            log_goal_to_go_yardline: 0.061,
            yards_to_go_yardline: 0.013,
            log_yards_to_go_yardline: 0.021,
            yardline_4th: -0.003,
            log_yardline_4th: 0.012,
            yardline_not_4th: 0.015,
            log_yardline_not_4th: 0.070,
            inside_2m_warning: -0.032,
            garbage_time_win: -0.002,
            garbage_time_loss: -0.004,
            clock_running: 0.075,
            inv_half_minutes: -0.032,
            log_inv_half_minutes: 0.017,
            inv_game_minutes: 0.004,
            log_inv_game_minutes: 0.168,
            possession_diff: -0.104,
            fg_possession_diff: -0.013,
            possdiff_per_minute: 0.013,
            fgpossdiff_per_minute: 0.038,
            clock_runs_pdpm: -0.007,
            clock_runs_fgpdpm: 0.029,
            clock_runs_pdpm2: -0.001,
            clock_runs_fgpdpm2: -0.004,
            off_timeouts_remaining_0: -0.010,
            off_timeouts_remaining_1: -0.014,
            off_timeouts_remaining_2: 0.054,
            off_timeouts_remaining_3: -0.029,
            clock_runs_pdpm_off0to: 0.003,
            clock_runs_pdpm_off1to: 0.000,
            clock_runs_pdpm_off2to: -0.000,
            clock_runs_pdpm_off3to: -0.009,
            def_timeouts_remaining_0: -0.006,
            def_timeouts_remaining_1: -0.014,
            def_timeouts_remaining_2: 0.143,
            def_timeouts_remaining_3: -0.123,
            clock_runs_pdpm_def0to: -0.004,
            clock_runs_pdpm_def1to: 0.001,
            clock_runs_pdpm_def2to: -0.008,
            clock_runs_pdpm_def3to: 0.004,
            offense_penalty_z: 0.089,
            defense_penalty_z: -0.200,
            off_def_penalty_z: -0.048,
        }
    }


    pub fn defensive_automatic_first_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: 1.066,
            is_offense_home: 0.056,
            offense_log_pass_prob: 0.247,
            defense_log_pass_prob: -0.210,
            off_def_lpp: -0.038,
            off_lpp_rz: -0.116,
            def_lpp_rz: 0.177,
            off_def_lpp_rz: -0.060,
            off_lpp_outside_rz: 0.315,
            off_lpp_inside_rz: 0.019,
            def_lpp_outside_rz: -0.165,
            def_lpp_inside_rz: 0.120,
            off_lpp_pdpm: -0.262,
            def_lpp_pdpm: 0.114,
            off_lpp_rz_pdpm: 0.325,
            def_lpp_rz_pdpm: -0.008,
            down_1: 0.017,
            down_2: 0.117,
            down_3: 0.074,
            down_4: -0.211,
            goal_to_go: -0.030,
            z_ydstogo: -0.362,
            ydstogo_pct: -0.184,
            log_ydstogo_pct: 0.662,
            to_go_1st: -0.117,
            to_go_2nd: 0.081,
            to_go_3rd: 0.214,
            to_go_4th: -0.027,
            log_to_go_1st: 0.433,
            log_to_go_2nd: 0.204,
            log_to_go_3rd: 0.020,
            log_to_go_4th: 0.005,
            fp_1st: -0.016,
            fp_2nd: 0.020,
            fp_3rd: 0.104,
            fp_4th: -0.111,
            yardline_fgsig_4th: 0.041,
            yardline_puntsig_4th: -0.083,
            yardline_pct: 0.085,
            yardline_pct_sq: 0.036,
            log_yardline_pct: -0.013,
            fg_sigmoid: -0.012,
            punt_sigmoid: 0.007,
            goal_to_go_yardline: -0.024,
            log_goal_to_go_yardline: -0.066,
            yards_to_go_yardline: 0.109,
            log_yards_to_go_yardline: 0.052,
            yardline_4th: -0.058,
            log_yardline_4th: 0.353,
            yardline_not_4th: 0.143,
            log_yardline_not_4th: -0.366,
            inside_2m_warning: 0.117,
            garbage_time_win: -0.573,
            garbage_time_loss: -0.280,
            clock_running: -0.072,
            inv_half_minutes: -0.042,
            log_inv_half_minutes: -0.034,
            inv_game_minutes: -0.391,
            log_inv_game_minutes: 0.212,
            possession_diff: 0.016,
            fg_possession_diff: -0.062,
            possdiff_per_minute: -0.097,
            fgpossdiff_per_minute: 0.028,
            clock_runs_pdpm: -0.227,
            clock_runs_fgpdpm: -0.136,
            clock_runs_pdpm2: 0.169,
            clock_runs_fgpdpm2: -0.165,
            off_timeouts_remaining_0: -0.164,
            off_timeouts_remaining_1: 0.134,
            off_timeouts_remaining_2: 0.024,
            off_timeouts_remaining_3: 0.004,
            clock_runs_pdpm_off0to: 0.082,
            clock_runs_pdpm_off1to: -0.199,
            clock_runs_pdpm_off2to: 0.046,
            clock_runs_pdpm_off3to: -0.156,
            def_timeouts_remaining_0: -0.226,
            def_timeouts_remaining_1: 0.063,
            def_timeouts_remaining_2: 0.120,
            def_timeouts_remaining_3: 0.040,
            clock_runs_pdpm_def0to: -0.220,
            clock_runs_pdpm_def1to: -0.055,
            clock_runs_pdpm_def2to: 0.025,
            clock_runs_pdpm_def3to: 0.022,
            offense_penalty_z: -0.007,
            defense_penalty_z: 0.095,
            off_def_penalty_z: -0.022,
        }
    }


    pub fn offensive_5_yards_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: -0.526,
            is_offense_home: -0.106,
            offense_log_pass_prob: 0.048,
            defense_log_pass_prob: -0.005,
            off_def_lpp: -0.027,
            off_lpp_rz: -0.277,
            def_lpp_rz: -0.215,
            off_def_lpp_rz: 0.306,
            off_lpp_outside_rz: 0.234,
            off_lpp_inside_rz: 0.164,
            def_lpp_outside_rz: 0.025,
            def_lpp_inside_rz: 0.204,
            off_lpp_pdpm: 0.144,
            def_lpp_pdpm: -0.117,
            off_lpp_rz_pdpm: -0.076,
            def_lpp_rz_pdpm: -0.151,
            down_1: -0.569,
            down_2: -0.470,
            down_3: 0.255,
            down_4: 0.782,
            goal_to_go: -0.054,
            z_ydstogo: 0.230,
            ydstogo_pct: 0.112,
            log_ydstogo_pct: -0.283,
            to_go_1st: -0.182,
            to_go_2nd: -0.217,
            to_go_3rd: -0.072,
            to_go_4th: 0.395,
            log_to_go_1st: -0.220,
            log_to_go_2nd: 0.006,
            log_to_go_3rd: 0.175,
            log_to_go_4th: -0.244,
            fp_1st: -0.320,
            fp_2nd: -0.131,
            fp_3rd: 0.183,
            fp_4th: 0.258,
            yardline_fgsig_4th: -0.018,
            yardline_puntsig_4th: -0.032,
            yardline_pct: 0.273,
            yardline_pct_sq: 0.459,
            log_yardline_pct: -0.123,
            fg_sigmoid: -0.000,
            punt_sigmoid: -0.004,
            goal_to_go_yardline: -0.001,
            log_goal_to_go_yardline: -0.083,
            yards_to_go_yardline: 0.274,
            log_yards_to_go_yardline: -0.040,
            yardline_4th: 0.308,
            log_yardline_4th: -0.048,
            yardline_not_4th: -0.035,
            log_yardline_not_4th: -0.075,
            inside_2m_warning: -0.063,
            garbage_time_win: 0.130,
            garbage_time_loss: 0.305,
            clock_running: 0.065,
            inv_half_minutes: 0.037,
            log_inv_half_minutes: 0.037,
            inv_game_minutes: 0.639,
            log_inv_game_minutes: -0.158,
            possession_diff: 0.011,
            fg_possession_diff: -0.027,
            possdiff_per_minute: 0.193,
            fgpossdiff_per_minute: -0.060,
            clock_runs_pdpm: -0.024,
            clock_runs_fgpdpm: 0.054,
            clock_runs_pdpm2: 0.022,
            clock_runs_fgpdpm2: -0.001,
            off_timeouts_remaining_0: 0.329,
            off_timeouts_remaining_1: -0.061,
            off_timeouts_remaining_2: -0.175,
            off_timeouts_remaining_3: -0.095,
            clock_runs_pdpm_off0to: -0.110,
            clock_runs_pdpm_off1to: -0.117,
            clock_runs_pdpm_off2to: -0.021,
            clock_runs_pdpm_off3to: 0.222,
            def_timeouts_remaining_0: 0.035,
            def_timeouts_remaining_1: 0.054,
            def_timeouts_remaining_2: -0.086,
            def_timeouts_remaining_3: -0.006,
            clock_runs_pdpm_def0to: 0.192,
            clock_runs_pdpm_def1to: 0.126,
            clock_runs_pdpm_def2to: -0.178,
            clock_runs_pdpm_def3to: -0.165,
            offense_penalty_z: -0.010,
            defense_penalty_z: 0.015,
            off_def_penalty_z: -0.057,
        }
    }


    pub fn offensive_15_yards_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: -4.000,
            is_offense_home: -0.065,
            offense_log_pass_prob: -0.087,
            defense_log_pass_prob: 0.046,
            off_def_lpp: 0.017,
            off_lpp_rz: -0.173,
            def_lpp_rz: -0.093,
            off_def_lpp_rz: 0.120,
            off_lpp_outside_rz: -0.249,
            off_lpp_inside_rz: 0.116,
            def_lpp_outside_rz: -0.084,
            def_lpp_inside_rz: 0.047,
            off_lpp_pdpm: -0.071,
            def_lpp_pdpm: -0.045,
            off_lpp_rz_pdpm: -0.045,
            def_lpp_rz_pdpm: -0.046,
            down_1: 0.006,
            down_2: 0.084,
            down_3: -0.042,
            down_4: -0.047,
            goal_to_go: -0.144,
            z_ydstogo: 0.107,
            ydstogo_pct: 0.055,
            log_ydstogo_pct: -0.053,
            to_go_1st: -0.017,
            to_go_2nd: 0.079,
            to_go_3rd: -0.015,
            to_go_4th: -0.056,
            log_to_go_1st: 0.020,
            log_to_go_2nd: -0.105,
            log_to_go_3rd: -0.104,
            log_to_go_4th: 0.136,
            fp_1st: -0.010,
            fp_2nd: 0.032,
            fp_3rd: -0.024,
            fp_4th: 0.002,
            yardline_fgsig_4th: -0.091,
            yardline_puntsig_4th: -0.092,
            yardline_pct: 0.055,
            yardline_pct_sq: 0.038,
            log_yardline_pct: -0.092,
            fg_sigmoid: 0.026,
            punt_sigmoid: 0.014,
            goal_to_go_yardline: -0.025,
            log_goal_to_go_yardline: -0.056,
            yards_to_go_yardline: 0.080,
            log_yards_to_go_yardline: -0.035,
            yardline_4th: -0.009,
            log_yardline_4th: -0.046,
            yardline_not_4th: 0.064,
            log_yardline_not_4th: -0.046,
            inside_2m_warning: -0.014,
            garbage_time_win: -0.060,
            garbage_time_loss: -0.121,
            clock_running: 0.355,
            inv_half_minutes: -0.007,
            log_inv_half_minutes: 0.060,
            inv_game_minutes: 0.033,
            log_inv_game_minutes: -0.219,
            possession_diff: 0.101,
            fg_possession_diff: -0.088,
            possdiff_per_minute: 0.059,
            fgpossdiff_per_minute: -0.063,
            clock_runs_pdpm: -0.015,
            clock_runs_fgpdpm: -0.113,
            clock_runs_pdpm2: 0.077,
            clock_runs_fgpdpm2: -0.012,
            off_timeouts_remaining_0: 0.158,
            off_timeouts_remaining_1: -0.282,
            off_timeouts_remaining_2: -0.073,
            off_timeouts_remaining_3: 0.198,
            clock_runs_pdpm_off0to: 0.026,
            clock_runs_pdpm_off1to: 0.020,
            clock_runs_pdpm_off2to: -0.053,
            clock_runs_pdpm_off3to: -0.007,
            def_timeouts_remaining_0: 0.060,
            def_timeouts_remaining_1: 0.069,
            def_timeouts_remaining_2: 0.057,
            def_timeouts_remaining_3: -0.184,
            clock_runs_pdpm_def0to: 0.001,
            clock_runs_pdpm_def1to: 0.005,
            clock_runs_pdpm_def2to: -0.123,
            clock_runs_pdpm_def3to: 0.103,
            offense_penalty_z: 0.013,
            defense_penalty_z: -0.017,
            off_def_penalty_z: -0.041,
        }
    }


    pub fn defensive_5_yards_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: -0.386,
            is_offense_home: 0.062,
            offense_log_pass_prob: 0.131,
            defense_log_pass_prob: -0.238,
            off_def_lpp: 0.059,
            off_lpp_rz: 0.060,
            def_lpp_rz: 0.011,
            off_def_lpp_rz: -0.041,
            off_lpp_outside_rz: 0.087,
            off_lpp_inside_rz: -0.067,
            def_lpp_outside_rz: -0.202,
            def_lpp_inside_rz: 0.065,
            off_lpp_pdpm: 0.132,
            def_lpp_pdpm: 0.011,
            off_lpp_rz_pdpm: -0.029,
            def_lpp_rz_pdpm: -0.065,
            down_1: 0.039,
            down_2: -0.068,
            down_3: 0.011,
            down_4: 0.016,
            goal_to_go: 0.121,
            z_ydstogo: -0.218,
            ydstogo_pct: -0.110,
            log_ydstogo_pct: 0.292,
            to_go_1st: -0.083,
            to_go_2nd: -0.014,
            to_go_3rd: -0.027,
            to_go_4th: 0.065,
            log_to_go_1st: 0.307,
            log_to_go_2nd: 0.144,
            log_to_go_3rd: 0.069,
            log_to_go_4th: -0.228,
            fp_1st: 0.031,
            fp_2nd: 0.092,
            fp_3rd: 0.029,
            fp_4th: -0.035,
            yardline_fgsig_4th: 0.075,
            yardline_puntsig_4th: 0.020,
            yardline_pct: -0.054,
            yardline_pct_sq: -0.146,
            log_yardline_pct: 0.107,
            fg_sigmoid: -0.013,
            punt_sigmoid: -0.009,
            goal_to_go_yardline: 0.018,
            log_goal_to_go_yardline: 0.013,
            yards_to_go_yardline: -0.072,
            log_yards_to_go_yardline: 0.094,
            yardline_4th: 0.021,
            log_yardline_4th: 0.145,
            yardline_not_4th: -0.074,
            log_yardline_not_4th: -0.038,
            inside_2m_warning: -0.082,
            garbage_time_win: 0.109,
            garbage_time_loss: -0.171,
            clock_running: -0.017,
            inv_half_minutes: -0.170,
            log_inv_half_minutes: 0.108,
            inv_game_minutes: 0.073,
            log_inv_game_minutes: -0.104,
            possession_diff: 0.009,
            fg_possession_diff: 0.021,
            possdiff_per_minute: -0.172,
            fgpossdiff_per_minute: -0.083,
            clock_runs_pdpm: -0.003,
            clock_runs_fgpdpm: 0.067,
            clock_runs_pdpm2: 0.123,
            clock_runs_fgpdpm2: 0.083,
            off_timeouts_remaining_0: -0.076,
            off_timeouts_remaining_1: 0.041,
            off_timeouts_remaining_2: 0.073,
            off_timeouts_remaining_3: -0.039,
            clock_runs_pdpm_off0to: -0.166,
            clock_runs_pdpm_off1to: -0.179,
            clock_runs_pdpm_off2to: 0.242,
            clock_runs_pdpm_off3to: 0.101,
            def_timeouts_remaining_0: -0.222,
            def_timeouts_remaining_1: 0.062,
            def_timeouts_remaining_2: 0.068,
            def_timeouts_remaining_3: 0.092,
            clock_runs_pdpm_def0to: 0.160,
            clock_runs_pdpm_def1to: -0.188,
            clock_runs_pdpm_def2to: 0.187,
            clock_runs_pdpm_def3to: -0.162,
            offense_penalty_z: 0.013,
            defense_penalty_z: -0.002,
            off_def_penalty_z: 0.024,
        }
    }


    pub fn defensive_15_yards_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: -1.222,
            is_offense_home: -0.023,
            offense_log_pass_prob: -0.305,
            defense_log_pass_prob: -0.001,
            off_def_lpp: 0.159,
            off_lpp_rz: -0.396,
            def_lpp_rz: -0.126,
            off_def_lpp_rz: 0.285,
            off_lpp_outside_rz: -0.200,
            off_lpp_inside_rz: -0.194,
            def_lpp_outside_rz: 0.069,
            def_lpp_inside_rz: -0.053,
            off_lpp_pdpm: -0.047,
            def_lpp_pdpm: 0.107,
            off_lpp_rz_pdpm: 0.071,
            def_lpp_rz_pdpm: 0.082,
            down_1: -0.009,
            down_2: -0.065,
            down_3: -0.124,
            down_4: 0.196,
            goal_to_go: 0.135,
            z_ydstogo: 0.170,
            ydstogo_pct: 0.082,
            log_ydstogo_pct: 0.000,
            to_go_1st: 0.011,
            to_go_2nd: 0.116,
            to_go_3rd: -0.100,
            to_go_4th: -0.061,
            log_to_go_1st: -0.076,
            log_to_go_2nd: -0.243,
            log_to_go_3rd: -0.092,
            log_to_go_4th: 0.411,
            fp_1st: 0.013,
            fp_2nd: 0.000,
            fp_3rd: -0.112,
            fp_4th: 0.098,
            yardline_fgsig_4th: -0.092,
            yardline_puntsig_4th: -0.061,
            yardline_pct: 0.118,
            yardline_pct_sq: 0.227,
            log_yardline_pct: 0.066,
            fg_sigmoid: 0.024,
            punt_sigmoid: -0.009,
            goal_to_go_yardline: 0.033,
            log_goal_to_go_yardline: 0.018,
            yards_to_go_yardline: 0.085,
            log_yards_to_go_yardline: 0.048,
            yardline_4th: 0.086,
            log_yardline_4th: -0.120,
            yardline_not_4th: 0.031,
            log_yardline_not_4th: 0.186,
            inside_2m_warning: -0.437,
            garbage_time_win: 0.055,
            garbage_time_loss: 0.039,
            clock_running: -0.071,
            inv_half_minutes: 0.056,
            log_inv_half_minutes: 0.085,
            inv_game_minutes: -0.154,
            log_inv_game_minutes: -0.151,
            possession_diff: -0.023,
            fg_possession_diff: 0.022,
            possdiff_per_minute: -0.075,
            fgpossdiff_per_minute: 0.230,
            clock_runs_pdpm: 0.043,
            clock_runs_fgpdpm: -0.175,
            clock_runs_pdpm2: 0.063,
            clock_runs_fgpdpm2: -0.045,
            off_timeouts_remaining_0: 0.301,
            off_timeouts_remaining_1: -0.071,
            off_timeouts_remaining_2: -0.044,
            off_timeouts_remaining_3: -0.188,
            clock_runs_pdpm_off0to: -0.160,
            clock_runs_pdpm_off1to: 0.132,
            clock_runs_pdpm_off2to: 0.074,
            clock_runs_pdpm_off3to: -0.003,
            def_timeouts_remaining_0: 0.193,
            def_timeouts_remaining_1: -0.066,
            def_timeouts_remaining_2: -0.019,
            def_timeouts_remaining_3: -0.110,
            clock_runs_pdpm_def0to: -0.097,
            clock_runs_pdpm_def1to: -0.114,
            clock_runs_pdpm_def2to: 0.133,
            clock_runs_pdpm_def3to: 0.121,
            offense_penalty_z: 0.002,
            defense_penalty_z: -0.003,
            off_def_penalty_z: 0.021,
        }
    }


    pub fn defensive_yards_to_1_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: -4.543,
            is_offense_home: 0.194,
            offense_log_pass_prob: -0.007,
            defense_log_pass_prob: -0.056,
            off_def_lpp: 0.023,
            off_lpp_rz: 0.097,
            def_lpp_rz: -0.074,
            off_def_lpp_rz: -0.005,
            off_lpp_outside_rz: 0.239,
            off_lpp_inside_rz: -0.178,
            def_lpp_outside_rz: 0.081,
            def_lpp_inside_rz: -0.161,
            off_lpp_pdpm: 0.001,
            def_lpp_pdpm: -0.085,
            off_lpp_rz_pdpm: -0.030,
            def_lpp_rz_pdpm: -0.101,
            down_1: 0.273,
            down_2: -0.172,
            down_3: -0.094,
            down_4: -0.007,
            goal_to_go: 0.142,
            z_ydstogo: 0.127,
            ydstogo_pct: 0.063,
            log_ydstogo_pct: -0.090,
            to_go_1st: 0.107,
            to_go_2nd: -0.044,
            to_go_3rd: -0.082,
            to_go_4th: 0.036,
            log_to_go_1st: -0.026,
            log_to_go_2nd: -0.093,
            log_to_go_3rd: 0.155,
            log_to_go_4th: -0.127,
            fp_1st: 0.101,
            fp_2nd: -0.111,
            fp_3rd: -0.014,
            fp_4th: -0.016,
            yardline_fgsig_4th: 0.050,
            yardline_puntsig_4th: -0.104,
            yardline_pct: -0.143,
            yardline_pct_sq: -0.069,
            log_yardline_pct: -0.866,
            fg_sigmoid: 0.199,
            punt_sigmoid: -0.062,
            goal_to_go_yardline: 0.001,
            log_goal_to_go_yardline: -0.524,
            yards_to_go_yardline: -0.144,
            log_yards_to_go_yardline: -0.342,
            yardline_4th: -0.009,
            log_yardline_4th: -0.037,
            yardline_not_4th: -0.134,
            log_yardline_not_4th: -0.829,
            inside_2m_warning: -0.123,
            garbage_time_win: -0.034,
            garbage_time_loss: -0.145,
            clock_running: -0.094,
            inv_half_minutes: 0.247,
            log_inv_half_minutes: 0.138,
            inv_game_minutes: -0.004,
            log_inv_game_minutes: 0.020,
            possession_diff: 0.046,
            fg_possession_diff: -0.025,
            possdiff_per_minute: 0.184,
            fgpossdiff_per_minute: -0.084,
            clock_runs_pdpm: -0.035,
            clock_runs_fgpdpm: -0.091,
            clock_runs_pdpm2: 0.058,
            clock_runs_fgpdpm2: -0.084,
            off_timeouts_remaining_0: 0.001,
            off_timeouts_remaining_1: -0.205,
            off_timeouts_remaining_2: 0.105,
            off_timeouts_remaining_3: 0.098,
            clock_runs_pdpm_off0to: 0.008,
            clock_runs_pdpm_off1to: -0.072,
            clock_runs_pdpm_off2to: -0.060,
            clock_runs_pdpm_off3to: 0.089,
            def_timeouts_remaining_0: 0.247,
            def_timeouts_remaining_1: -0.232,
            def_timeouts_remaining_2: 0.032,
            def_timeouts_remaining_3: -0.048,
            clock_runs_pdpm_def0to: 0.011,
            clock_runs_pdpm_def1to: 0.021,
            clock_runs_pdpm_def2to: 0.009,
            clock_runs_pdpm_def3to: -0.077,
            offense_penalty_z: -0.004,
            defense_penalty_z: -0.067,
            off_def_penalty_z: 0.028,
        }
    }

    pub fn defensive_yards_vary_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: 2.997,
            is_offense_home: -0.002,
            offense_log_pass_prob: 0.001,
            defense_log_pass_prob: -0.000,
            off_def_lpp: -0.001,
            off_lpp_rz: 0.002,
            def_lpp_rz: 0.002,
            off_def_lpp_rz: -0.002,
            off_lpp_outside_rz: -0.013,
            off_lpp_inside_rz: 0.014,
            def_lpp_outside_rz: -0.014,
            def_lpp_inside_rz: 0.015,
            off_lpp_pdpm: 0.000,
            def_lpp_pdpm: -0.001,
            off_lpp_rz_pdpm: 0.001,
            def_lpp_rz_pdpm: -0.000,
            down_1: 0.045,
            down_2: -0.013,
            down_3: -0.030,
            down_4: -0.002,
            goal_to_go: -0.003,
            z_ydstogo: 0.040,
            ydstogo_pct: 0.020,
            log_ydstogo_pct: 0.020,
            to_go_1st: 0.017,
            to_go_2nd: -0.001,
            to_go_3rd: -0.022,
            to_go_4th: -0.002,
            log_to_go_1st: -0.002,
            log_to_go_2nd: -0.012,
            log_to_go_3rd: 0.030,
            log_to_go_4th: 0.004,
            fp_1st: 0.017,
            fp_2nd: -0.006,
            fp_3rd: -0.009,
            fp_4th: -0.000,
            yardline_fgsig_4th: -0.009,
            yardline_puntsig_4th: 0.010,
            yardline_pct: 0.005,
            yardline_pct_sq: 0.001,
            log_yardline_pct: 0.029,
            fg_sigmoid: 0.011,
            punt_sigmoid: 0.025,
            goal_to_go_yardline: -0.000,
            log_goal_to_go_yardline: 0.008,
            yards_to_go_yardline: 0.005,
            log_yards_to_go_yardline: 0.021,
            yardline_4th: -0.001,
            log_yardline_4th: 0.003,
            yardline_not_4th: 0.006,
            log_yardline_not_4th: 0.027,
            inside_2m_warning: -0.005,
            garbage_time_win: 0.002,
            garbage_time_loss: -0.002,
            clock_running: -0.011,
            inv_half_minutes: 0.011,
            log_inv_half_minutes: -0.009,
            inv_game_minutes: 0.004,
            log_inv_game_minutes: -0.008,
            possession_diff: 0.007,
            fg_possession_diff: 0.015,
            possdiff_per_minute: 0.001,
            fgpossdiff_per_minute: 0.004,
            clock_runs_pdpm: 0.003,
            clock_runs_fgpdpm: 0.002,
            clock_runs_pdpm2: 0.004,
            clock_runs_fgpdpm2: 0.004,
            off_timeouts_remaining_0: 0.003,
            off_timeouts_remaining_1: 0.002,
            off_timeouts_remaining_2: -0.002,
            off_timeouts_remaining_3: -0.002,
            clock_runs_pdpm_off0to: 0.003,
            clock_runs_pdpm_off1to: 0.001,
            clock_runs_pdpm_off2to: -0.003,
            clock_runs_pdpm_off3to: 0.003,
            def_timeouts_remaining_0: 0.002,
            def_timeouts_remaining_1: 0.009,
            def_timeouts_remaining_2: -0.003,
            def_timeouts_remaining_3: -0.008,
            clock_runs_pdpm_def0to: -0.001,
            clock_runs_pdpm_def1to: -0.000,
            clock_runs_pdpm_def2to: 0.001,
            clock_runs_pdpm_def3to: 0.004,
            offense_penalty_z: -0.002,
            defense_penalty_z: 0.001,
            off_def_penalty_z: -0.027,
        }
    }

    pub fn defensive_yards_vary_var_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: 5.061,
            is_offense_home: 0.042,
            offense_log_pass_prob: 0.003,
            defense_log_pass_prob: -0.008,
            off_def_lpp: 0.002,
            off_lpp_rz: 0.017,
            def_lpp_rz: 0.003,
            off_def_lpp_rz: -0.015,
            off_lpp_outside_rz: -0.025,
            off_lpp_inside_rz: 0.032,
            def_lpp_outside_rz: -0.036,
            def_lpp_inside_rz: 0.033,
            off_lpp_pdpm: -0.008,
            def_lpp_pdpm: -0.012,
            off_lpp_rz_pdpm: -0.004,
            def_lpp_rz_pdpm: -0.009,
            down_1: 0.159,
            down_2: -0.043,
            down_3: -0.114,
            down_4: -0.002,
            goal_to_go: 0.004,
            z_ydstogo: 0.075,
            ydstogo_pct: 0.038,
            log_ydstogo_pct: -0.077,
            to_go_1st: 0.072,
            to_go_2nd: 0.000,
            to_go_3rd: -0.070,
            to_go_4th: -0.001,
            log_to_go_1st: -0.040,
            log_to_go_2nd: -0.041,
            log_to_go_3rd: 0.010,
            log_to_go_4th: -0.006,
            fp_1st: 0.058,
            fp_2nd: -0.011,
            fp_3rd: -0.063,
            fp_4th: 0.000,
            yardline_fgsig_4th: -0.034,
            yardline_puntsig_4th: 0.006,
            yardline_pct: 0.023,
            yardline_pct_sq: 0.009,
            log_yardline_pct: 0.073,
            fg_sigmoid: 0.016,
            punt_sigmoid: 0.061,
            goal_to_go_yardline: -0.000,
            log_goal_to_go_yardline: -0.018,
            yards_to_go_yardline: 0.023,
            log_yards_to_go_yardline: 0.090,
            yardline_4th: -0.000,
            log_yardline_4th: 0.008,
            yardline_not_4th: 0.023,
            log_yardline_not_4th: 0.065,
            inside_2m_warning: -0.080,
            garbage_time_win: 0.025,
            garbage_time_loss: -0.023,
            clock_running: 0.007,
            inv_half_minutes: 0.041,
            log_inv_half_minutes: -0.009,
            inv_game_minutes: 0.010,
            log_inv_game_minutes: 0.017,
            possession_diff: -0.024,
            fg_possession_diff: 0.039,
            possdiff_per_minute: 0.025,
            fgpossdiff_per_minute: 0.037,
            clock_runs_pdpm: 0.003,
            clock_runs_fgpdpm: 0.011,
            clock_runs_pdpm2: 0.080,
            clock_runs_fgpdpm2: 0.028,
            off_timeouts_remaining_0: 0.064,
            off_timeouts_remaining_1: 0.041,
            off_timeouts_remaining_2: -0.053,
            off_timeouts_remaining_3: -0.052,
            clock_runs_pdpm_off0to: 0.018,
            clock_runs_pdpm_off1to: 0.033,
            clock_runs_pdpm_off2to: -0.042,
            clock_runs_pdpm_off3to: -0.006,
            def_timeouts_remaining_0: -0.029,
            def_timeouts_remaining_1: 0.018,
            def_timeouts_remaining_2: -0.022,
            def_timeouts_remaining_3: 0.032,
            clock_runs_pdpm_def0to: 0.000,
            clock_runs_pdpm_def1to: -0.003,
            clock_runs_pdpm_def2to: 0.018,
            clock_runs_pdpm_def3to: -0.013,
            offense_penalty_z: 0.003,
            defense_penalty_z: 0.007,
            off_def_penalty_z: -0.047,
        }
    }

}