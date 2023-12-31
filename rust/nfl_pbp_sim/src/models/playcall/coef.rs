use crate::models::playcall::PlaycallModel;

impl PlaycallModel {

    pub fn offensive_penalty_coefs() -> PlaycallModel {
        PlaycallModel {
            intercept: -2.9578,
            is_offense_home: -0.0373,
            offense_log_pass_prob: -0.0214,
            defense_log_pass_prob: 0.0372,
            off_def_lpp: 0.0461,
            off_lpp_rz: -0.0279,
            def_lpp_rz: 0.0140,
            off_def_lpp_rz: 0.0019,
            off_lpp_outside_rz: -0.0433,
            off_lpp_inside_rz: 0.0130,
            def_lpp_outside_rz: -0.0040,
            def_lpp_inside_rz: 0.1211,
            off_lpp_pdpm: -0.1337,
            def_lpp_pdpm: 0.2216,
            off_lpp_rz_pdpm: 0.0439,
            def_lpp_rz_pdpm: -0.0959,
            down_1: -0.0184,
            down_2: -0.3042,
            down_3: -0.1267,
            down_4: 0.3941,
            goal_to_go: -0.1482,
            z_ydstogo: 0.0422,
            ydstogo_pct: -0.0340,
            log_ydstogo_pct: -0.0865,
            to_go_1st: -0.0642,
            to_go_2nd: 0.1110,
            to_go_3rd: 0.0631,
            to_go_4th: -0.0216,
            log_to_go_1st: 0.0489,
            log_to_go_2nd: 0.0955,
            log_to_go_3rd: 0.0740,
            log_to_go_4th: -0.3049,
            fp_1st: -0.2499,
            fp_2nd: 0.2138,
            fp_3rd: 0.2109,
            fp_4th: -0.2125,
            yardline_fgsig_4th: 0.0540,
            yardline_puntsig_4th: 0.0525,
            yardline_pct: -0.0583,
            yardline_pct_sq: -0.2478,
            log_yardline_pct: 0.0163,
            fg_sigmoid: -0.0004,
            punt_sigmoid: -0.0027,
            goal_to_go_yardline: -0.0530,
            log_goal_to_go_yardline: -0.0221,
            yards_to_go_yardline: -0.0053,
            log_yards_to_go_yardline: 0.0384,
            yardline_4th: 0.0091,
            log_yardline_4th: 0.0432,
            yardline_not_4th: -0.0674,
            log_yardline_not_4th: -0.0269,
            inside_2m_warning: -0.2422,
            garbage_time_win: -0.4223,
            garbage_time_loss: -0.4475,
            clock_running: 0.0448,
            inv_half_minutes: -0.0083,
            log_inv_half_minutes: 0.0657,
            inv_game_minutes: -0.0520,
            log_inv_game_minutes: -0.0157,
            possession_diff: -0.0305,
            fg_possession_diff: 0.0468,
            possdiff_per_minute: -0.0091,
            fgpossdiff_per_minute: -0.0419,
            clock_runs_pdpm: 0.1540,
            clock_runs_fgpdpm: -0.0683,
            clock_runs_pdpm2: -0.0071,
            clock_runs_fgpdpm2: 0.0045,
            off_timeouts_remaining_0: 0.0265,
            off_timeouts_remaining_1: -0.0546,
            off_timeouts_remaining_2: 0.0011,
            off_timeouts_remaining_3: -0.0281,
            clock_runs_pdpm_off0to: 0.0618,
            clock_runs_pdpm_off1to: 0.0521,
            clock_runs_pdpm_off2to: 0.0132,
            clock_runs_pdpm_off3to: 0.0269,
            def_timeouts_remaining_0: -0.0618,
            def_timeouts_remaining_1: 0.0018,
            def_timeouts_remaining_2: -0.0091,
            def_timeouts_remaining_3: 0.0140,
            clock_runs_pdpm_def0to: 0.2161,
            clock_runs_pdpm_def1to: -0.0543,
            clock_runs_pdpm_def2to: -0.0195,
            clock_runs_pdpm_def3to: 0.0116,
            offense_penalty_z: -0.1804,
            defense_penalty_z: 0.0044,
            off_def_penalty_z: 0.0062,
        }
    }


