use crate::models::post_pass_penalty::PostPassPenaltyModel;

impl PostPassPenaltyModel {

    pub fn is_postpass_off_penalty_coef() -> PostPassPenaltyModel {
        PostPassPenaltyModel{
            intercept: -4.479,
            is_offense_home: -0.034,
            offense_log_pass_prob: 0.059,
            defense_log_pass_prob: -0.024,
            off_def_lpp: -0.015,
            off_lpp_rz: 0.337,
            def_lpp_rz: 0.374,
            off_def_lpp_rz: -0.352,
            off_lpp_outside_rz: -0.196,
            off_lpp_inside_rz: 0.283,
            def_lpp_outside_rz: -0.198,
            def_lpp_inside_rz: 0.248,
            off_lpp_pdpm: 0.029,
            def_lpp_pdpm: -0.005,
            off_lpp_rz_pdpm: -0.108,
            def_lpp_rz_pdpm: 0.018,
            down_1: 0.010,
            down_2: 0.040,
            down_3: 0.152,
            down_4: 0.006,
            goal_to_go: -0.181,
            z_ydstogo: 0.145,
            ydstogo_pct: 0.070,
            log_ydstogo_pct: 0.121,
            to_go_1st: -0.002,
            to_go_2nd: -0.001,
            to_go_3rd: 0.116,
            to_go_4th: -0.007,
            log_to_go_1st: 0.030,
            log_to_go_2nd: -0.145,
            log_to_go_3rd: -0.239,
            log_to_go_4th: -0.011,
            fp_1st: 0.020,
            fp_2nd: -0.101,
            fp_3rd: 0.108,
            fp_4th: 0.037,
            yardline_fgsig_4th: -0.049,
            yardline_puntsig_4th: 0.022,
            yardline_pct: 0.033,
            yardline_pct_sq: 0.067,
            log_yardline_pct: -0.131,
            fg_sigmoid: 0.015,
            punt_sigmoid: -0.005,
            goal_to_go_yardline: -0.040,
            log_goal_to_go_yardline: -0.107,
            yards_to_go_yardline: 0.073,
            log_yards_to_go_yardline: -0.024,
            yardline_4th: 0.030,
            log_yardline_4th: -0.217,
            yardline_not_4th: 0.003,
            log_yardline_not_4th: 0.086,
            inside_2m_warning: -0.060,
            garbage_time_win: -0.142,
            garbage_time_loss: -0.179,
            clock_running: -0.051,
            inv_half_minutes: -0.115,
            log_inv_half_minutes: 0.015,
            inv_game_minutes: -0.096,
            log_inv_game_minutes: 0.066,
            possession_diff: -0.079,
            fg_possession_diff: 0.047,
            possdiff_per_minute: -0.056,
            fgpossdiff_per_minute: -0.022,
            clock_runs_pdpm: 0.191,
            clock_runs_fgpdpm: -0.042,
            clock_runs_pdpm2: 0.174,
            clock_runs_fgpdpm2: -0.165,
            off_timeouts_remaining_0: -0.124,
            off_timeouts_remaining_1: -0.040,
            off_timeouts_remaining_2: 0.209,
            off_timeouts_remaining_3: -0.047,
            clock_runs_pdpm_off0to: 0.057,
            clock_runs_pdpm_off1to: -0.047,
            clock_runs_pdpm_off2to: -0.003,
            clock_runs_pdpm_off3to: 0.184,
            def_timeouts_remaining_0: -0.032,
            def_timeouts_remaining_1: 0.031,
            def_timeouts_remaining_2: 0.043,
            def_timeouts_remaining_3: -0.044,
            clock_runs_pdpm_def0to: 0.073,
            clock_runs_pdpm_def1to: -0.046,
            clock_runs_pdpm_def2to: 0.087,
            clock_runs_pdpm_def3to: 0.077,
            offense_penalty_z: -0.147,
            defense_penalty_z: -0.041,
            off_def_penalty_z: -0.052,
            qb_scramble: 0.111,
            sack: -1.258,
            throwaway: 2.230,
            target_incomplete: -1.139,
            target_complete: -0.149,
            interception: 0.412,
            yards_gained_div10: -0.390,
            yards_gained_div10_sq: 0.025,
            loss_of_down: 0.0,
        }
    }


