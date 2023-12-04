use crate::models::air_yards::AirYardsModel;

impl AirYardsModel {

    pub fn is_pos_air_yards_coef() -> AirYardsModel {
        AirYardsModel {
            intercept: 4.1985,
            clock_running: 0.1053,
            inv_half_minutes: 0.1161,
            log_inv_half_minutes: 0.0503,
            inv_game_minutes: -0.0883,
            log_inv_game_minutes: 0.0801,
            possession_diff: -0.0385,
            fg_possession_diff: -0.0048,
            possdiff_per_minute: -0.1692,
            fgpossdiff_per_minute: -0.0187,
            clock_runs_pdpm: -0.0237,
            clock_runs_fgpdpm: -0.1661,
            clock_runs_pdpm2: 0.0012,
            clock_runs_fgpdpm2: 0.0102,
            is_offense_home: 0.0674,
            offense_log_pass_prob: -0.2009,
            defense_log_pass_prob: 0.1582,
            off_def_lpp: 0.0512,
            off_lpp_rz: 0.1744,
            def_lpp_rz: 0.0026,
            off_def_lpp_rz: 0.0404,
            off_lpp_outside_rz: 0.3256,
            off_lpp_inside_rz: 0.0017,
            def_lpp_outside_rz: -0.2146,
            def_lpp_inside_rz: 0.1695,
            off_lpp_pdpm: -0.2555,
            def_lpp_pdpm: -0.1030,
            off_lpp_rz_pdpm: -0.1500,
            def_lpp_rz_pdpm: 0.2265,
            down_1: 0.0385,
            down_2: 0.2900,
            down_3: -0.9540,
            down_4: 0.6279,
            goal_to_go: -0.0125,
            z_ydstogo: -0.2546,
            ydstogo_pct: -0.1250,
            log_ydstogo_pct: 0.4084,
            to_go_1st: -0.2564,
            to_go_2nd: -0.7371,
            to_go_3rd: 1.5789,
            to_go_4th: 0.1478,
            log_to_go_1st: 0.0234,
            log_to_go_2nd: -0.0946,
            log_to_go_3rd: 0.1189,
            log_to_go_4th: 0.3607,
            fp_1st: -0.9708,
            fp_2nd: -0.8814,
            fp_3rd: 1.5249,
            fp_4th: 0.2439,
            yardline_fgsig_4th: -0.0049,
            yardline_puntsig_4th: -0.0398,
            yardline_pct: 0.1101,
            yardline_pct_sq: 0.0154,
            log_yardline_pct: -0.0013,
            fg_sigmoid: 0.0045,
            punt_sigmoid: 0.0127,
            goal_to_go_yardline: -0.0693,
            log_goal_to_go_yardline: -0.0336,
            yards_to_go_yardline: 0.1794,
            log_yards_to_go_yardline: 0.0324,
            yardline_4th: 0.3178,
            log_yardline_4th: 0.0717,
            yardline_not_4th: -0.2078,
            log_yardline_not_4th: -0.0729,
            inside_2m_warning: 0.1140,
            garbage_time_win: -0.3722,
            garbage_time_loss: -0.1367,
            qb_scramble_rate: 0.2534,
            log_qb_scramble: 0.0816,
            qb_prob_sack_given_hit: -0.2296,
            log_qbps: -0.0609,
            offense_pass_rush_z: -0.0243,
            defense_pass_rush_z: 0.0011,
            off_def_pass_rush_z: 0.0088,
            defense_completion_z: 0.0117,
            defense_interception_z: -0.0046,
            def_comp_scramble: 0.0019,
            def_int_scramble: 0.0019,
            olpz_qbps: 0.1004,
            dlpz_qbps: 0.0182,
            olpz_scramble: -0.0258,
            dlpz_scramble: 0.0019,
            qb_ay_oe: 0.2961,
            qb_ay_std: -0.2444,
            log_qb_mean_ay: 0.1903,
            log_qb_std_ay: 0.0748,
        }
    }