    pub fn defensive_penalty_coefs() -> PlaycallModel {
        PlaycallModel {
            intercept: -2.9553,
            is_offense_home: 0.1607,
            offense_log_pass_prob: 0.1638,
            defense_log_pass_prob: 0.2507,
            off_def_lpp: -0.2139,
            off_lpp_rz: 0.2308,
            def_lpp_rz: -0.0519,
            off_def_lpp_rz: 0.0011,
            off_lpp_outside_rz: 0.0508,
            off_lpp_inside_rz: 0.3117,
            def_lpp_outside_rz: -0.0111,
            def_lpp_inside_rz: -0.1660,
            off_lpp_pdpm: -0.1223,
            def_lpp_pdpm: 0.0143,
            off_lpp_rz_pdpm: 0.0380,
            def_lpp_rz_pdpm: 0.1341,
            down_1: -0.1006,
            down_2: 0.2158,
            down_3: -0.0329,
            down_4: -0.1355,
            goal_to_go: 0.0794,
            z_ydstogo: -0.1696,
            ydstogo_pct: -0.1380,
            log_ydstogo_pct: 0.2602,
            to_go_1st: -0.2016,
            to_go_2nd: -0.4439,
            to_go_3rd: 0.7196,
            to_go_4th: -0.1248,
            log_to_go_1st: 0.0500,
            log_to_go_2nd: 0.1621,
            log_to_go_3rd: 0.2792,
            log_to_go_4th: -0.2312,
            fp_1st: -0.3860,
            fp_2nd: -0.2413,
            fp_3rd: 1.0785,
            fp_4th: -0.3268,
            yardline_fgsig_4th: 0.0441,
            yardline_puntsig_4th: -0.0155,
            yardline_pct: 0.0296,
            yardline_pct_sq: -0.4601,
            log_yardline_pct: -0.0331,
            fg_sigmoid: -0.0076,
            punt_sigmoid: 0.0109,
            goal_to_go_yardline: 0.0035,
            log_goal_to_go_yardline: -0.0125,
            yards_to_go_yardline: 0.0261,
            log_yards_to_go_yardline: -0.0206,
            yardline_4th: 0.0555,
            log_yardline_4th: 0.0587,
            yardline_not_4th: -0.0259,
            log_yardline_not_4th: -0.0919,
            inside_2m_warning: 0.0079,
            garbage_time_win: -0.2591,
            garbage_time_loss: -0.2925,
            clock_running: 0.0466,
            inv_half_minutes: -0.1014,
            log_inv_half_minutes: 0.1078,
            inv_game_minutes: -0.0496,
            log_inv_game_minutes: -0.0289,
            possession_diff: -0.0262,
            fg_possession_diff: 0.0054,
            possdiff_per_minute: -0.0348,
            fgpossdiff_per_minute: -0.0154,
            clock_runs_pdpm: 0.0189,
            clock_runs_fgpdpm: -0.1106,
            clock_runs_pdpm2: 0.0244,
            clock_runs_fgpdpm2: -0.0168,
            off_timeouts_remaining_0: 0.1336,
            off_timeouts_remaining_1: -0.0767,
            off_timeouts_remaining_2: -0.0728,
            off_timeouts_remaining_3: -0.0372,
            clock_runs_pdpm_off0to: 0.0130,
            clock_runs_pdpm_off1to: 0.1099,
            clock_runs_pdpm_off2to: -0.0680,
            clock_runs_pdpm_off3to: -0.0361,
            def_timeouts_remaining_0: -0.1207,
            def_timeouts_remaining_1: 0.0562,
            def_timeouts_remaining_2: 0.0057,
            def_timeouts_remaining_3: 0.0057,
            clock_runs_pdpm_def0to: -0.0688,
            clock_runs_pdpm_def1to: 0.1136,
            clock_runs_pdpm_def2to: -0.0080,
            clock_runs_pdpm_def3to: -0.0180,
            offense_penalty_z: 0.0044,
            defense_penalty_z: -0.2071,
            off_def_penalty_z: -0.0017,
        }
    }