    pub fn is_5_postpass_off_penalty_yards_coef() -> PostPassPenaltyModel {
        PostPassPenaltyModel{
            intercept: -4.805,
            is_offense_home: 0.058,
            offense_log_pass_prob: -0.058,
            defense_log_pass_prob: -0.030,
            off_def_lpp: 0.046,
            off_lpp_rz: -0.036,
            def_lpp_rz: -0.036,
            off_def_lpp_rz: 0.041,
            off_lpp_outside_rz: -0.006,
            off_lpp_inside_rz: -0.050,
            def_lpp_outside_rz: 0.020,
            def_lpp_inside_rz: -0.059,
            off_lpp_pdpm: 0.062,
            def_lpp_pdpm: 0.041,
            off_lpp_rz_pdpm: 0.045,
            def_lpp_rz_pdpm: 0.031,
            down_1: -0.235,
            down_2: -0.192,
            down_3: 0.433,
            down_4: -0.006,
            goal_to_go: 0.098,
            z_ydstogo: 0.411,
            ydstogo_pct: 0.205,
            log_ydstogo_pct: 0.119,
            to_go_1st: -0.078,
            to_go_2nd: -0.097,
            to_go_3rd: 0.147,
            to_go_4th: -0.002,
            log_to_go_1st: -0.021,
            log_to_go_2nd: 0.091,
            log_to_go_3rd: 0.047,
            log_to_go_4th: 0.002,
            fp_1st: -0.081,
            fp_2nd: -0.084,
            fp_3rd: 0.134,
            fp_4th: -0.000,
            yardline_fgsig_4th: -0.037,
            yardline_puntsig_4th: 0.093,
            yardline_pct: -0.029,
            yardline_pct_sq: -0.038,
            log_yardline_pct: -0.018,
            fg_sigmoid: -0.074,
            punt_sigmoid: -0.082,
            goal_to_go_yardline: 0.013,
            log_goal_to_go_yardline: -0.155,
            yards_to_go_yardline: -0.042,
            log_yards_to_go_yardline: 0.137,
            yardline_4th: 0.001,
            log_yardline_4th: 0.017,
            yardline_not_4th: -0.030,
            log_yardline_not_4th: -0.035,
            inside_2m_warning: -0.055,
            garbage_time_win: 0.000,
            garbage_time_loss: -0.046,
            clock_running: 0.097,
            inv_half_minutes: 0.085,
            log_inv_half_minutes: 0.097,
            inv_game_minutes: 0.212,
            log_inv_game_minutes: -0.029,
            possession_diff: 0.126,
            fg_possession_diff: -0.139,
            possdiff_per_minute: -0.108,
            fgpossdiff_per_minute: 0.129,
            clock_runs_pdpm: -0.002,
            clock_runs_fgpdpm: -0.017,
            clock_runs_pdpm2: 0.017,
            clock_runs_fgpdpm2: 0.006,
            off_timeouts_remaining_0: 0.096,
            off_timeouts_remaining_1: -0.084,
            off_timeouts_remaining_2: 0.035,
            off_timeouts_remaining_3: -0.047,
            clock_runs_pdpm_off0to: 0.014,
            clock_runs_pdpm_off1to: 0.004,
            clock_runs_pdpm_off2to: -0.016,
            clock_runs_pdpm_off3to: -0.004,
            def_timeouts_remaining_0: -0.031,
            def_timeouts_remaining_1: -0.052,
            def_timeouts_remaining_2: -0.077,
            def_timeouts_remaining_3: 0.160,
            clock_runs_pdpm_def0to: -0.001,
            clock_runs_pdpm_def1to: 0.005,
            clock_runs_pdpm_def2to: 0.002,
            clock_runs_pdpm_def3to: -0.008,
            offense_penalty_z: -0.151,
            defense_penalty_z: 0.019,
            off_def_penalty_z: 0.067,
            qb_scramble: 0.971,
            sack: 0.064,
            throwaway: -0.774,
            target_incomplete: -0.110,
            target_complete: -0.029,
            interception: -0.121,
            yards_gained_div10: -0.049,
            yards_gained_div10_sq: 0.072,
            loss_of_down: 0.924,
        }
    }