    pub fn neg_air_yards_coef() -> AirYardsModel {
        AirYardsModel {
            intercept: 0.8747,
            clock_running: -0.0023,
            inv_half_minutes: -0.0022,
            log_inv_half_minutes: -0.0030,
            inv_game_minutes: -0.0018,
            log_inv_game_minutes: -0.0057,
            possession_diff: 0.0023,
            fg_possession_diff: 0.0021,
            possdiff_per_minute: 0.0021,
            fgpossdiff_per_minute: 0.0005,
            clock_runs_pdpm: 0.0011,
            clock_runs_fgpdpm: 0.0005,
            clock_runs_pdpm2: 0.0007,
            clock_runs_fgpdpm2: -0.0011,
            is_offense_home: -0.0000,
            offense_log_pass_prob: -0.0002,
            defense_log_pass_prob: 0.0001,
            off_def_lpp: 0.0001,
            off_lpp_rz: -0.0011,
            def_lpp_rz: -0.0004,
            off_def_lpp_rz: 0.0008,
            off_lpp_outside_rz: -0.0010,
            off_lpp_inside_rz: 0.0006,
            def_lpp_outside_rz: -0.0006,
            def_lpp_inside_rz: 0.0007,
            off_lpp_pdpm: -0.0011,
            def_lpp_pdpm: -0.0011,
            off_lpp_rz_pdpm: -0.0010,
            def_lpp_rz_pdpm: -0.0011,
            down_1: -0.0012,
            down_2: 0.0043,
            down_3: -0.0028,
            down_4: -0.0004,
            goal_to_go: -0.0013,
            z_ydstogo: 0.0106,
            ydstogo_pct: 0.0053,
            log_ydstogo_pct: 0.0090,
            to_go_1st: -0.0008,
            to_go_2nd: 0.0011,
            to_go_3rd: -0.0025,
            to_go_4th: -0.0003,
            log_to_go_1st: 0.0011,
            log_to_go_2nd: 0.0018,
            log_to_go_3rd: 0.0054,
            log_to_go_4th: 0.0007,
            fp_1st: -0.0004,
            fp_2nd: 0.0017,
            fp_3rd: -0.0005,
            fp_4th: -0.0000,
            yardline_fgsig_4th: 0.0011,
            yardline_puntsig_4th: 0.0041,
            yardline_pct: 0.0004,
            yardline_pct_sq: 0.0000,
            log_yardline_pct: 0.0041,
            fg_sigmoid: -0.0010,
            punt_sigmoid: 0.0011,
            goal_to_go_yardline: -0.0000,
            log_goal_to_go_yardline: 0.0051,
            yards_to_go_yardline: 0.0004,
            log_yards_to_go_yardline: -0.0010,
            yardline_4th: -0.0001,
            log_yardline_4th: 0.0008,
            yardline_not_4th: 0.0005,
            log_yardline_not_4th: 0.0033,
            inside_2m_warning: -0.0009,
            garbage_time_win: 0.0000,
            garbage_time_loss: 0.0000,
            qb_scramble_rate: 0.0000,
            log_qb_scramble: 0.0002,
            qb_prob_sack_given_hit: -0.0003,
            log_qbps: -0.0007,
            offense_pass_rush_z: -0.0012,
            defense_pass_rush_z: -0.0001,
            off_def_pass_rush_z: -0.0002,
            defense_completion_z: -0.0010,
            defense_interception_z: 0.0006,
            def_comp_scramble: 0.0001,
            def_int_scramble: 0.0001,
            olpz_qbps: 0.0016,
            dlpz_qbps: 0.0002,
            olpz_scramble: 0.0040,
            dlpz_scramble: 0.0001,
            qb_ay_oe: -0.0056,
            qb_ay_std: 0.0007,
            log_qb_mean_ay: -0.0007,
            log_qb_std_ay: 0.0001,
        }
    }