    pub fn fg_attempt_coefs() -> PlaycallModel {
        PlaycallModel {
            intercept: -3.7256,
            is_offense_home: -0.0038,
            offense_log_pass_prob: -0.0711,
            defense_log_pass_prob: 0.0081,
            off_def_lpp: 0.0291,
            off_lpp_rz: 0.2107,
            def_lpp_rz: -0.2579,
            off_def_lpp_rz: 0.0209,
            off_lpp_outside_rz: -0.2682,
            off_lpp_inside_rz: 0.4746,
            def_lpp_outside_rz: -0.2280,
            def_lpp_inside_rz: 0.0992,
            off_lpp_pdpm: -0.1326,
            def_lpp_pdpm: 0.3825,
            off_lpp_rz_pdpm: 0.0152,
            def_lpp_rz_pdpm: 0.3558,
            down_1: -0.7780,
            down_2: -0.5066,
            down_3: -0.3027,
            down_4: 1.5883,
            goal_to_go: 0.0071,
            z_ydstogo: -0.4258,
            ydstogo_pct: -0.2120,
            log_ydstogo_pct: 0.8940,
            to_go_1st: -0.2175,
            to_go_2nd: -0.2825,
            to_go_3rd: -0.1362,
            to_go_4th: 1.0831,
            log_to_go_1st: -0.3052,
            log_to_go_2nd: 0.0446,
            log_to_go_3rd: -0.0890,
            log_to_go_4th: 1.2436,
            fp_1st: -0.3403,
            fp_2nd: -0.2232,
            fp_3rd: -0.2002,
            fp_4th: 0.5636,
            yardline_fgsig_4th: 1.9169,
            yardline_puntsig_4th: -0.4087,
            yardline_pct: -0.5593,
            yardline_pct_sq: -0.5171,
            log_yardline_pct: 0.4136,
            fg_sigmoid: 3.0998,
            punt_sigmoid: 0.0540,
            goal_to_go_yardline: -0.0024,
            log_goal_to_go_yardline: 0.1792,
            yards_to_go_yardline: -0.5569,
            log_yards_to_go_yardline: 0.2344,
            yardline_4th: 0.0003,
            log_yardline_4th: -0.0552,
            yardline_not_4th: -0.5596,
            log_yardline_not_4th: 0.4688,
            inside_2m_warning: -0.6725,
            garbage_time_win: -1.0498,
            garbage_time_loss: -1.7732,
            clock_running: -0.4773,
            inv_half_minutes: 1.4298,
            log_inv_half_minutes: -0.3064,
            inv_game_minutes: -1.1866,
            log_inv_game_minutes: -0.2227,
            possession_diff: -0.3350,
            fg_possession_diff: 0.6226,
            possdiff_per_minute: -0.8930,
            fgpossdiff_per_minute: 1.2642,
            clock_runs_pdpm: -0.0001,
            clock_runs_fgpdpm: 0.0973,
            clock_runs_pdpm2: 0.1494,
            clock_runs_fgpdpm2: -0.1815,
            off_timeouts_remaining_0: 0.3201,
            off_timeouts_remaining_1: -0.1784,
            off_timeouts_remaining_2: -0.0912,
            off_timeouts_remaining_3: -0.0497,
            clock_runs_pdpm_off0to: 0.2139,
            clock_runs_pdpm_off1to: -0.0924,
            clock_runs_pdpm_off2to: -0.1147,
            clock_runs_pdpm_off3to: -0.0069,
            def_timeouts_remaining_0: 0.3293,
            def_timeouts_remaining_1: 0.0398,
            def_timeouts_remaining_2: -0.1942,
            def_timeouts_remaining_3: -0.1741,
            clock_runs_pdpm_def0to: -0.1928,
            clock_runs_pdpm_def1to: -0.1947,
            clock_runs_pdpm_def2to: 0.3349,
            clock_runs_pdpm_def3to: 0.0525,
            offense_penalty_z: 0.0006,
            defense_penalty_z: -0.0266,
            off_def_penalty_z: 0.0169,
        }
    }


