use crate::models::air_yards::AirYardsModel;

impl AirYardsModel {

    pub fn is_pos_air_yards_coef() -> AirYardsModel {
        AirYardsModel {
            intercept: 4.0922,
            clock_running: 0.1051,
            inv_half_minutes: 0.1229,
            log_inv_half_minutes: 0.0482,
            inv_game_minutes: -0.0931,
            log_inv_game_minutes: 0.0824,
            possession_diff: -0.0376,
            fg_possession_diff: -0.0068,
            possdiff_per_minute: -0.1644,
            fgpossdiff_per_minute: -0.0177,
            clock_runs_pdpm: -0.0148,
            clock_runs_fgpdpm: -0.1830,
            clock_runs_pdpm2: 0.0007,
            clock_runs_fgpdpm2: 0.0113,
            is_offense_home: 0.0674,
            offense_log_pass_prob: -0.2452,
            defense_log_pass_prob: 0.1641,
            off_def_lpp: 0.0847,
            off_lpp_rz: 0.1755,
            def_lpp_rz: 0.0283,
            off_def_lpp_rz: 0.0189,
            off_lpp_outside_rz: 0.3068,
            off_lpp_inside_rz: 0.0139,
            def_lpp_outside_rz: -0.2265,
            def_lpp_inside_rz: 0.1663,
            off_lpp_pdpm: -0.2522,
            def_lpp_pdpm: -0.0863,
            off_lpp_rz_pdpm: -0.1431,
            def_lpp_rz_pdpm: 0.2143,
            down_1: 0.0190,
            down_2: 0.2984,
            down_3: -0.9801,
            down_4: 0.6484,
            goal_to_go: 0.0573,
            z_ydstogo: -0.2463,
            ydstogo_pct: -0.1375,
            log_ydstogo_pct: 0.4160,
            to_go_1st: -0.2380,
            to_go_2nd: -0.7643,
            to_go_3rd: 1.6377,
            to_go_4th: 0.1760,
            log_to_go_1st: 0.0266,
            log_to_go_2nd: -0.1107,
            log_to_go_3rd: 0.1225,
            log_to_go_4th: 0.3777,
            fp_1st: -0.9281,
            fp_2nd: -0.8738,
            fp_3rd: 1.5441,
            fp_4th: 0.2601,
            yardline_fgsig_4th: -0.0024,
            yardline_puntsig_4th: -0.0384,
            yardline_pct: 0.1424,
            yardline_pct_sq: 0.0229,
            log_yardline_pct: -0.0149,
            fg_sigmoid: 0.0047,
            punt_sigmoid: 0.0132,
            goal_to_go_yardline: -0.0883,
            log_goal_to_go_yardline: -0.0242,
            yards_to_go_yardline: 0.2307,
            log_yards_to_go_yardline: 0.0093,
            yardline_4th: 0.3488,
            log_yardline_4th: 0.0650,
            yardline_not_4th: -0.2065,
            log_yardline_not_4th: -0.0799,
            inside_2m_warning: 0.1131,
            garbage_time_win: -0.3615,
            garbage_time_loss: -0.1390,
            qb_scramble_rate: 0.3709,
            log_qb_scramble: 0.0721,
            qb_prob_sack_given_hit: -0.2941,
            log_qbps: -0.0452,
            offense_pass_rush_z: -0.0173,
            defense_pass_rush_z: -0.0065,
            off_def_pass_rush_z: 0.0092,
            defense_completion_z: 0.0116,
            defense_interception_z: -0.0052,
            def_comp_scramble: 0.0024,
            def_int_scramble: 0.0024,
            olpz_qbps: 0.1081,
            dlpz_qbps: 0.0059,
            olpz_scramble: -0.0258,
            dlpz_scramble: 0.0024,
            qb_ay_oe: 0.2958,
            qb_ay_std: -0.2399,
            log_qb_mean_ay: 0.1805,
            log_qb_std_ay: 0.0743,
        }
    }