    pub fn is_10_postpass_off_penalty_yards_coef() -> PostPassPenaltyModel {
        PostPassPenaltyModel{
            intercept: -1.634,
            is_offense_home: -0.145,
            offense_log_pass_prob: 0.007,
            defense_log_pass_prob: 0.006,
            off_def_lpp: -0.006,
            off_lpp_rz: 0.065,
            def_lpp_rz: 0.106,
            off_def_lpp_rz: -0.093,
            off_lpp_outside_rz: -0.044,
            off_lpp_inside_rz: 0.075,
            def_lpp_outside_rz: -0.057,
            def_lpp_inside_rz: 0.078,
            off_lpp_pdpm: -0.055,
            def_lpp_pdpm: -0.066,
            off_lpp_rz_pdpm: -0.082,
            def_lpp_rz_pdpm: -0.077,
            down_1: 0.199,
            down_2: 0.154,
            down_3: -0.297,
            down_4: -0.056,
            goal_to_go: -0.020,
            z_ydstogo: 0.039,
            ydstogo_pct: 0.019,
            log_ydstogo_pct: 0.173,
            to_go_1st: 0.064,
            to_go_2nd: 0.140,
            to_go_3rd: -0.223,
            to_go_4th: -0.043,
            log_to_go_1st: 0.017,
            log_to_go_2nd: -0.286,
            log_to_go_3rd: 0.347,
            log_to_go_4th: 0.095,
            fp_1st: 0.077,
            fp_2nd: 0.053,
            fp_3rd: -0.081,
            fp_4th: -0.009,
            yardline_fgsig_4th: 0.135,
            yardline_puntsig_4th: 0.140,
            yardline_pct: 0.006,
            yardline_pct_sq: 0.023,
            log_yardline_pct: 0.028,
            fg_sigmoid: 0.031,
            punt_sigmoid: 0.021,
            goal_to_go_yardline: 0.001,
            log_goal_to_go_yardline: 0.090,
            yards_to_go_yardline: 0.006,
            log_yards_to_go_yardline: -0.062,
            yardline_4th: -0.026,
            log_yardline_4th: 0.046,
            yardline_not_4th: 0.032,
            log_yardline_not_4th: -0.017,
            inside_2m_warning: 0.082,
            garbage_time_win: 0.000,
            garbage_time_loss: -0.180,
            clock_running: -0.305,
            inv_half_minutes: -0.040,
            log_inv_half_minutes: -0.108,
            inv_game_minutes: -0.051,
            log_inv_game_minutes: -0.137,
            possession_diff: -0.053,
            fg_possession_diff: 0.122,
            possdiff_per_minute: 0.136,
            fgpossdiff_per_minute: 0.069,
            clock_runs_pdpm: 0.041,
            clock_runs_fgpdpm: 0.037,
            clock_runs_pdpm2: 0.087,
            clock_runs_fgpdpm2: 0.069,
            off_timeouts_remaining_0: -0.169,
            off_timeouts_remaining_1: 0.122,
            off_timeouts_remaining_2: -0.015,
            off_timeouts_remaining_3: 0.062,
            clock_runs_pdpm_off0to: -0.013,
            clock_runs_pdpm_off1to: 0.047,
            clock_runs_pdpm_off2to: -0.004,
            clock_runs_pdpm_off3to: 0.012,
            def_timeouts_remaining_0: -0.091,
            def_timeouts_remaining_1: 0.085,
            def_timeouts_remaining_2: -0.012,
            def_timeouts_remaining_3: 0.017,
            clock_runs_pdpm_def0to: -0.011,
            clock_runs_pdpm_def1to: 0.012,
            clock_runs_pdpm_def2to: 0.046,
            clock_runs_pdpm_def3to: -0.006,
            offense_penalty_z: -0.126,
            defense_penalty_z: -0.021,
            off_def_penalty_z: 0.030,
            qb_scramble: 0.159,
            sack: -0.015,
            throwaway: -0.858,
            target_incomplete: -0.603,
            target_complete: 1.709,
            interception: -0.393,
            yards_gained_div10: 0.262,
            yards_gained_div10_sq: -0.257,
            loss_of_down: -1.151,
        }
    }