    pub fn punt_coefs() -> PlaycallModel {
        PlaycallModel {
            intercept: -1.8012,
            is_offense_home: -0.0909,
            offense_log_pass_prob: -0.3026,
            defense_log_pass_prob: -0.1428,
            off_def_lpp: 0.2286,
            off_lpp_rz: 0.5591,
            def_lpp_rz: -0.0088,
            off_def_lpp_rz: -0.2335,
            off_lpp_outside_rz: -0.4499,
            off_lpp_inside_rz: 0.1532,
            def_lpp_outside_rz: -0.2944,
            def_lpp_inside_rz: 0.1558,
            off_lpp_pdpm: 0.0757,
            def_lpp_pdpm: 0.0297,
            off_lpp_rz_pdpm: -0.3730,
            def_lpp_rz_pdpm: -0.1974,
            down_1: -0.8871,
            down_2: -0.6918,
            down_3: -0.4872,
            down_4: 2.0589,
            goal_to_go: -0.0738,
            z_ydstogo: -0.2733,
            ydstogo_pct: -0.1438,
            log_ydstogo_pct: 1.0776,
            to_go_1st: -0.3193,
            to_go_2nd: -0.2875,
            to_go_3rd: -0.2129,
            to_go_4th: 0.7456,
            log_to_go_1st: -0.0207,
            log_to_go_2nd: 0.1030,
            log_to_go_3rd: 0.1105,
            log_to_go_4th: 0.8847,
            fp_1st: -0.3228,
            fp_2nd: -0.2334,
            fp_3rd: -0.1559,
            fp_4th: 0.8479,
            yardline_fgsig_4th: -1.5185,
            yardline_puntsig_4th: -1.3962,
            yardline_pct: 0.2384,
            yardline_pct_sq: -0.2224,
            log_yardline_pct: 1.5051,
            fg_sigmoid: 1.4581,
            punt_sigmoid: 1.5413,
            goal_to_go_yardline: -0.0066,
            log_goal_to_go_yardline: 0.1992,
            yards_to_go_yardline: 0.2450,
            log_yards_to_go_yardline: 1.3059,
            yardline_4th: 1.1836,
            log_yardline_4th: -0.1184,
            yardline_not_4th: -0.9452,
            log_yardline_not_4th: 1.6235,
            inside_2m_warning: 0.2852,
            garbage_time_win: -0.6046,
            garbage_time_loss: 0.6387,
            clock_running: -0.3314,
            inv_half_minutes: -0.5158,
            log_inv_half_minutes: 0.0572,
            inv_game_minutes: -0.4291,
            log_inv_game_minutes: -0.9770,
            possession_diff: 0.3406,
            fg_possession_diff: 0.1295,
            possdiff_per_minute: 0.1504,
            fgpossdiff_per_minute: 0.8176,
            clock_runs_pdpm: -0.1260,
            clock_runs_fgpdpm: 0.1747,
            clock_runs_pdpm2: 0.0069,
            clock_runs_fgpdpm2: -0.0776,
            off_timeouts_remaining_0: -0.8278,
            off_timeouts_remaining_1: -0.1261,
            off_timeouts_remaining_2: 0.3716,
            off_timeouts_remaining_3: 0.5752,
            clock_runs_pdpm_off0to: -0.1403,
            clock_runs_pdpm_off1to: 0.2887,
            clock_runs_pdpm_off2to: -0.2085,
            clock_runs_pdpm_off3to: -0.0659,
            def_timeouts_remaining_0: 0.6741,
            def_timeouts_remaining_1: -0.1679,
            def_timeouts_remaining_2: -0.3339,
            def_timeouts_remaining_3: -0.1794,
            clock_runs_pdpm_def0to: 0.3544,
            clock_runs_pdpm_def1to: -0.1970,
            clock_runs_pdpm_def2to: -0.2807,
            clock_runs_pdpm_def3to: -0.0026,
            offense_penalty_z: 0.0242,
            defense_penalty_z: -0.1517,
            off_def_penalty_z: 0.0216,
        }
    }