    pub fn neg_air_yards_coef() -> AirYardsModel {
        AirYardsModel {
            intercept: 0.8722,
            clock_running: -0.0023,
            inv_half_minutes: -0.0023,
            log_inv_half_minutes: -0.0034,
            inv_game_minutes: -0.0018,
            log_inv_game_minutes: -0.0059,
            possession_diff: 0.0021,
            fg_possession_diff: 0.0019,
            possdiff_per_minute: 0.0021,
            fgpossdiff_per_minute: 0.0006,
            clock_runs_pdpm: 0.0010,
            clock_runs_fgpdpm: 0.0005,
            clock_runs_pdpm2: 0.0007,
            clock_runs_fgpdpm2: -0.0011,
            is_offense_home: -0.0002,
            offense_log_pass_prob: -0.0003,
            defense_log_pass_prob: 0.0001,
            off_def_lpp: 0.0001,
            off_lpp_rz: -0.0012,
            def_lpp_rz: -0.0004,
            off_def_lpp_rz: 0.0009,
            off_lpp_outside_rz: -0.0010,
            off_lpp_inside_rz: 0.0006,
            def_lpp_outside_rz: -0.0007,
            def_lpp_inside_rz: 0.0007,
            off_lpp_pdpm: -0.0011,
            def_lpp_pdpm: -0.0011,
            off_lpp_rz_pdpm: -0.0010,
            def_lpp_rz_pdpm: -0.0011,
            down_1: -0.0013,
            down_2: 0.0043,
            down_3: -0.0026,
            down_4: -0.0004,
            goal_to_go: -0.0013,
            z_ydstogo: 0.0104,
            ydstogo_pct: 0.0052,
            log_ydstogo_pct: 0.0089,
            to_go_1st: -0.0008,
            to_go_2nd: 0.0011,
            to_go_3rd: -0.0024,
            to_go_4th: -0.0003,
            log_to_go_1st: 0.0010,
            log_to_go_2nd: 0.0019,
            log_to_go_3rd: 0.0052,
            log_to_go_4th: 0.0008,
            fp_1st: -0.0004,
            fp_2nd: 0.0017,
            fp_3rd: -0.0004,
            fp_4th: -0.0000,
            yardline_fgsig_4th: 0.0013,
            yardline_puntsig_4th: 0.0039,
            yardline_pct: 0.0004,
            yardline_pct_sq: 0.0000,
            log_yardline_pct: 0.0042,
            fg_sigmoid: -0.0010,
            punt_sigmoid: 0.0010,
            goal_to_go_yardline: -0.0000,
            log_goal_to_go_yardline: 0.0052,
            yards_to_go_yardline: 0.0004,
            log_yards_to_go_yardline: -0.0010,
            yardline_4th: -0.0001,
            log_yardline_4th: 0.0008,
            yardline_not_4th: 0.0005,
            log_yardline_not_4th: 0.0034,
            inside_2m_warning: -0.0009,
            garbage_time_win: 0.0000,
            garbage_time_loss: -0.0000,
            qb_scramble_rate: 0.0000,
            log_qb_scramble: 0.0001,
            qb_prob_sack_given_hit: -0.0002,
            log_qbps: -0.0006,
            offense_pass_rush_z: -0.0013,
            defense_pass_rush_z: -0.0001,
            off_def_pass_rush_z: -0.0004,
            defense_completion_z: -0.0012,
            defense_interception_z: 0.0006,
            def_comp_scramble: -0.0001,
            def_int_scramble: -0.0001,
            olpz_qbps: 0.0017,
            dlpz_qbps: 0.0001,
            olpz_scramble: 0.0039,
            dlpz_scramble: -0.0001,
            qb_ay_oe: -0.0057,
            qb_ay_std: 0.0013,
            log_qb_mean_ay: -0.0007,
            log_qb_std_ay: 0.0001,
        }
    }