    pub fn pos_air_yards_coef() -> AirYardsModel {
        AirYardsModel {
            intercept: 2.1504,
            clock_running: -0.0024,
            inv_half_minutes: 0.0096,
            log_inv_half_minutes: 0.0045,
            inv_game_minutes: 0.0070,
            log_inv_game_minutes: 0.0094,
            possession_diff: 0.0056,
            fg_possession_diff: 0.0008,
            possdiff_per_minute: -0.0009,
            fgpossdiff_per_minute: 0.0018,
            clock_runs_pdpm: 0.0022,
            clock_runs_fgpdpm: 0.0010,
            clock_runs_pdpm2: 0.0016,
            clock_runs_fgpdpm2: -0.0022,
            is_offense_home: -0.0021,
            offense_log_pass_prob: 0.0004,
            defense_log_pass_prob: -0.0003,
            off_def_lpp: -0.0001,
            off_lpp_rz: 0.0009,
            def_lpp_rz: 0.0002,
            off_def_lpp_rz: -0.0006,
            off_lpp_outside_rz: -0.0101,
            off_lpp_inside_rz: 0.0110,
            def_lpp_outside_rz: -0.0109,
            def_lpp_inside_rz: 0.0111,
            off_lpp_pdpm: 0.0003,
            def_lpp_pdpm: 0.0003,
            off_lpp_rz_pdpm: 0.0004,
            def_lpp_rz_pdpm: 0.0005,
            down_1: 0.0064,
            down_2: -0.0154,
            down_3: 0.0079,
            down_4: 0.0011,
            goal_to_go: -0.0131,
            z_ydstogo: 0.0236,
            ydstogo_pct: 0.0118,
            log_ydstogo_pct: 0.0254,
            to_go_1st: 0.0013,
            to_go_2nd: -0.0060,
            to_go_3rd: -0.0015,
            to_go_4th: -0.0011,
            log_to_go_1st: 0.0040,
            log_to_go_2nd: 0.0010,
            log_to_go_3rd: 0.0150,
            log_to_go_4th: 0.0054,
            fp_1st: 0.0032,
            fp_2nd: -0.0052,
            fp_3rd: 0.0046,
            fp_4th: 0.0008,
            yardline_fgsig_4th: -0.0098,
            yardline_puntsig_4th: -0.0020,
            yardline_pct: 0.0031,
            yardline_pct_sq: -0.0015,
            log_yardline_pct: 0.0395,
            fg_sigmoid: 0.0072,
            punt_sigmoid: 0.0115,
            goal_to_go_yardline: -0.0005,
            log_goal_to_go_yardline: 0.0457,
            yards_to_go_yardline: 0.0036,
            log_yards_to_go_yardline: -0.0061,
            yardline_4th: 0.0011,
            log_yardline_4th: 0.0033,
            yardline_not_4th: 0.0020,
            log_yardline_not_4th: 0.0362,
            inside_2m_warning: -0.0010,
            garbage_time_win: -0.0000,
            garbage_time_loss: -0.0022,
            qb_scramble_rate: 0.0003,
            log_qb_scramble: 0.0092,
            qb_prob_sack_given_hit: 0.0004,
            log_qbps: 0.0010,
            offense_pass_rush_z: 0.0014,
            defense_pass_rush_z: -0.0002,
            off_def_pass_rush_z: 0.0010,
            defense_completion_z: 0.0010,
            defense_interception_z: -0.0067,
            def_comp_scramble: 0.0000,
            def_int_scramble: 0.0000,
            olpz_qbps: 0.0010,
            dlpz_qbps: 0.0002,
            olpz_scramble: -0.0008,
            dlpz_scramble: 0.0000,
            qb_ay_oe: 0.0323,
            qb_ay_std: 0.0342,
            log_qb_mean_ay: 0.0040,
            log_qb_std_ay: 0.0034,
        }
    }