    pub fn qb_spike_coefs() -> PlaycallModel {
        PlaycallModel {
            intercept: -9.4030,
            is_offense_home: 0.2510,
            offense_log_pass_prob: -0.2000,
            defense_log_pass_prob: -0.0301,
            off_def_lpp: 0.1133,
            off_lpp_rz: -0.1696,
            def_lpp_rz: -0.1162,
            off_def_lpp_rz: 0.1680,
            off_lpp_outside_rz: -0.2934,
            off_lpp_inside_rz: 0.1025,
            def_lpp_outside_rz: -0.1263,
            def_lpp_inside_rz: 0.1598,
            off_lpp_pdpm: 0.2115,
            def_lpp_pdpm: -0.1696,
            off_lpp_rz_pdpm: -0.0197,
            def_lpp_rz_pdpm: 0.0210,
            down_1: 0.7327,
            down_2: -0.1021,
            down_3: -0.4335,
            down_4: -0.1953,
            goal_to_go: -0.1657,
            z_ydstogo: 0.3491,
            ydstogo_pct: 0.1763,
            log_ydstogo_pct: -0.1938,
            to_go_1st: 0.2486,
            to_go_2nd: -0.0108,
            to_go_3rd: -0.1391,
            to_go_4th: -0.0666,
            log_to_go_1st: 0.0928,
            log_to_go_2nd: -0.1403,
            log_to_go_3rd: -0.1490,
            log_to_go_4th: 0.0027,
            fp_1st: 0.2949,
            fp_2nd: -0.1667,
            fp_3rd: -0.1822,
            fp_4th: -0.0517,
            yardline_fgsig_4th: 0.3298,
            yardline_puntsig_4th: 0.3820,
            yardline_pct: -0.2238,
            yardline_pct_sq: -0.3810,
            log_yardline_pct: 0.0477,
            fg_sigmoid: 0.0306,
            punt_sigmoid: -0.0144,
            goal_to_go_yardline: -0.0269,
            log_goal_to_go_yardline: 0.1635,
            yards_to_go_yardline: -0.1969,
            log_yards_to_go_yardline: -0.1158,
            yardline_4th: -0.0853,
            log_yardline_4th: 0.1668,
            yardline_not_4th: -0.1385,
            log_yardline_not_4th: -0.1191,
            inside_2m_warning: 0.2745,
            garbage_time_win: -0.0567,
            garbage_time_loss: -0.5163,
            clock_running: 3.9537,
            inv_half_minutes: 0.2686,
            log_inv_half_minutes: 1.7011,
            inv_game_minutes: -0.2897,
            log_inv_game_minutes: 0.3248,
            possession_diff: 0.0896,
            fg_possession_diff: -0.1009,
            possdiff_per_minute: 0.2736,
            fgpossdiff_per_minute: -0.0365,
            clock_runs_pdpm: -0.2765,
            clock_runs_fgpdpm: 0.2600,
            clock_runs_pdpm2: 0.0094,
            clock_runs_fgpdpm2: -0.0064,
            off_timeouts_remaining_0: 1.6374,
            off_timeouts_remaining_1: 0.0706,
            off_timeouts_remaining_2: -0.8805,
            off_timeouts_remaining_3: -0.8257,
            clock_runs_pdpm_off0to: -0.1000,
            clock_runs_pdpm_off1to: -0.1092,
            clock_runs_pdpm_off2to: -0.0982,
            clock_runs_pdpm_off3to: 0.0310,
            def_timeouts_remaining_0: -0.0741,
            def_timeouts_remaining_1: -0.0248,
            def_timeouts_remaining_2: 0.1107,
            def_timeouts_remaining_3: -0.0100,
            clock_runs_pdpm_def0to: -0.0452,
            clock_runs_pdpm_def1to: -0.1034,
            clock_runs_pdpm_def2to: -0.0185,
            clock_runs_pdpm_def3to: -0.1094,
            offense_penalty_z: -0.0662,
            defense_penalty_z: -0.0269,
            off_def_penalty_z: -0.0176,
        }
    }