    pub fn pos_air_yards_coef() -> AirYardsModel {
        AirYardsModel {
            intercept: 2.1556,
            clock_running: -0.0024,
            inv_half_minutes: 0.0098,
            log_inv_half_minutes: 0.0045,
            inv_game_minutes: 0.0070,
            log_inv_game_minutes: 0.0097,
            possession_diff: 0.0053,
            fg_possession_diff: 0.0009,
            possdiff_per_minute: -0.0009,
            fgpossdiff_per_minute: 0.0016,
            clock_runs_pdpm: 0.0023,
            clock_runs_fgpdpm: 0.0010,
            clock_runs_pdpm2: 0.0019,
            clock_runs_fgpdpm2: -0.0025,
            is_offense_home: -0.0021,
            offense_log_pass_prob: 0.0004,
            defense_log_pass_prob: -0.0003,
            off_def_lpp: -0.0001,
            off_lpp_rz: 0.0009,
            def_lpp_rz: 0.0003,
            off_def_lpp_rz: -0.0006,
            off_lpp_outside_rz: -0.0101,
            off_lpp_inside_rz: 0.0111,
            def_lpp_outside_rz: -0.0109,
            def_lpp_inside_rz: 0.0111,
            off_lpp_pdpm: 0.0003,
            def_lpp_pdpm: 0.0003,
            off_lpp_rz_pdpm: 0.0004,
            def_lpp_rz_pdpm: 0.0004,
            down_1: 0.0063,
            down_2: -0.0152,
            down_3: 0.0077,
            down_4: 0.0011,
            goal_to_go: -0.0129,
            z_ydstogo: 0.0234,
            ydstogo_pct: 0.0117,
            log_ydstogo_pct: 0.0250,
            to_go_1st: 0.0013,
            to_go_2nd: -0.0058,
            to_go_3rd: -0.0016,
            to_go_4th: -0.0010,
            log_to_go_1st: 0.0040,
            log_to_go_2nd: 0.0005,
            log_to_go_3rd: 0.0152,
            log_to_go_4th: 0.0054,
            fp_1st: 0.0032,
            fp_2nd: -0.0051,
            fp_3rd: 0.0046,
            fp_4th: 0.0008,
            yardline_fgsig_4th: -0.0102,
            yardline_puntsig_4th: -0.0019,
            yardline_pct: 0.0031,
            yardline_pct_sq: -0.0015,
            log_yardline_pct: 0.0397,
            fg_sigmoid: 0.0071,
            punt_sigmoid: 0.0115,
            goal_to_go_yardline: -0.0005,
            log_goal_to_go_yardline: 0.0448,
            yards_to_go_yardline: 0.0036,
            log_yards_to_go_yardline: -0.0051,
            yardline_4th: 0.0012,
            log_yardline_4th: 0.0033,
            yardline_not_4th: 0.0019,
            log_yardline_not_4th: 0.0364,
            inside_2m_warning: -0.0010,
            garbage_time_win: -0.0001,
            garbage_time_loss: -0.0022,
            qb_scramble_rate: 0.0003,
            log_qb_scramble: 0.0094,
            qb_prob_sack_given_hit: 0.0003,
            log_qbps: 0.0009,
            offense_pass_rush_z: 0.0012,
            defense_pass_rush_z: -0.0004,
            off_def_pass_rush_z: 0.0009,
            defense_completion_z: 0.0010,
            defense_interception_z: -0.0070,
            def_comp_scramble: 0.0000,
            def_int_scramble: 0.0000,
            olpz_qbps: 0.0012,
            dlpz_qbps: 0.0003,
            olpz_scramble: -0.0008,
            dlpz_scramble: 0.0000,
            qb_ay_oe: 0.0321,
            qb_ay_std: 0.0339,
            log_qb_mean_ay: 0.0040,
            log_qb_std_ay: 0.0034,
        }
    }