    pub fn pos_air_yards_var_coef() -> AirYardsModel {
        AirYardsModel {
            intercept: 2.7927,
            clock_running: 0.0189,
            inv_half_minutes: 0.0588,
            log_inv_half_minutes: -0.0030,
            inv_game_minutes: 0.0308,
            log_inv_game_minutes: 0.0147,
            possession_diff: 0.0287,
            fg_possession_diff: -0.0172,
            possdiff_per_minute: 0.0050,
            fgpossdiff_per_minute: -0.0045,
            clock_runs_pdpm: 0.0277,
            clock_runs_fgpdpm: 0.0091,
            clock_runs_pdpm2: 0.0028,
            clock_runs_fgpdpm2: -0.0050,
            is_offense_home: -0.0015,
            offense_log_pass_prob: 0.0071,
            defense_log_pass_prob: 0.0001,
            off_def_lpp: -0.0039,
            off_lpp_rz: 0.0174,
            def_lpp_rz: 0.0021,
            off_def_lpp_rz: -0.0113,
            off_lpp_outside_rz: -0.0853,
            off_lpp_inside_rz: 0.0962,
            def_lpp_outside_rz: -0.0938,
            def_lpp_inside_rz: 0.0971,
            off_lpp_pdpm: -0.0012,
            def_lpp_pdpm: -0.0054,
            off_lpp_rz_pdpm: -0.0003,
            def_lpp_rz_pdpm: 0.0008,
            down_1: 0.0608,
            down_2: -0.0723,
            down_3: 0.0147,
            down_4: -0.0032,
            goal_to_go: -0.0480,
            z_ydstogo: 0.0535,
            ydstogo_pct: 0.0268,
            log_ydstogo_pct: 0.0103,
            to_go_1st: 0.0328,
            to_go_2nd: -0.0074,
            to_go_3rd: -0.0251,
            to_go_4th: -0.0124,
            log_to_go_1st: -0.0329,
            log_to_go_2nd: -0.0694,
            log_to_go_3rd: 0.0754,
            log_to_go_4th: 0.0372,
            fp_1st: 0.0261,
            fp_2nd: -0.0321,
            fp_3rd: 0.0051,
            fp_4th: 0.0008,
            yardline_fgsig_4th: -0.0078,
            yardline_puntsig_4th: -0.0046,
            yardline_pct: 0.0002,
            yardline_pct_sq: -0.0420,
            log_yardline_pct: 0.1449,
            fg_sigmoid: 0.0196,
            punt_sigmoid: 0.0290,
            goal_to_go_yardline: -0.0041,
            log_goal_to_go_yardline: 0.1126,
            yards_to_go_yardline: 0.0044,
            log_yards_to_go_yardline: 0.0323,
            yardline_4th: 0.0016,
            log_yardline_4th: 0.0121,
            yardline_not_4th: -0.0013,
            log_yardline_not_4th: 0.1328,
            inside_2m_warning: -0.0417,
            garbage_time_win: 0.0016,
            garbage_time_loss: -0.0156,
            qb_scramble_rate: 0.0003,
            log_qb_scramble: 0.0055,
            qb_prob_sack_given_hit: -0.0004,
            log_qbps: 0.0000,
            offense_pass_rush_z: -0.0031,
            defense_pass_rush_z: -0.0075,
            off_def_pass_rush_z: 0.0043,
            defense_completion_z: 0.0016,
            defense_interception_z: -0.0165,
            def_comp_scramble: -0.0027,
            def_int_scramble: -0.0027,
            olpz_qbps: -0.0047,
            dlpz_qbps: 0.0053,
            olpz_scramble: -0.0028,
            dlpz_scramble: -0.0027,
            qb_ay_oe: 0.0205,
            qb_ay_std: 0.2133,
            log_qb_mean_ay: 0.0029,
            log_qb_std_ay: 0.0219,
        }
    }

}