    pub fn qb_kneel_coefs() -> PlaycallModel {
        PlaycallModel {
            intercept: -7.0940,
            is_offense_home: -0.0524,
            offense_log_pass_prob: -0.0210,
            defense_log_pass_prob: -0.1370,
            off_def_lpp: 0.0542,
            off_lpp_rz: 0.0310,
            def_lpp_rz: 0.1062,
            off_def_lpp_rz: -0.0891,
            off_lpp_outside_rz: 0.3439,
            off_lpp_inside_rz: -0.2790,
            def_lpp_outside_rz: 0.2215,
            def_lpp_inside_rz: -0.3107,
            off_lpp_pdpm: 0.1212,
            def_lpp_pdpm: -0.2558,
            off_lpp_rz_pdpm: 0.1663,
            def_lpp_rz_pdpm: 0.0224,
            down_1: 1.2332,
            down_2: 0.3389,
            down_3: -0.7815,
            down_4: -0.7862,
            goal_to_go: 0.0978,
            z_ydstogo: -0.0898,
            ydstogo_pct: -0.0405,
            log_ydstogo_pct: 1.0049,
            to_go_1st: 0.4957,
            to_go_2nd: -0.0852,
            to_go_3rd: -0.4898,
            to_go_4th: -0.4189,
            log_to_go_1st: 0.0365,
            log_to_go_2nd: 0.2606,
            log_to_go_3rd: 0.3557,
            log_to_go_4th: 0.3521,
            fp_1st: 0.6610,
            fp_2nd: 0.2334,
            fp_3rd: -0.2822,
            fp_4th: -0.2400,
            yardline_fgsig_4th: 0.4974,
            yardline_puntsig_4th: 0.1598,
            yardline_pct: 0.9143,
            yardline_pct_sq: 1.5339,
            log_yardline_pct: -0.1925,
            fg_sigmoid: 0.0005,
            punt_sigmoid: -0.0274,
            goal_to_go_yardline: 0.0026,
            log_goal_to_go_yardline: -0.1079,
            yards_to_go_yardline: 0.9117,
            log_yards_to_go_yardline: -0.0846,
            yardline_4th: -0.4032,
            log_yardline_4th: 0.1820,
            yardline_not_4th: 1.3175,
            log_yardline_not_4th: -0.3745,
            inside_2m_warning: 2.6365,
            garbage_time_win: 0.2495,
            garbage_time_loss: 1.7567,
            clock_running: 0.2780,
            inv_half_minutes: 0.2310,
            log_inv_half_minutes: 1.6272,
            inv_game_minutes: -1.4702,
            log_inv_game_minutes: 1.4049,
            possession_diff: 0.1940,
            fg_possession_diff: 0.1070,
            possdiff_per_minute: 0.8835,
            fgpossdiff_per_minute: -0.0708,
            clock_runs_pdpm: 0.0716,
            clock_runs_fgpdpm: 0.3479,
            clock_runs_pdpm2: -0.0308,
            clock_runs_fgpdpm2: -0.0062,
            off_timeouts_remaining_0: -0.6600,
            off_timeouts_remaining_1: -0.0279,
            off_timeouts_remaining_2: 0.2692,
            off_timeouts_remaining_3: 0.4231,
            clock_runs_pdpm_off0to: -0.1150,
            clock_runs_pdpm_off1to: 0.1433,
            clock_runs_pdpm_off2to: 0.0163,
            clock_runs_pdpm_off3to: 0.0270,
            def_timeouts_remaining_0: 1.5027,
            def_timeouts_remaining_1: 0.2467,
            def_timeouts_remaining_2: -0.6872,
            def_timeouts_remaining_3: -1.0577,
            clock_runs_pdpm_def0to: 0.2611,
            clock_runs_pdpm_def1to: -0.0964,
            clock_runs_pdpm_def2to: -0.0332,
            clock_runs_pdpm_def3to: -0.0600,
            offense_penalty_z: 0.0540,
            defense_penalty_z: -0.0144,
            off_def_penalty_z: 0.0365,
        }
    }