    pub fn is_postpass_def_penalty_coef() -> PostPassPenaltyModel {
        PostPassPenaltyModel{
            intercept: -4.723,
            is_offense_home: 0.052,
            offense_log_pass_prob: -0.216,
            defense_log_pass_prob: -0.240,
            off_def_lpp: 0.236,
            off_lpp_rz: 0.037,
            def_lpp_rz: 0.073,
            off_def_lpp_rz: -0.097,
            off_lpp_outside_rz: -0.083,
            off_lpp_inside_rz: -0.067,
            def_lpp_outside_rz: -0.031,
            def_lpp_inside_rz: -0.121,
            off_lpp_pdpm: 0.078,
            def_lpp_pdpm: 0.170,
            off_lpp_rz_pdpm: -0.093,
            def_lpp_rz_pdpm: 0.058,
            down_1: -0.032,
            down_2: 0.071,
            down_3: 0.157,
            down_4: 0.028,
            goal_to_go: 0.185,
            z_ydstogo: -0.063,
            ydstogo_pct: -0.035,
            log_ydstogo_pct: 0.266,
            to_go_1st: 0.040,
            to_go_2nd: 0.011,
            to_go_3rd: 0.050,
            to_go_4th: 0.021,
            log_to_go_1st: -0.128,
            log_to_go_2nd: -0.069,
            log_to_go_3rd: -0.138,
            log_to_go_4th: 0.079,
            fp_1st: 0.031,
            fp_2nd: -0.009,
            fp_3rd: 0.017,
            fp_4th: 0.040,
            yardline_fgsig_4th: -0.012,
            yardline_puntsig_4th: -0.065,
            yardline_pct: -0.072,
            yardline_pct_sq: -0.031,
            log_yardline_pct: 0.133,
            fg_sigmoid: 0.009,
            punt_sigmoid: 0.007,
            goal_to_go_yardline: 0.064,
            log_goal_to_go_yardline: 0.078,
            yards_to_go_yardline: -0.136,
            log_yards_to_go_yardline: 0.055,
            yardline_4th: 0.030,
            log_yardline_4th: 0.182,
            yardline_not_4th: -0.102,
            log_yardline_not_4th: -0.048,
            inside_2m_warning: -0.048,
            garbage_time_win: 0.029,
            garbage_time_loss: 0.013,
            clock_running: -0.041,
            inv_half_minutes: -0.038,
            log_inv_half_minutes: 0.101,
            inv_game_minutes: -0.010,
            log_inv_game_minutes: -0.019,
            possession_diff: -0.003,
            fg_possession_diff: 0.005,
            possdiff_per_minute: 0.032,
            fgpossdiff_per_minute: 0.113,
            clock_runs_pdpm: 0.010,
            clock_runs_fgpdpm: -0.086,
            clock_runs_pdpm2: 0.212,
            clock_runs_fgpdpm2: -0.188,
            off_timeouts_remaining_0: -0.025,
            off_timeouts_remaining_1: -0.000,
            off_timeouts_remaining_2: 0.051,
            off_timeouts_remaining_3: -0.030,
            clock_runs_pdpm_off0to: 0.089,
            clock_runs_pdpm_off1to: -0.072,
            clock_runs_pdpm_off2to: 0.052,
            clock_runs_pdpm_off3to: -0.059,
            def_timeouts_remaining_0: -0.010,
            def_timeouts_remaining_1: -0.128,
            def_timeouts_remaining_2: 0.044,
            def_timeouts_remaining_3: 0.091,
            clock_runs_pdpm_def0to: 0.028,
            clock_runs_pdpm_def1to: -0.090,
            clock_runs_pdpm_def2to: -0.014,
            clock_runs_pdpm_def3to: 0.085,
            offense_penalty_z: 0.123,
            defense_penalty_z: -0.109,
            off_def_penalty_z: 0.036,
            qb_scramble: 1.774,
            sack: -0.190,
            throwaway: -0.886,
            target_incomplete: -1.325,
            target_complete: 0.210,
            interception: 0.629,
            yards_gained_div10: 0.310,
            yards_gained_div10_sq: -0.039,
            loss_of_down: 0.0,
        }
    }