    pub fn pos_air_yards_var_coef() -> AirYardsModel {
        AirYardsModel {
            intercept: 2.7940,
            clock_running: 0.0208,
            inv_half_minutes: 0.0595,
            log_inv_half_minutes: -0.0037,
            inv_game_minutes: 0.0291,
            log_inv_game_minutes: 0.0155,
            possession_diff: 0.0259,
            fg_possession_diff: -0.0151,
            possdiff_per_minute: 0.0054,
            fgpossdiff_per_minute: -0.0068,
            clock_runs_pdpm: 0.0280,
            clock_runs_fgpdpm: 0.0096,
            clock_runs_pdpm2: 0.0029,
            clock_runs_fgpdpm2: -0.0052,
            is_offense_home: -0.0004,
            offense_log_pass_prob: 0.0074,
            defense_log_pass_prob: -0.0000,
            off_def_lpp: -0.0039,
            off_lpp_rz: 0.0172,
            def_lpp_rz: 0.0034,
            off_def_lpp_rz: -0.0119,
            off_lpp_outside_rz: -0.0849,
            off_lpp_inside_rz: 0.0962,
            def_lpp_outside_rz: -0.0937,
            def_lpp_inside_rz: 0.0970,
            off_lpp_pdpm: -0.0017,
            def_lpp_pdpm: -0.0056,
            off_lpp_rz_pdpm: -0.0018,
            def_lpp_rz_pdpm: -0.0004,
            down_1: 0.0606,
            down_2: -0.0725,
            down_3: 0.0153,
            down_4: -0.0034,
            goal_to_go: -0.0472,
            z_ydstogo: 0.0550,
            ydstogo_pct: 0.0275,
            log_ydstogo_pct: 0.0071,
            to_go_1st: 0.0325,
            to_go_2nd: -0.0061,
            to_go_3rd: -0.0256,
            to_go_4th: -0.0127,
            log_to_go_1st: -0.0324,
            log_to_go_2nd: -0.0761,
            log_to_go_3rd: 0.0780,
            log_to_go_4th: 0.0376,
            fp_1st: 0.0260,
            fp_2nd: -0.0329,
            fp_3rd: 0.0055,
            fp_4th: 0.0007,
            yardline_fgsig_4th: -0.0087,
            yardline_puntsig_4th: -0.0047,
            yardline_pct: 0.0000,
            yardline_pct_sq: -0.0419,
            log_yardline_pct: 0.1441,
            fg_sigmoid: 0.0190,
            punt_sigmoid: 0.0289,
            goal_to_go_yardline: -0.0041,
            log_goal_to_go_yardline: 0.1108,
            yards_to_go_yardline: 0.0041,
            log_yards_to_go_yardline: 0.0333,
            yardline_4th: 0.0017,
            log_yardline_4th: 0.0126,
            yardline_not_4th: -0.0016,
            log_yardline_not_4th: 0.1315,
            inside_2m_warning: -0.0411,
            garbage_time_win: 0.0015,
            garbage_time_loss: -0.0147,
            qb_scramble_rate: 0.0003,
            log_qb_scramble: 0.0061,
            qb_prob_sack_given_hit: -0.0007,
            log_qbps: -0.0006,
            offense_pass_rush_z: -0.0029,
            defense_pass_rush_z: -0.0081,
            off_def_pass_rush_z: 0.0047,
            defense_completion_z: 0.0040,
            defense_interception_z: -0.0183,
            def_comp_scramble: -0.0026,
            def_int_scramble: -0.0026,
            olpz_qbps: -0.0048,
            dlpz_qbps: 0.0070,
            olpz_scramble: -0.0029,
            dlpz_scramble: -0.0026,
            qb_ay_oe: 0.0200,
            qb_ay_std: 0.2124,
            log_qb_mean_ay: 0.0029,
            log_qb_std_ay: 0.0218,
        }
    }

}