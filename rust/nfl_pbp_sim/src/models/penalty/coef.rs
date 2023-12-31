use crate::models::penalty::PenaltyModel;

impl PenaltyModel {

    pub fn offensive_loss_of_down_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: -9.496,
            is_offense_home: 0.096,
            offense_log_pass_prob: -0.017,
            defense_log_pass_prob: 0.007,
            off_def_lpp: 0.004,
            off_lpp_rz: -0.022,
            def_lpp_rz: 0.006,
            off_def_lpp_rz: 0.008,
            off_lpp_outside_rz: -0.050,
            off_lpp_inside_rz: 0.035,
            def_lpp_outside_rz: -0.026,
            def_lpp_inside_rz: 0.035,
            off_lpp_pdpm: -0.006,
            def_lpp_pdpm: -0.007,
            off_lpp_rz_pdpm: -0.006,
            def_lpp_rz_pdpm: -0.006,
            down_1: -0.076,
            down_2: 0.034,
            down_3: 0.050,
            down_4: -0.008,
            goal_to_go: -0.019,
            z_ydstogo: -0.021,
            ydstogo_pct: -0.010,
            log_ydstogo_pct: 0.023,
            to_go_1st: -0.030,
            to_go_2nd: 0.016,
            to_go_3rd: 0.017,
            to_go_4th: -0.005,
            log_to_go_1st: 0.007,
            log_to_go_2nd: -0.005,
            log_to_go_3rd: 0.013,
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
            punt_sigmoid: -0.205,
            goal_to_go_yardline: -0.001,
            log_goal_to_go_yardline: 0.060,
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
            inv_game_minutes: 0.005,
            log_inv_game_minutes: 0.168,
            possession_diff: -0.103,
            fg_possession_diff: -0.012,
            possdiff_per_minute: 0.013,
            fgpossdiff_per_minute: 0.038,
            clock_runs_pdpm: -0.007,
            clock_runs_fgpdpm: 0.030,
            clock_runs_pdpm2: -0.001,
            clock_runs_fgpdpm2: -0.003,
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
            clock_runs_pdpm_def2to: -0.007,
            clock_runs_pdpm_def3to: 0.004,
            offense_penalty_z: 0.091,
            defense_penalty_z: -0.201,
            off_def_penalty_z: -0.050,
        }
    }


    pub fn defensive_automatic_first_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: 1.013,
            is_offense_home: 0.053,
            offense_log_pass_prob: 0.273,
            defense_log_pass_prob: -0.214,
            off_def_lpp: -0.050,
            off_lpp_rz: -0.098,
            def_lpp_rz: 0.117,
            off_def_lpp_rz: -0.038,
            off_lpp_outside_rz: 0.330,
            off_lpp_inside_rz: 0.040,
            def_lpp_outside_rz: -0.177,
            def_lpp_inside_rz: 0.101,
            off_lpp_pdpm: -0.263,
            def_lpp_pdpm: 0.113,
            off_lpp_rz_pdpm: 0.342,
            def_lpp_rz_pdpm: -0.024,
            down_1: 0.016,
            down_2: 0.120,
            down_3: 0.070,
            down_4: -0.207,
            goal_to_go: -0.019,
            z_ydstogo: -0.364,
            ydstogo_pct: -0.183,
            log_ydstogo_pct: 0.660,
            to_go_1st: -0.114,
            to_go_2nd: 0.072,
            to_go_3rd: 0.221,
            to_go_4th: -0.022,
            log_to_go_1st: 0.426,
            log_to_go_2nd: 0.205,
            log_to_go_3rd: 0.028,
            log_to_go_4th: 0.001,
            fp_1st: -0.017,
            fp_2nd: 0.012,
            fp_3rd: 0.108,
            fp_4th: -0.108,
            yardline_fgsig_4th: 0.043,
            yardline_puntsig_4th: -0.084,
            yardline_pct: 0.113,
            yardline_pct_sq: 0.072,
            log_yardline_pct: -0.024,
            fg_sigmoid: -0.011,
            punt_sigmoid: 0.008,
            goal_to_go_yardline: -0.021,
            log_goal_to_go_yardline: -0.062,
            yards_to_go_yardline: 0.134,
            log_yards_to_go_yardline: 0.038,
            yardline_4th: -0.035,
            log_yardline_4th: 0.349,
            yardline_not_4th: 0.148,
            log_yardline_not_4th: -0.373,
            inside_2m_warning: 0.114,
            garbage_time_win: -0.574,
            garbage_time_loss: -0.295,
            clock_running: -0.081,
            inv_half_minutes: -0.040,
            log_inv_half_minutes: -0.029,
            inv_game_minutes: -0.397,
            log_inv_game_minutes: 0.210,
            possession_diff: 0.026,
            fg_possession_diff: -0.071,
            possdiff_per_minute: -0.102,
            fgpossdiff_per_minute: 0.033,
            clock_runs_pdpm: -0.237,
            clock_runs_fgpdpm: -0.138,
            clock_runs_pdpm2: 0.172,
            clock_runs_fgpdpm2: -0.169,
            off_timeouts_remaining_0: -0.162,
            off_timeouts_remaining_1: 0.127,
            off_timeouts_remaining_2: 0.022,
            off_timeouts_remaining_3: 0.011,
            clock_runs_pdpm_off0to: 0.081,
            clock_runs_pdpm_off1to: -0.203,
            clock_runs_pdpm_off2to: 0.020,
            clock_runs_pdpm_off3to: -0.135,
            def_timeouts_remaining_0: -0.241,
            def_timeouts_remaining_1: 0.063,
            def_timeouts_remaining_2: 0.130,
            def_timeouts_remaining_3: 0.046,
            clock_runs_pdpm_def0to: -0.249,
            clock_runs_pdpm_def1to: -0.067,
            clock_runs_pdpm_def2to: 0.060,
            clock_runs_pdpm_def3to: 0.019,
            offense_penalty_z: -0.013,
            defense_penalty_z: 0.091,
            off_def_penalty_z: -0.022,
        }
    }


    pub fn offensive_5_yards_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: -0.511,
            is_offense_home: -0.100,
            offense_log_pass_prob: 0.055,
            defense_log_pass_prob: -0.003,
            off_def_lpp: -0.032,
            off_lpp_rz: -0.283,
            def_lpp_rz: -0.252,
            off_def_lpp_rz: 0.347,
            off_lpp_outside_rz: 0.247,
            off_lpp_inside_rz: 0.153,
            def_lpp_outside_rz: 0.047,
            def_lpp_inside_rz: 0.212,
            off_lpp_pdpm: 0.142,
            def_lpp_pdpm: -0.105,
            off_lpp_rz_pdpm: -0.070,
            def_lpp_rz_pdpm: -0.156,
            down_1: -0.567,
            down_2: -0.474,
            down_3: 0.257,
            down_4: 0.783,
            goal_to_go: -0.063,
            z_ydstogo: 0.232,
            ydstogo_pct: 0.115,
            log_ydstogo_pct: -0.292,
            to_go_1st: -0.177,
            to_go_2nd: -0.228,
            to_go_3rd: -0.068,
            to_go_4th: 0.391,
            log_to_go_1st: -0.227,
            log_to_go_2nd: 0.002,
            log_to_go_3rd: 0.175,
            log_to_go_4th: -0.242,
            fp_1st: -0.325,
            fp_2nd: -0.117,
            fp_3rd: 0.187,
            fp_4th: 0.250,
            yardline_fgsig_4th: -0.013,
            yardline_puntsig_4th: -0.032,
            yardline_pct: 0.275,
            yardline_pct_sq: 0.463,
            log_yardline_pct: -0.117,
            fg_sigmoid: 0.001,
            punt_sigmoid: -0.005,
            goal_to_go_yardline: -0.003,
            log_goal_to_go_yardline: -0.083,
            yards_to_go_yardline: 0.278,
            log_yards_to_go_yardline: -0.034,
            yardline_4th: 0.322,
            log_yardline_4th: -0.051,
            yardline_not_4th: -0.047,
            log_yardline_not_4th: -0.065,
            inside_2m_warning: -0.069,
            garbage_time_win: 0.130,
            garbage_time_loss: 0.317,
            clock_running: 0.056,
            inv_half_minutes: 0.034,
            log_inv_half_minutes: 0.037,
            inv_game_minutes: 0.645,
            log_inv_game_minutes: -0.159,
            possession_diff: 0.010,
            fg_possession_diff: -0.027,
            possdiff_per_minute: 0.197,
            fgpossdiff_per_minute: -0.054,
            clock_runs_pdpm: -0.022,
            clock_runs_fgpdpm: 0.046,
            clock_runs_pdpm2: 0.021,
            clock_runs_fgpdpm2: 0.000,
            off_timeouts_remaining_0: 0.335,
            off_timeouts_remaining_1: -0.068,
            off_timeouts_remaining_2: -0.175,
            off_timeouts_remaining_3: -0.093,
            clock_runs_pdpm_off0to: -0.107,
            clock_runs_pdpm_off1to: -0.119,
            clock_runs_pdpm_off2to: -0.025,
            clock_runs_pdpm_off3to: 0.230,
            def_timeouts_remaining_0: 0.052,
            def_timeouts_remaining_1: 0.044,
            def_timeouts_remaining_2: -0.089,
            def_timeouts_remaining_3: -0.009,
            clock_runs_pdpm_def0to: 0.190,
            clock_runs_pdpm_def1to: 0.138,
            clock_runs_pdpm_def2to: -0.191,
            clock_runs_pdpm_def3to: -0.159,
            offense_penalty_z: -0.009,
            defense_penalty_z: 0.020,
            off_def_penalty_z: -0.060,
        }
    }


    pub fn offensive_15_yards_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: -3.958,
            is_offense_home: -0.067,
            offense_log_pass_prob: -0.091,
            defense_log_pass_prob: 0.055,
            off_def_lpp: 0.014,
            off_lpp_rz: -0.155,
            def_lpp_rz: -0.062,
            off_def_lpp_rz: 0.096,
            off_lpp_outside_rz: -0.239,
            off_lpp_inside_rz: 0.101,
            def_lpp_outside_rz: -0.067,
            def_lpp_inside_rz: 0.049,
            off_lpp_pdpm: -0.067,
            def_lpp_pdpm: -0.042,
            off_lpp_rz_pdpm: -0.047,
            def_lpp_rz_pdpm: -0.044,
            down_1: 0.022,
            down_2: 0.074,
            down_3: -0.050,
            down_4: -0.045,
            goal_to_go: -0.147,
            z_ydstogo: 0.104,
            ydstogo_pct: 0.052,
            log_ydstogo_pct: -0.048,
            to_go_1st: -0.012,
            to_go_2nd: 0.079,
            to_go_3rd: -0.019,
            to_go_4th: -0.058,
            log_to_go_1st: 0.021,
            log_to_go_2nd: -0.109,
            log_to_go_3rd: -0.101,
            log_to_go_4th: 0.140,
            fp_1st: -0.004,
            fp_2nd: 0.030,
            fp_3rd: -0.027,
            fp_4th: 0.001,
            yardline_fgsig_4th: -0.086,
            yardline_puntsig_4th: -0.091,
            yardline_pct: 0.054,
            yardline_pct_sq: 0.037,
            log_yardline_pct: -0.084,
            fg_sigmoid: 0.025,
            punt_sigmoid: 0.012,
            goal_to_go_yardline: -0.025,
            log_goal_to_go_yardline: -0.054,
            yards_to_go_yardline: 0.080,
            log_yards_to_go_yardline: -0.030,
            yardline_4th: -0.004,
            log_yardline_4th: -0.044,
            yardline_not_4th: 0.059,
            log_yardline_not_4th: -0.040,
            inside_2m_warning: -0.007,
            garbage_time_win: -0.061,
            garbage_time_loss: -0.119,
            clock_running: 0.384,
            inv_half_minutes: -0.009,
            log_inv_half_minutes: 0.075,
            inv_game_minutes: 0.030,
            log_inv_game_minutes: -0.227,
            possession_diff: 0.106,
            fg_possession_diff: -0.086,
            possdiff_per_minute: 0.056,
            fgpossdiff_per_minute: -0.064,
            clock_runs_pdpm: -0.010,
            clock_runs_fgpdpm: -0.107,
            clock_runs_pdpm2: 0.076,
            clock_runs_fgpdpm2: -0.013,
            off_timeouts_remaining_0: 0.153,
            off_timeouts_remaining_1: -0.297,
            off_timeouts_remaining_2: -0.067,
            off_timeouts_remaining_3: 0.212,
            clock_runs_pdpm_off0to: 0.027,
            clock_runs_pdpm_off1to: 0.020,
            clock_runs_pdpm_off2to: -0.056,
            clock_runs_pdpm_off3to: -0.000,
            def_timeouts_remaining_0: 0.056,
            def_timeouts_remaining_1: 0.093,
            def_timeouts_remaining_2: 0.048,
            def_timeouts_remaining_3: -0.197,
            clock_runs_pdpm_def0to: -0.000,
            clock_runs_pdpm_def1to: 0.012,
            clock_runs_pdpm_def2to: -0.121,
            clock_runs_pdpm_def3to: 0.099,
            offense_penalty_z: 0.005,
            defense_penalty_z: -0.006,
            off_def_penalty_z: -0.046,
        }
    }


    pub fn defensive_5_yards_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: -0.432,
            is_offense_home: 0.060,
            offense_log_pass_prob: 0.130,
            defense_log_pass_prob: -0.239,
            off_def_lpp: 0.062,
            off_lpp_rz: 0.103,
            def_lpp_rz: -0.028,
            off_def_lpp_rz: -0.041,
            off_lpp_outside_rz: 0.095,
            off_lpp_inside_rz: -0.049,
            def_lpp_outside_rz: -0.208,
            def_lpp_inside_rz: 0.064,
            off_lpp_pdpm: 0.143,
            def_lpp_pdpm: 0.018,
            off_lpp_rz_pdpm: -0.046,
            def_lpp_rz_pdpm: -0.069,
            down_1: 0.029,
            down_2: -0.072,
            down_3: 0.003,
            down_4: 0.040,
            goal_to_go: 0.126,
            z_ydstogo: -0.217,
            ydstogo_pct: -0.109,
            log_ydstogo_pct: 0.278,
            to_go_1st: -0.082,
            to_go_2nd: -0.024,
            to_go_3rd: -0.020,
            to_go_4th: 0.075,
            log_to_go_1st: 0.293,
            log_to_go_2nd: 0.167,
            log_to_go_3rd: 0.082,
            log_to_go_4th: -0.264,
            fp_1st: 0.021,
            fp_2nd: 0.091,
            fp_3rd: 0.031,
            fp_4th: -0.037,
            yardline_fgsig_4th: 0.083,
            yardline_puntsig_4th: 0.025,
            yardline_pct: -0.046,
            yardline_pct_sq: -0.141,
            log_yardline_pct: 0.108,
            fg_sigmoid: -0.014,
            punt_sigmoid: -0.010,
            goal_to_go_yardline: 0.020,
            log_goal_to_go_yardline: 0.014,
            yards_to_go_yardline: -0.065,
            log_yards_to_go_yardline: 0.094,
            yardline_4th: 0.030,
            log_yardline_4th: 0.151,
            yardline_not_4th: -0.075,
            log_yardline_not_4th: -0.044,
            inside_2m_warning: -0.070,
            garbage_time_win: 0.113,
            garbage_time_loss: -0.176,
            clock_running: -0.011,
            inv_half_minutes: -0.173,
            log_inv_half_minutes: 0.099,
            inv_game_minutes: 0.082,
            log_inv_game_minutes: -0.111,
            possession_diff: 0.009,
            fg_possession_diff: 0.019,
            possdiff_per_minute: -0.183,
            fgpossdiff_per_minute: -0.075,
            clock_runs_pdpm: -0.007,
            clock_runs_fgpdpm: 0.075,
            clock_runs_pdpm2: 0.123,
            clock_runs_fgpdpm2: 0.080,
            off_timeouts_remaining_0: -0.085,
            off_timeouts_remaining_1: 0.049,
            off_timeouts_remaining_2: 0.077,
            off_timeouts_remaining_3: -0.041,
            clock_runs_pdpm_off0to: -0.169,
            clock_runs_pdpm_off1to: -0.178,
            clock_runs_pdpm_off2to: 0.240,
            clock_runs_pdpm_off3to: 0.101,
            def_timeouts_remaining_0: -0.214,
            def_timeouts_remaining_1: 0.068,
            def_timeouts_remaining_2: 0.068,
            def_timeouts_remaining_3: 0.078,
            clock_runs_pdpm_def0to: 0.160,
            clock_runs_pdpm_def1to: -0.179,
            clock_runs_pdpm_def2to: 0.172,
            clock_runs_pdpm_def3to: -0.160,
            offense_penalty_z: 0.005,
            defense_penalty_z: 0.004,
            off_def_penalty_z: 0.015,
        }
    }


    pub fn defensive_15_yards_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: -1.208,
            is_offense_home: -0.020,
            offense_log_pass_prob: -0.324,
            defense_log_pass_prob: 0.012,
            off_def_lpp: 0.163,
            off_lpp_rz: -0.414,
            def_lpp_rz: -0.097,
            off_def_lpp_rz: 0.284,
            off_lpp_outside_rz: -0.210,
            off_lpp_inside_rz: -0.193,
            def_lpp_outside_rz: 0.093,
            def_lpp_inside_rz: -0.053,
            off_lpp_pdpm: -0.038,
            def_lpp_pdpm: 0.104,
            off_lpp_rz_pdpm: 0.089,
            def_lpp_rz_pdpm: 0.072,
            down_1: -0.018,
            down_2: -0.058,
            down_3: -0.130,
            down_4: 0.202,
            goal_to_go: 0.128,
            z_ydstogo: 0.185,
            ydstogo_pct: 0.089,
            log_ydstogo_pct: -0.023,
            to_go_1st: 0.008,
            to_go_2nd: 0.116,
            to_go_3rd: -0.103,
            to_go_4th: -0.061,
            log_to_go_1st: -0.077,
            log_to_go_2nd: -0.251,
            log_to_go_3rd: -0.111,
            log_to_go_4th: 0.416,
            fp_1st: 0.012,
            fp_2nd: -0.001,
            fp_3rd: -0.114,
            fp_4th: 0.102,
            yardline_fgsig_4th: -0.095,
            yardline_puntsig_4th: -0.062,
            yardline_pct: 0.105,
            yardline_pct_sq: 0.208,
            log_yardline_pct: 0.089,
            fg_sigmoid: 0.022,
            punt_sigmoid: -0.011,
            goal_to_go_yardline: 0.032,
            log_goal_to_go_yardline: 0.021,
            yards_to_go_yardline: 0.074,
            log_yards_to_go_yardline: 0.068,
            yardline_4th: 0.099,
            log_yardline_4th: -0.084,
            yardline_not_4th: 0.006,
            log_yardline_not_4th: 0.173,
            inside_2m_warning: -0.428,
            garbage_time_win: 0.054,
            garbage_time_loss: 0.033,
            clock_running: -0.080,
            inv_half_minutes: 0.052,
            log_inv_half_minutes: 0.082,
            inv_game_minutes: -0.146,
            log_inv_game_minutes: -0.159,
            possession_diff: -0.020,
            fg_possession_diff: 0.024,
            possdiff_per_minute: -0.070,
            fgpossdiff_per_minute: 0.229,
            clock_runs_pdpm: 0.046,
            clock_runs_fgpdpm: -0.173,
            clock_runs_pdpm2: 0.060,
            clock_runs_fgpdpm2: -0.043,
            off_timeouts_remaining_0: 0.324,
            off_timeouts_remaining_1: -0.077,
            off_timeouts_remaining_2: -0.053,
            off_timeouts_remaining_3: -0.198,
            clock_runs_pdpm_off0to: -0.155,
            clock_runs_pdpm_off1to: 0.133,
            clock_runs_pdpm_off2to: 0.066,
            clock_runs_pdpm_off3to: 0.001,
            def_timeouts_remaining_0: 0.193,
            def_timeouts_remaining_1: -0.072,
            def_timeouts_remaining_2: -0.009,
            def_timeouts_remaining_3: -0.115,
            clock_runs_pdpm_def0to: -0.100,
            clock_runs_pdpm_def1to: -0.088,
            clock_runs_pdpm_def2to: 0.124,
            clock_runs_pdpm_def3to: 0.110,
            offense_penalty_z: 0.008,
            defense_penalty_z: -0.009,
            off_def_penalty_z: 0.029,
        }
    }


    pub fn defensive_yards_to_1_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: -4.631,
            is_offense_home: 0.148,
            offense_log_pass_prob: 0.002,
            defense_log_pass_prob: -0.058,
            off_def_lpp: 0.020,
            off_lpp_rz: 0.108,
            def_lpp_rz: -0.087,
            off_def_lpp_rz: -0.005,
            off_lpp_outside_rz: 0.238,
            off_lpp_inside_rz: -0.168,
            def_lpp_outside_rz: 0.069,
            def_lpp_inside_rz: -0.165,
            off_lpp_pdpm: -0.005,
            def_lpp_pdpm: -0.087,
            off_lpp_rz_pdpm: -0.036,
            def_lpp_rz_pdpm: -0.094,
            down_1: 0.287,
            down_2: -0.176,
            down_3: -0.104,
            down_4: -0.007,
            goal_to_go: 0.144,
            z_ydstogo: 0.132,
            ydstogo_pct: 0.066,
            log_ydstogo_pct: -0.079,
            to_go_1st: 0.115,
            to_go_2nd: -0.044,
            to_go_3rd: -0.088,
            to_go_4th: 0.031,
            log_to_go_1st: -0.031,
            log_to_go_2nd: -0.100,
            log_to_go_3rd: 0.162,
            log_to_go_4th: -0.110,
            fp_1st: 0.106,
            fp_2nd: -0.114,
            fp_3rd: -0.016,
            fp_4th: -0.013,
            yardline_fgsig_4th: 0.054,
            yardline_puntsig_4th: -0.103,
            yardline_pct: -0.146,
            yardline_pct_sq: -0.070,
            log_yardline_pct: -0.896,
            fg_sigmoid: 0.196,
            punt_sigmoid: -0.062,
            goal_to_go_yardline: 0.002,
            log_goal_to_go_yardline: -0.527,
            yards_to_go_yardline: -0.148,
            log_yards_to_go_yardline: -0.370,
            yardline_4th: -0.014,
            log_yardline_4th: -0.076,
            yardline_not_4th: -0.132,
            log_yardline_not_4th: -0.821,
            inside_2m_warning: -0.130,
            garbage_time_win: -0.034,
            garbage_time_loss: -0.115,
            clock_running: -0.094,
            inv_half_minutes: 0.250,
            log_inv_half_minutes: 0.129,
            inv_game_minutes: 0.003,
            log_inv_game_minutes: 0.015,
            possession_diff: 0.048,
            fg_possession_diff: -0.029,
            possdiff_per_minute: 0.185,
            fgpossdiff_per_minute: -0.081,
            clock_runs_pdpm: -0.045,
            clock_runs_fgpdpm: -0.106,
            clock_runs_pdpm2: 0.064,
            clock_runs_fgpdpm2: -0.088,
            off_timeouts_remaining_0: 0.005,
            off_timeouts_remaining_1: -0.203,
            off_timeouts_remaining_2: 0.104,
            off_timeouts_remaining_3: 0.094,
            clock_runs_pdpm_off0to: 0.008,
            clock_runs_pdpm_off1to: -0.073,
            clock_runs_pdpm_off2to: -0.053,
            clock_runs_pdpm_off3to: 0.073,
            def_timeouts_remaining_0: 0.247,
            def_timeouts_remaining_1: -0.216,
            def_timeouts_remaining_2: 0.018,
            def_timeouts_remaining_3: -0.049,
            clock_runs_pdpm_def0to: 0.016,
            clock_runs_pdpm_def1to: 0.004,
            clock_runs_pdpm_def2to: 0.012,
            clock_runs_pdpm_def3to: -0.076,
            offense_penalty_z: 0.005,
            defense_penalty_z: -0.074,
            off_def_penalty_z: 0.026,
        }
    }

    pub fn defensive_yards_vary_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: 2.991,
            is_offense_home: -0.002,
            offense_log_pass_prob: 0.002,
            defense_log_pass_prob: 0.000,
            off_def_lpp: -0.001,
            off_lpp_rz: 0.002,
            def_lpp_rz: 0.002,
            off_def_lpp_rz: -0.002,
            off_lpp_outside_rz: -0.012,
            off_lpp_inside_rz: 0.015,
            def_lpp_outside_rz: -0.014,
            def_lpp_inside_rz: 0.015,
            off_lpp_pdpm: 0.000,
            def_lpp_pdpm: -0.001,
            off_lpp_rz_pdpm: 0.001,
            def_lpp_rz_pdpm: -0.000,
            down_1: 0.044,
            down_2: -0.012,
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
            fp_1st: 0.016,
            fp_2nd: -0.006,
            fp_3rd: -0.009,
            fp_4th: -0.000,
            yardline_fgsig_4th: -0.008,
            yardline_puntsig_4th: 0.012,
            yardline_pct: 0.005,
            yardline_pct_sq: 0.001,
            log_yardline_pct: 0.029,
            fg_sigmoid: 0.010,
            punt_sigmoid: 0.025,
            goal_to_go_yardline: -0.000,
            log_goal_to_go_yardline: 0.008,
            yards_to_go_yardline: 0.006,
            log_yards_to_go_yardline: 0.021,
            yardline_4th: -0.001,
            log_yardline_4th: 0.003,
            yardline_not_4th: 0.006,
            log_yardline_not_4th: 0.027,
            inside_2m_warning: -0.005,
            garbage_time_win: 0.002,
            garbage_time_loss: -0.002,
            clock_running: -0.010,
            inv_half_minutes: 0.010,
            log_inv_half_minutes: -0.010,
            inv_game_minutes: 0.004,
            log_inv_game_minutes: -0.008,
            possession_diff: 0.007,
            fg_possession_diff: 0.014,
            possdiff_per_minute: 0.000,
            fgpossdiff_per_minute: 0.004,
            clock_runs_pdpm: 0.003,
            clock_runs_fgpdpm: 0.002,
            clock_runs_pdpm2: 0.005,
            clock_runs_fgpdpm2: 0.004,
            off_timeouts_remaining_0: 0.002,
            off_timeouts_remaining_1: 0.002,
            off_timeouts_remaining_2: -0.002,
            off_timeouts_remaining_3: -0.002,
            clock_runs_pdpm_off0to: 0.003,
            clock_runs_pdpm_off1to: 0.001,
            clock_runs_pdpm_off2to: -0.003,
            clock_runs_pdpm_off3to: 0.002,
            def_timeouts_remaining_0: 0.002,
            def_timeouts_remaining_1: 0.008,
            def_timeouts_remaining_2: -0.003,
            def_timeouts_remaining_3: -0.007,
            clock_runs_pdpm_def0to: -0.001,
            clock_runs_pdpm_def1to: -0.000,
            clock_runs_pdpm_def2to: 0.001,
            clock_runs_pdpm_def3to: 0.004,
            offense_penalty_z: -0.001,
            defense_penalty_z: 0.000,
            off_def_penalty_z: -0.026,
        }
    }

    pub fn defensive_yards_vary_var_coef() -> PenaltyModel {
        PenaltyModel{
            intercept: 5.025,
            is_offense_home: 0.038,
            offense_log_pass_prob: 0.001,
            defense_log_pass_prob: -0.006,
            off_def_lpp: 0.002,
            off_lpp_rz: 0.017,
            def_lpp_rz: 0.002,
            off_def_lpp_rz: -0.014,
            off_lpp_outside_rz: -0.027,
            off_lpp_inside_rz: 0.032,
            def_lpp_outside_rz: -0.034,
            def_lpp_inside_rz: 0.033,
            off_lpp_pdpm: -0.009,
            def_lpp_pdpm: -0.013,
            off_lpp_rz_pdpm: -0.005,
            def_lpp_rz_pdpm: -0.010,
            down_1: 0.151,
            down_2: -0.047,
            down_3: -0.104,
            down_4: -0.001,
            goal_to_go: 0.004,
            z_ydstogo: 0.082,
            ydstogo_pct: 0.041,
            log_ydstogo_pct: -0.077,
            to_go_1st: 0.069,
            to_go_2nd: -0.000,
            to_go_3rd: -0.069,
            to_go_4th: 0.000,
            log_to_go_1st: -0.040,
            log_to_go_2nd: -0.044,
            log_to_go_3rd: 0.015,
            log_to_go_4th: -0.008,
            fp_1st: 0.055,
            fp_2nd: -0.013,
            fp_3rd: -0.059,
            fp_4th: 0.000,
            yardline_fgsig_4th: -0.033,
            yardline_puntsig_4th: 0.006,
            yardline_pct: 0.022,
            yardline_pct_sq: 0.008,
            log_yardline_pct: 0.072,
            fg_sigmoid: 0.015,
            punt_sigmoid: 0.061,
            goal_to_go_yardline: -0.001,
            log_goal_to_go_yardline: -0.018,
            yards_to_go_yardline: 0.023,
            log_yards_to_go_yardline: 0.090,
            yardline_4th: 0.001,
            log_yardline_4th: 0.007,
            yardline_not_4th: 0.022,
            log_yardline_not_4th: 0.065,
            inside_2m_warning: -0.069,
            garbage_time_win: 0.025,
            garbage_time_loss: -0.022,
            clock_running: 0.018,
            inv_half_minutes: 0.042,
            log_inv_half_minutes: -0.020,
            inv_game_minutes: 0.014,
            log_inv_game_minutes: 0.018,
            possession_diff: -0.026,
            fg_possession_diff: 0.035,
            possdiff_per_minute: 0.026,
            fgpossdiff_per_minute: 0.039,
            clock_runs_pdpm: 0.001,
            clock_runs_fgpdpm: 0.010,
            clock_runs_pdpm2: 0.080,
            clock_runs_fgpdpm2: 0.029,
            off_timeouts_remaining_0: 0.067,
            off_timeouts_remaining_1: 0.046,
            off_timeouts_remaining_2: -0.053,
            off_timeouts_remaining_3: -0.060,
            clock_runs_pdpm_off0to: 0.017,
            clock_runs_pdpm_off1to: 0.032,
            clock_runs_pdpm_off2to: -0.041,
            clock_runs_pdpm_off3to: -0.008,
            def_timeouts_remaining_0: -0.027,
            def_timeouts_remaining_1: 0.021,
            def_timeouts_remaining_2: -0.025,
            def_timeouts_remaining_3: 0.032,
            clock_runs_pdpm_def0to: 0.001,
            clock_runs_pdpm_def1to: -0.002,
            clock_runs_pdpm_def2to: 0.017,
            clock_runs_pdpm_def3to: -0.014,
            offense_penalty_z: 0.008,
            defense_penalty_z: 0.010,
            off_def_penalty_z: -0.048,
        }
    }

}