    pub fn qb_dropback_coefs() -> PlaycallModel {
        PlaycallModel {
            intercept: 4.0606,
            is_offense_home: 0.0785,
            offense_log_pass_prob: 0.6302,
            defense_log_pass_prob: 0.4455,
            off_def_lpp: -0.5859,
            off_lpp_rz: 0.1719,
            def_lpp_rz: -0.1143,
            off_def_lpp_rz: -0.0928,
            off_lpp_outside_rz: 1.8749,
            off_lpp_inside_rz: 2.0079,
            def_lpp_outside_rz: 1.7339,
            def_lpp_inside_rz: 1.5308,
            off_lpp_pdpm: -0.3082,
            def_lpp_pdpm: -0.3180,
            off_lpp_rz_pdpm: 0.3696,
            def_lpp_rz_pdpm: 0.1471,
            down_1: 0.0556,
            down_2: 0.6590,
            down_3: -1.9553,
            down_4: 1.3865,
            goal_to_go: 0.0922,
            z_ydstogo: -0.4368,
            ydstogo_pct: -0.0727,
            log_ydstogo_pct: 1.5058,
            to_go_1st: -1.4814,
            to_go_2nd: -3.0040,
            to_go_3rd: 3.7781,
            to_go_4th: 1.0192,
            log_to_go_1st: -0.0713,
            log_to_go_2nd: -0.6158,
            log_to_go_3rd: 0.9838,
            log_to_go_4th: 1.2092,
            fp_1st: -4.0997,
            fp_2nd: -1.6975,
            fp_3rd: 4.3362,
            fp_4th: 0.7072,
            yardline_fgsig_4th: 0.0625,
            yardline_puntsig_4th: -0.0424,
            yardline_pct: 0.8317,
            yardline_pct_sq: -1.8284,
            log_yardline_pct: -0.0884,
            fg_sigmoid: -0.0096,
            punt_sigmoid: 0.0112,
            goal_to_go_yardline: -0.0470,
            log_goal_to_go_yardline: -0.0744,
            yards_to_go_yardline: 0.8787,
            log_yards_to_go_yardline: -0.0140,
            yardline_4th: 0.4511,
            log_yardline_4th: 0.0108,
            yardline_not_4th: 0.3806,
            log_yardline_not_4th: -0.0992,
            inside_2m_warning: 0.5173,
            garbage_time_win: -0.8669,
            garbage_time_loss: -1.0242,
            clock_running: 0.1847,
            inv_half_minutes: 0.3412,
            log_inv_half_minutes: 0.0818,
            inv_game_minutes: -0.9533,
            log_inv_game_minutes: 0.0466,
            possession_diff: -0.0711,
            fg_possession_diff: -0.0549,
            possdiff_per_minute: -1.2649,
            fgpossdiff_per_minute: 0.1659,
            clock_runs_pdpm: 0.0868,
            clock_runs_fgpdpm: -0.4686,
            clock_runs_pdpm2: 0.0421,
            clock_runs_fgpdpm2: 0.0166,
            off_timeouts_remaining_0: 0.5498,
            off_timeouts_remaining_1: -0.0140,
            off_timeouts_remaining_2: -0.1766,
            off_timeouts_remaining_3: -0.2135,
            clock_runs_pdpm_off0to: 0.1493,
            clock_runs_pdpm_off1to: -0.1792,
            clock_runs_pdpm_off2to: -0.0174,
            clock_runs_pdpm_off3to: 0.1342,
            def_timeouts_remaining_0: -0.5921,
            def_timeouts_remaining_1: 0.1586,
            def_timeouts_remaining_2: 0.2896,
            def_timeouts_remaining_3: 0.2896,
            clock_runs_pdpm_def0to: -0.2436,
            clock_runs_pdpm_def1to: 0.0894,
            clock_runs_pdpm_def2to: 0.0279,
            clock_runs_pdpm_def3to: 0.2131,
            offense_penalty_z: 0.0083,
            defense_penalty_z: 0.0198,
            off_def_penalty_z: 0.0071,
        }
    }

}