    pub fn is_5_postpass_def_penalty_yards_coef() -> PostPassPenaltyModel {
        PostPassPenaltyModel{
            intercept: -2.329,
            is_offense_home: 0.224,
            offense_log_pass_prob: -0.152,
            defense_log_pass_prob: -0.024,
            off_def_lpp: 0.094,
            off_lpp_rz: -0.123,
            def_lpp_rz: -0.000,
            off_def_lpp_rz: 0.068,
            off_lpp_outside_rz: -0.165,
            off_lpp_inside_rz: 0.023,
            def_lpp_outside_rz: -0.053,
            def_lpp_inside_rz: 0.029,
            off_lpp_pdpm: -0.044,
            def_lpp_pdpm: -0.015,
            off_lpp_rz_pdpm: -0.097,
            def_lpp_rz_pdpm: -0.016,
            down_1: -0.032,
            down_2: -0.126,
            down_3: 0.115,
            down_4: 0.043,
            goal_to_go: -0.009,
            z_ydstogo: -0.017,
            ydstogo_pct: -0.008,
            log_ydstogo_pct: -0.032,
            to_go_1st: -0.016,
            to_go_2nd: -0.030,
            to_go_3rd: -0.003,
            to_go_4th: 0.057,
            log_to_go_1st: 0.013,
            log_to_go_2nd: -0.094,
            log_to_go_3rd: 0.178,
            log_to_go_4th: -0.129,
            fp_1st: -0.027,
            fp_2nd: -0.040,
            fp_3rd: 0.061,
            fp_4th: 0.001,
            yardline_fgsig_4th: 0.146,
            yardline_puntsig_4th: 0.002,
            yardline_pct: -0.064,
            yardline_pct_sq: -0.098,
            log_yardline_pct: -0.040,
            fg_sigmoid: -0.009,
            punt_sigmoid: -0.006,
            goal_to_go_yardline: 0.002,
            log_goal_to_go_yardline: 0.070,
            yards_to_go_yardline: -0.066,
            log_yards_to_go_yardline: -0.109,
            yardline_4th: 0.028,
            log_yardline_4th: -0.001,
            yardline_not_4th: -0.092,
            log_yardline_not_4th: -0.039,
            inside_2m_warning: 0.062,
            garbage_time_win: 0.023,
            garbage_time_loss: 0.006,
            clock_running: -0.169,
            inv_half_minutes: -0.123,
            log_inv_half_minutes: 0.171,
            inv_game_minutes: 0.179,
            log_inv_game_minutes: 0.034,
            possession_diff: -0.116,
            fg_possession_diff: 0.006,
            possdiff_per_minute: 0.057,
            fgpossdiff_per_minute: 0.202,
            clock_runs_pdpm: -0.008,
            clock_runs_fgpdpm: -0.040,
            clock_runs_pdpm2: 0.067,
            clock_runs_fgpdpm2: -0.058,
            off_timeouts_remaining_0: 0.051,
            off_timeouts_remaining_1: 0.016,
            off_timeouts_remaining_2: -0.102,
            off_timeouts_remaining_3: 0.035,
            clock_runs_pdpm_off0to: 0.026,
            clock_runs_pdpm_off1to: 0.028,
            clock_runs_pdpm_off2to: -0.030,
            clock_runs_pdpm_off3to: -0.031,
            def_timeouts_remaining_0: 0.037,
            def_timeouts_remaining_1: -0.029,
            def_timeouts_remaining_2: -0.019,
            def_timeouts_remaining_3: 0.011,
            clock_runs_pdpm_def0to: 0.001,
            clock_runs_pdpm_def1to: 0.017,
            clock_runs_pdpm_def2to: 0.056,
            clock_runs_pdpm_def3to: -0.081,
            offense_penalty_z: 0.073,
            defense_penalty_z: -0.103,
            off_def_penalty_z: -0.080,
            qb_scramble: 2.114,
            sack: -0.040,
            throwaway: -0.032,
            target_incomplete: -0.243,
            target_complete: -1.562,
            interception: -0.237,
            yards_gained_div10: 0.591,
            yards_gained_div10_sq: -0.173,
            loss_of_down: 0.0,
        }
    }

}