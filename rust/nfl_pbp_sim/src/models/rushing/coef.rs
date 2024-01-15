use crate::models::rushing::RushingModel;

impl RushingModel {

    pub fn scrambling_fumble_lost_coef() -> RushingModel {
        RushingModel {
            intercept: -3.8690,
            is_offense_home: 0.1195,
            offense_log_pass_prob: 0.0151,
            defense_log_pass_prob: -0.0209,
            off_def_lpp: 0.0047,
            off_lpp_rz: -0.0140,
            def_lpp_rz: -0.0001,
            off_def_lpp_rz: 0.0103,
            off_lpp_outside_rz: 0.0199,
            off_lpp_inside_rz: 0.0028,
            def_lpp_outside_rz: -0.0340,
            def_lpp_inside_rz: 0.0222,
            off_lpp_pdpm: 0.0080,
            def_lpp_pdpm: 0.0205,
            off_lpp_rz_pdpm: 0.0216,
            def_lpp_rz_pdpm: 0.0211,
            down_1: -0.1126,
            down_2: -0.2055,
            down_3: 0.1988,
            down_4: 0.1192,
            goal_to_go: -0.0067,
            z_ydstogo: -0.0132,
            ydstogo_pct: -0.0067,
            log_ydstogo_pct: 0.0967,
            to_go_1st: -0.0494,
            to_go_2nd: -0.1251,
            to_go_3rd: 0.0834,
            to_go_4th: 0.0871,
            log_to_go_1st: 0.0257,
            log_to_go_2nd: 0.1457,
            log_to_go_3rd: 0.0420,
            log_to_go_4th: -0.1166,
            fp_1st: -0.0359,
            fp_2nd: -0.0644,
            fp_3rd: 0.0805,
            fp_4th: 0.0299,
            yardline_fgsig_4th: 0.1129,
            yardline_puntsig_4th: 0.0382,
            yardline_pct: 0.0371,
            yardline_pct_sq: 0.0662,
            log_yardline_pct: -0.0300,
            fg_sigmoid: 0.0091,
            punt_sigmoid: 0.0176,
            goal_to_go_yardline: -0.0063,
            log_goal_to_go_yardline: -0.0921,
            yards_to_go_yardline: 0.0434,
            log_yards_to_go_yardline: 0.0621,
            yardline_4th: 0.0366,
            log_yardline_4th: -0.1985,
            yardline_not_4th: 0.0005,
            log_yardline_not_4th: 0.1685,
            inside_2m_warning: 0.1661,
            garbage_time_win: -0.0077,
            garbage_time_loss: 0.1025,
            clock_running: -0.1391,
            possdiff_per_minute: -0.0829,
            fgpossdiff_per_minute: -0.0033,
            ol_z: -0.0490,
            dl_z: 0.0957,
            ol_dl_z: -0.1276,
            log_mean_yards: 0.0001,
            log_std_yards: -0.0408,
            yoe_mean: 0.0156,
            yoe_std: -0.1983,
            togo_std: -0.0986,
            yardline_std: -0.0633,
            clock_runs_after: 0.0,
        }
    }


    pub fn scrambling_prob_fl_td_coef() -> RushingModel {
        RushingModel {
            intercept: -3.8854,
            is_offense_home: -0.0730,
            offense_log_pass_prob: -0.0085,
            defense_log_pass_prob: -0.0004,
            off_def_lpp: 0.0044,
            off_lpp_rz: -0.0104,
            def_lpp_rz: 0.0145,
            off_def_lpp_rz: -0.0029,
            off_lpp_outside_rz: -0.0088,
            off_lpp_inside_rz: 0.0003,
            def_lpp_outside_rz: -0.0007,
            def_lpp_inside_rz: 0.0003,
            off_lpp_pdpm: -0.0095,
            def_lpp_pdpm: -0.0113,
            off_lpp_rz_pdpm: -0.0137,
            def_lpp_rz_pdpm: -0.0163,
            down_1: -0.0361,
            down_2: 0.0247,
            down_3: 0.0116,
            down_4: -0.0003,
            goal_to_go: -0.0000,
            z_ydstogo: 0.0825,
            ydstogo_pct: 0.0412,
            log_ydstogo_pct: 0.0407,
            to_go_1st: -0.0133,
            to_go_2nd: 0.0018,
            to_go_3rd: -0.0029,
            to_go_4th: -0.0002,
            log_to_go_1st: 0.0000,
            log_to_go_2nd: 0.0203,
            log_to_go_3rd: 0.0201,
            log_to_go_4th: 0.0003,
            fp_1st: -0.0133,
            fp_2nd: 0.0078,
            fp_3rd: 0.0052,
            fp_4th: -0.0001,
            yardline_fgsig_4th: 0.0002,
            yardline_puntsig_4th: 0.0007,
            yardline_pct: 0.0169,
            yardline_pct_sq: 0.0258,
            log_yardline_pct: 0.0247,
            fg_sigmoid: -0.1508,
            punt_sigmoid: 0.1040,
            goal_to_go_yardline: -0.0000,
            log_goal_to_go_yardline: 0.0002,
            yards_to_go_yardline: 0.0169,
            log_yards_to_go_yardline: 0.0245,
            yardline_4th: -0.0001,
            log_yardline_4th: 0.0003,
            yardline_not_4th: 0.0170,
            log_yardline_not_4th: 0.0244,
            inside_2m_warning: -0.0196,
            garbage_time_win: 0.0000,
            garbage_time_loss: -0.0018,
            clock_running: -0.0053,
            possdiff_per_minute: 0.0244,
            fgpossdiff_per_minute: 0.0215,
            ol_z: 0.0441,
            dl_z: -0.1768,
            ol_dl_z: -0.0544,
            log_mean_yards: -0.0067,
            log_std_yards: -0.0112,
            yoe_mean: -0.0487,
            yoe_std: -0.0566,
            togo_std: 0.3750,
            yardline_std: 0.0413,
            clock_runs_after: 0.0,
        }
    }


    pub fn scrambling_rush_td_coef() -> RushingModel {
        RushingModel {
            intercept: -6.7932,
            is_offense_home: 0.2681,
            offense_log_pass_prob: -0.0272,
            defense_log_pass_prob: 0.0674,
            off_def_lpp: -0.0220,
            off_lpp_rz: 0.0760,
            def_lpp_rz: -0.0245,
            off_def_lpp_rz: -0.0387,
            off_lpp_outside_rz: 0.1206,
            off_lpp_inside_rz: -0.0463,
            def_lpp_outside_rz: 0.1946,
            def_lpp_inside_rz: -0.2724,
            off_lpp_pdpm: -0.0818,
            def_lpp_pdpm: -0.0278,
            off_lpp_rz_pdpm: -0.1513,
            def_lpp_rz_pdpm: -0.0346,
            down_1: -0.1386,
            down_2: -0.0274,
            down_3: 0.1679,
            down_4: -0.0021,
            goal_to_go: 0.0815,
            z_ydstogo: -0.1419,
            ydstogo_pct: -0.0711,
            log_ydstogo_pct: 0.0466,
            to_go_1st: -0.0392,
            to_go_2nd: -0.0316,
            to_go_3rd: 0.0507,
            to_go_4th: 0.0032,
            log_to_go_1st: -0.0678,
            log_to_go_2nd: 0.0824,
            log_to_go_3rd: 0.0229,
            log_to_go_4th: 0.0091,
            fp_1st: -0.0633,
            fp_2nd: 0.0150,
            fp_3rd: 0.0940,
            fp_4th: 0.0006,
            yardline_fgsig_4th: 0.0587,
            yardline_puntsig_4th: -0.0996,
            yardline_pct: -0.1390,
            yardline_pct_sq: -0.0598,
            log_yardline_pct: -0.8198,
            fg_sigmoid: 0.1226,
            punt_sigmoid: 0.0009,
            goal_to_go_yardline: -0.0094,
            log_goal_to_go_yardline: -0.4546,
            yards_to_go_yardline: -0.1296,
            log_yards_to_go_yardline: -0.3653,
            yardline_4th: -0.0269,
            log_yardline_4th: -0.2339,
            yardline_not_4th: -0.1121,
            log_yardline_not_4th: -0.5859,
            inside_2m_warning: 0.0945,
            garbage_time_win: -0.0200,
            garbage_time_loss: 0.0340,
            clock_running: 0.1202,
            possdiff_per_minute: 0.0881,
            fgpossdiff_per_minute: -0.1948,
            ol_z: -0.0173,
            dl_z: -0.2850,
            ol_dl_z: 0.0197,
            log_mean_yards: 0.0695,
            log_std_yards: 0.0418,
            yoe_mean: 0.4999,
            yoe_std: 0.1079,
            togo_std: -0.0021,
            yardline_std: -0.2059,
            clock_runs_after: 0.0,
        }
    }


    pub fn scrambling_clock_runs_coef() -> RushingModel {
        RushingModel {
            intercept: 0.7120,
            is_offense_home: -0.0438,
            offense_log_pass_prob: -0.0667,
            defense_log_pass_prob: 0.0324,
            off_def_lpp: 0.0254,
            off_lpp_rz: -0.1945,
            def_lpp_rz: 0.0341,
            off_def_lpp_rz: 0.1086,
            off_lpp_outside_rz: -0.1935,
            off_lpp_inside_rz: 0.1480,
            def_lpp_outside_rz: 0.0805,
            def_lpp_inside_rz: 0.0183,
            off_lpp_pdpm: 0.1704,
            def_lpp_pdpm: 0.0255,
            off_lpp_rz_pdpm: -0.2476,
            def_lpp_rz_pdpm: -0.2430,
            down_1: -0.1042,
            down_2: -0.0225,
            down_3: 0.0982,
            down_4: 0.0288,
            goal_to_go: -0.0166,
            z_ydstogo: 0.0674,
            ydstogo_pct: 0.0341,
            log_ydstogo_pct: 0.1984,
            to_go_1st: -0.0292,
            to_go_2nd: -0.0189,
            to_go_3rd: 0.1338,
            to_go_4th: -0.0682,
            log_to_go_1st: -0.0368,
            log_to_go_2nd: -0.0101,
            log_to_go_3rd: -0.0845,
            log_to_go_4th: 0.3298,
            fp_1st: -0.0525,
            fp_2nd: -0.0028,
            fp_3rd: 0.0113,
            fp_4th: 0.0229,
            yardline_fgsig_4th: -0.0928,
            yardline_puntsig_4th: -0.0682,
            yardline_pct: -0.0021,
            yardline_pct_sq: -0.0667,
            log_yardline_pct: -0.2575,
            fg_sigmoid: -0.0217,
            punt_sigmoid: 0.0123,
            goal_to_go_yardline: 0.0018,
            log_goal_to_go_yardline: -0.0994,
            yards_to_go_yardline: -0.0039,
            log_yards_to_go_yardline: -0.1581,
            yardline_4th: -0.0470,
            log_yardline_4th: -0.2336,
            yardline_not_4th: 0.0449,
            log_yardline_not_4th: -0.0239,
            inside_2m_warning: -0.0957,
            garbage_time_win: 0.0545,
            garbage_time_loss: -0.0243,
            clock_running: 0.0906,
            possdiff_per_minute: -0.0563,
            fgpossdiff_per_minute: -0.0560,
            ol_z: -0.0786,
            dl_z: 0.0155,
            ol_dl_z: -0.0037,
            log_mean_yards: 0.0097,
            log_std_yards: -0.1256,
            yoe_mean: 0.0883,
            yoe_std: -0.1732,
            togo_std: -0.0285,
            yardline_std: 0.0694,
            clock_runs_after: 0.0,
        }
    }


    pub fn scrambling_pos_yards_coef() -> RushingModel {
        RushingModel {
            intercept: 1.7509,
            is_offense_home: 0.0052,
            offense_log_pass_prob: 0.0009,
            defense_log_pass_prob: -0.0000,
            off_def_lpp: -0.0005,
            off_lpp_rz: 0.0020,
            def_lpp_rz: 0.0002,
            off_def_lpp_rz: -0.0011,
            off_lpp_outside_rz: -0.0049,
            off_lpp_inside_rz: 0.0064,
            def_lpp_outside_rz: -0.0059,
            def_lpp_inside_rz: 0.0059,
            off_lpp_pdpm: 0.0047,
            def_lpp_pdpm: 0.0045,
            off_lpp_rz_pdpm: 0.0050,
            def_lpp_rz_pdpm: 0.0048,
            down_1: -0.0174,
            down_2: -0.0002,
            down_3: 0.0165,
            down_4: 0.0011,
            goal_to_go: -0.0048,
            z_ydstogo: 0.0050,
            ydstogo_pct: 0.0025,
            log_ydstogo_pct: 0.0092,
            to_go_1st: -0.0062,
            to_go_2nd: -0.0000,
            to_go_3rd: 0.0039,
            to_go_4th: 0.0001,
            log_to_go_1st: -0.0008,
            log_to_go_2nd: -0.0000,
            log_to_go_3rd: 0.0088,
            log_to_go_4th: 0.0012,
            fp_1st: -0.0061,
            fp_2nd: 0.0001,
            fp_3rd: 0.0069,
            fp_4th: 0.0006,
            yardline_fgsig_4th: -0.0118,
            yardline_puntsig_4th: 0.0001,
            yardline_pct: 0.0011,
            yardline_pct_sq: -0.0012,
            log_yardline_pct: 0.0141,
            fg_sigmoid: 0.0073,
            punt_sigmoid: 0.0147,
            goal_to_go_yardline: -0.0003,
            log_goal_to_go_yardline: 0.0135,
            yards_to_go_yardline: 0.0014,
            log_yards_to_go_yardline: 0.0006,
            yardline_4th: 0.0008,
            log_yardline_4th: -0.0000,
            yardline_not_4th: 0.0003,
            log_yardline_not_4th: 0.0141,
            inside_2m_warning: 0.0101,
            garbage_time_win: -0.0000,
            garbage_time_loss: 0.0012,
            clock_running: -0.0048,
            possdiff_per_minute: -0.0089,
            fgpossdiff_per_minute: -0.0008,
            ol_z: 0.0078,
            dl_z: -0.0088,
            ol_dl_z: 0.0020,
            log_mean_yards: 0.0055,
            log_std_yards: 0.0059,
            yoe_mean: 0.0404,
            yoe_std: 0.0317,
            togo_std: 0.0143,
            yardline_std: 0.0245,
            clock_runs_after: -0.0137,
        }
    }


    pub fn scrambling_pos_yards_var_coef() -> RushingModel {
        RushingModel {
            intercept: 2.1431,
            is_offense_home: 0.0089,
            offense_log_pass_prob: 0.0127,
            defense_log_pass_prob: 0.0025,
            off_def_lpp: -0.0082,
            off_lpp_rz: 0.0215,
            def_lpp_rz: 0.0071,
            off_def_lpp_rz: -0.0160,
            off_lpp_outside_rz: -0.0142,
            off_lpp_inside_rz: 0.0300,
            def_lpp_outside_rz: -0.0246,
            def_lpp_inside_rz: 0.0274,
            off_lpp_pdpm: 0.0092,
            def_lpp_pdpm: 0.0106,
            off_lpp_rz_pdpm: 0.0186,
            def_lpp_rz_pdpm: 0.0148,
            down_1: -0.0203,
            down_2: 0.0136,
            down_3: 0.0055,
            down_4: 0.0013,
            goal_to_go: -0.0145,
            z_ydstogo: 0.0147,
            ydstogo_pct: 0.0073,
            log_ydstogo_pct: 0.0216,
            to_go_1st: -0.0038,
            to_go_2nd: 0.0016,
            to_go_3rd: -0.0019,
            to_go_4th: -0.0018,
            log_to_go_1st: -0.0110,
            log_to_go_2nd: 0.0114,
            log_to_go_3rd: 0.0165,
            log_to_go_4th: 0.0046,
            fp_1st: -0.0057,
            fp_2nd: 0.0056,
            fp_3rd: 0.0038,
            fp_4th: 0.0005,
            yardline_fgsig_4th: -0.0037,
            yardline_puntsig_4th: 0.0023,
            yardline_pct: -0.0106,
            yardline_pct_sq: -0.0276,
            log_yardline_pct: 0.0304,
            fg_sigmoid: 0.0223,
            punt_sigmoid: 0.0352,
            goal_to_go_yardline: -0.0013,
            log_goal_to_go_yardline: 0.0355,
            yards_to_go_yardline: -0.0092,
            log_yards_to_go_yardline: -0.0051,
            yardline_4th: 0.0007,
            log_yardline_4th: 0.0007,
            yardline_not_4th: -0.0112,
            log_yardline_not_4th: 0.0297,
            inside_2m_warning: 0.0234,
            garbage_time_win: 0.0005,
            garbage_time_loss: 0.0075,
            clock_running: 0.0029,
            possdiff_per_minute: -0.0166,
            fgpossdiff_per_minute: 0.0319,
            ol_z: 0.0005,
            dl_z: -0.0463,
            ol_dl_z: -0.0014,
            log_mean_yards: 0.0175,
            log_std_yards: 0.0504,
            yoe_mean: 0.1250,
            yoe_std: 0.2308,
            togo_std: 0.0037,
            yardline_std: 0.1337,
            clock_runs_after: -0.0744,
        }
    }


    pub fn designed_run_fumble_lost_coef() -> RushingModel {
        RushingModel {
            intercept: -3.8928,
            is_offense_home: 0.0665,
            offense_log_pass_prob: 0.0463,
            defense_log_pass_prob: 0.0481,
            off_def_lpp: -0.0520,
            off_lpp_rz: -0.0328,
            def_lpp_rz: 0.1860,
            off_def_lpp_rz: -0.0838,
            off_lpp_outside_rz: -0.0126,
            off_lpp_inside_rz: 0.0662,
            def_lpp_outside_rz: -0.0650,
            def_lpp_inside_rz: 0.0801,
            off_lpp_pdpm: 0.0963,
            def_lpp_pdpm: -0.0301,
            off_lpp_rz_pdpm: 0.0006,
            def_lpp_rz_pdpm: 0.0889,
            down_1: -0.1511,
            down_2: -0.0772,
            down_3: -0.0383,
            down_4: 0.2654,
            goal_to_go: -0.1008,
            z_ydstogo: 0.0457,
            ydstogo_pct: 0.0216,
            log_ydstogo_pct: 0.0911,
            to_go_1st: -0.1222,
            to_go_2nd: 0.0726,
            to_go_3rd: 0.0195,
            to_go_4th: 0.0332,
            log_to_go_1st: -0.0534,
            log_to_go_2nd: 0.0206,
            log_to_go_3rd: -0.1176,
            log_to_go_4th: 0.2416,
            fp_1st: -0.1428,
            fp_2nd: 0.0260,
            fp_3rd: 0.0261,
            fp_4th: 0.0855,
            yardline_fgsig_4th: -0.1257,
            yardline_puntsig_4th: -0.0638,
            yardline_pct: 0.0853,
            yardline_pct_sq: 0.0863,
            log_yardline_pct: -0.0995,
            fg_sigmoid: 0.0030,
            punt_sigmoid: 0.0142,
            goal_to_go_yardline: -0.0131,
            log_goal_to_go_yardline: -0.0858,
            yards_to_go_yardline: 0.0984,
            log_yards_to_go_yardline: -0.0137,
            yardline_4th: 0.2065,
            log_yardline_4th: -0.0636,
            yardline_not_4th: -0.1212,
            log_yardline_not_4th: -0.0359,
            inside_2m_warning: 0.0694,
            garbage_time_win: -0.1342,
            garbage_time_loss: 0.0010,
            clock_running: -0.0201,
            possdiff_per_minute: -0.0674,
            fgpossdiff_per_minute: 0.1043,
            ol_z: 0.0899,
            dl_z: 0.0116,
            ol_dl_z: 0.0677,
            log_mean_yards: -0.1064,
            log_std_yards: -0.1556,
            yoe_mean: -0.1896,
            yoe_std: -0.1066,
            togo_std: -0.0115,
            yardline_std: -0.0335,
            clock_runs_after: 0.0,
        }
    }


    pub fn designed_run_prob_fl_td_coef() -> RushingModel {
        RushingModel {
            intercept: -3.5797,
            is_offense_home: 0.2101,
            offense_log_pass_prob: 0.0326,
            defense_log_pass_prob: -0.0187,
            off_def_lpp: -0.0058,
            off_lpp_rz: 0.0599,
            def_lpp_rz: 0.0551,
            off_def_lpp_rz: -0.0606,
            off_lpp_outside_rz: 0.0715,
            off_lpp_inside_rz: -0.0166,
            def_lpp_outside_rz: 0.0135,
            def_lpp_inside_rz: -0.0076,
            off_lpp_pdpm: -0.0003,
            def_lpp_pdpm: -0.0162,
            off_lpp_rz_pdpm: -0.0058,
            def_lpp_rz_pdpm: -0.0230,
            down_1: -0.0270,
            down_2: 0.0238,
            down_3: 0.0141,
            down_4: -0.0108,
            goal_to_go: -0.0291,
            z_ydstogo: 0.0996,
            ydstogo_pct: 0.0500,
            log_ydstogo_pct: 0.1066,
            to_go_1st: -0.0110,
            to_go_2nd: 0.0269,
            to_go_3rd: 0.0013,
            to_go_4th: -0.0233,
            log_to_go_1st: -0.0171,
            log_to_go_2nd: -0.0290,
            log_to_go_3rd: 0.0502,
            log_to_go_4th: 0.1025,
            fp_1st: -0.0368,
            fp_2nd: 0.0106,
            fp_3rd: 0.0151,
            fp_4th: 0.0157,
            yardline_fgsig_4th: -0.1302,
            yardline_puntsig_4th: -0.0992,
            yardline_pct: 0.0852,
            yardline_pct_sq: 0.1586,
            log_yardline_pct: -0.0210,
            fg_sigmoid: 0.0101,
            punt_sigmoid: 0.0128,
            goal_to_go_yardline: -0.0061,
            log_goal_to_go_yardline: -0.0584,
            yards_to_go_yardline: 0.0912,
            log_yards_to_go_yardline: 0.0374,
            yardline_4th: 0.0260,
            log_yardline_4th: 0.1008,
            yardline_not_4th: 0.0592,
            log_yardline_not_4th: -0.1218,
            inside_2m_warning: -0.1461,
            garbage_time_win: -0.0632,
            garbage_time_loss: -0.0374,
            clock_running: -0.1559,
            possdiff_per_minute: 0.0293,
            fgpossdiff_per_minute: 0.3213,
            ol_z: -0.0855,
            dl_z: -0.0452,
            ol_dl_z: 0.1331,
            log_mean_yards: 0.0090,
            log_std_yards: -0.0041,
            yoe_mean: 0.0361,
            yoe_std: 0.0114,
            togo_std: -0.0229,
            yardline_std: 0.2493,
            clock_runs_after: 0.0,
        }
    }


    pub fn designed_run_rush_td_coef() -> RushingModel {
        RushingModel {
            intercept: -6.8831,
            is_offense_home: 0.0891,
            offense_log_pass_prob: -0.1131,
            defense_log_pass_prob: 0.0538,
            off_def_lpp: 0.0387,
            off_lpp_rz: 0.2228,
            def_lpp_rz: -0.1463,
            off_def_lpp_rz: -0.0570,
            off_lpp_outside_rz: 0.3492,
            off_lpp_inside_rz: -0.4009,
            def_lpp_outside_rz: 0.2193,
            def_lpp_inside_rz: -0.3094,
            off_lpp_pdpm: -0.0442,
            def_lpp_pdpm: -0.0362,
            off_lpp_rz_pdpm: -0.1249,
            def_lpp_rz_pdpm: 0.0548,
            down_1: -0.1145,
            down_2: -0.1184,
            down_3: 0.2460,
            down_4: -0.0147,
            goal_to_go: 0.2684,
            z_ydstogo: -0.4791,
            ydstogo_pct: -0.2412,
            log_ydstogo_pct: 0.6026,
            to_go_1st: 0.0916,
            to_go_2nd: -0.0367,
            to_go_3rd: 0.1067,
            to_go_4th: -0.0482,
            log_to_go_1st: 0.1218,
            log_to_go_2nd: 0.0150,
            log_to_go_3rd: 0.2169,
            log_to_go_4th: 0.2488,
            fp_1st: -0.0778,
            fp_2nd: -0.0972,
            fp_3rd: 0.1437,
            fp_4th: 0.0262,
            yardline_fgsig_4th: -0.0697,
            yardline_puntsig_4th: -0.0520,
            yardline_pct: -0.7998,
            yardline_pct_sq: -0.6856,
            log_yardline_pct: -0.7525,
            fg_sigmoid: 0.0715,
            punt_sigmoid: -0.0469,
            goal_to_go_yardline: -0.0113,
            log_goal_to_go_yardline: -0.3279,
            yards_to_go_yardline: -0.7885,
            log_yards_to_go_yardline: -0.4246,
            yardline_4th: -0.0128,
            log_yardline_4th: -0.3383,
            yardline_not_4th: -0.7870,
            log_yardline_not_4th: -0.4142,
            inside_2m_warning: 0.0842,
            garbage_time_win: -0.0379,
            garbage_time_loss: -0.0157,
            clock_running: -0.0229,
            possdiff_per_minute: 0.0956,
            fgpossdiff_per_minute: -0.2432,
            ol_z: 0.1551,
            dl_z: -0.1399,
            ol_dl_z: 0.0017,
            log_mean_yards: 0.0918,
            log_std_yards: 0.3537,
            yoe_mean: 0.4795,
            yoe_std: -0.1809,
            togo_std: -0.0130,
            yardline_std: 0.9468,
            clock_runs_after: 0.0,
        }
    }


    pub fn designed_run_safety_coef() -> RushingModel {
        RushingModel {
            intercept: -9.9277,
            is_offense_home: -0.2187,
            offense_log_pass_prob: 0.0356,
            defense_log_pass_prob: -0.0239,
            off_def_lpp: -0.0035,
            off_lpp_rz: 0.0372,
            def_lpp_rz: -0.0721,
            off_def_lpp_rz: 0.0207,
            off_lpp_outside_rz: 0.0356,
            off_lpp_inside_rz: 0.0000,
            def_lpp_outside_rz: -0.0239,
            def_lpp_inside_rz: 0.0000,
            off_lpp_pdpm: 0.0112,
            def_lpp_pdpm: -0.0128,
            off_lpp_rz_pdpm: 0.0013,
            def_lpp_rz_pdpm: -0.0594,
            down_1: -0.1375,
            down_2: 0.0170,
            down_3: 0.0956,
            down_4: 0.0249,
            goal_to_go: -0.0000,
            z_ydstogo: 0.0818,
            ydstogo_pct: 0.0409,
            log_ydstogo_pct: 0.3682,
            to_go_1st: -0.0401,
            to_go_2nd: -0.0348,
            to_go_3rd: 0.0642,
            to_go_4th: -0.0722,
            log_to_go_1st: -0.0315,
            log_to_go_2nd: 0.1260,
            log_to_go_3rd: -0.0601,
            log_to_go_4th: 0.3337,
            fp_1st: -0.0450,
            fp_2nd: 0.0257,
            fp_3rd: 0.0483,
            fp_4th: 0.0419,
            yardline_fgsig_4th: -0.3454,
            yardline_puntsig_4th: 0.0230,
            yardline_pct: 0.3545,
            yardline_pct_sq: 0.6043,
            log_yardline_pct: 0.4215,
            fg_sigmoid: -0.1038,
            punt_sigmoid: 0.0843,
            goal_to_go_yardline: -0.0000,
            log_goal_to_go_yardline: 0.0000,
            yards_to_go_yardline: 0.3545,
            log_yards_to_go_yardline: 0.4215,
            yardline_4th: 0.0544,
            log_yardline_4th: 0.0402,
            yardline_not_4th: 0.3001,
            log_yardline_not_4th: 0.3813,
            inside_2m_warning: 0.0155,
            garbage_time_win: -0.1031,
            garbage_time_loss: -0.0180,
            clock_running: -0.2443,
            possdiff_per_minute: -0.0088,
            fgpossdiff_per_minute: 0.2934,
            ol_z: -0.3843,
            dl_z: -0.0381,
            ol_dl_z: -0.0876,
            log_mean_yards: -0.0687,
            log_std_yards: -0.1413,
            yoe_mean: -0.2762,
            yoe_std: -0.7905,
            togo_std: -0.0179,
            yardline_std: 1.1845,
            clock_runs_after: 0.0,
        }
    }


    pub fn designed_run_clock_runs_coef() -> RushingModel {
        RushingModel {
            intercept: 1.5025,
            is_offense_home: 0.0055,
            offense_log_pass_prob: 0.0730,
            defense_log_pass_prob: -0.0084,
            off_def_lpp: -0.0790,
            off_lpp_rz: 0.1490,
            def_lpp_rz: -0.1548,
            off_def_lpp_rz: 0.0114,
            off_lpp_outside_rz: -0.0591,
            off_lpp_inside_rz: -0.1193,
            def_lpp_outside_rz: -0.1325,
            def_lpp_inside_rz: -0.0379,
            off_lpp_pdpm: 0.3262,
            def_lpp_pdpm: 0.1388,
            off_lpp_rz_pdpm: 0.0994,
            def_lpp_rz_pdpm: -0.0875,
            down_1: 0.0613,
            down_2: 0.0646,
            down_3: 0.1223,
            down_4: -0.2708,
            goal_to_go: 0.1325,
            z_ydstogo: 0.0648,
            ydstogo_pct: 0.0098,
            log_ydstogo_pct: -0.4061,
            to_go_1st: 0.0872,
            to_go_2nd: -0.0451,
            to_go_3rd: -0.1792,
            to_go_4th: -0.1048,
            log_to_go_1st: -0.0842,
            log_to_go_2nd: -0.0364,
            log_to_go_3rd: 0.0491,
            log_to_go_4th: -0.3346,
            fp_1st: 0.0145,
            fp_2nd: 0.1458,
            fp_3rd: 0.0886,
            fp_4th: -0.1483,
            yardline_fgsig_4th: 0.0359,
            yardline_puntsig_4th: 0.0145,
            yardline_pct: 0.2305,
            yardline_pct_sq: 0.4966,
            log_yardline_pct: -0.1218,
            fg_sigmoid: 0.0037,
            punt_sigmoid: -0.0024,
            goal_to_go_yardline: -0.0101,
            log_goal_to_go_yardline: -0.0464,
            yards_to_go_yardline: 0.2406,
            log_yards_to_go_yardline: -0.0754,
            yardline_4th: -0.1477,
            log_yardline_4th: 0.0528,
            yardline_not_4th: 0.3782,
            log_yardline_not_4th: -0.1746,
            inside_2m_warning: 0.1214,
            garbage_time_win: 0.2865,
            garbage_time_loss: 0.7477,
            clock_running: 0.0166,
            possdiff_per_minute: -0.2359,
            fgpossdiff_per_minute: 0.6519,
            ol_z: -0.1431,
            dl_z: 0.0622,
            ol_dl_z: -0.0024,
            log_mean_yards: 0.0109,
            log_std_yards: -0.5308,
            yoe_mean: -0.7297,
            yoe_std: 0.1440,
            togo_std: 0.0208,
            yardline_std: -0.0293,
            clock_runs_after: 0.0,
        }
    }


    pub fn designed_run_is_yards_pos_sign_coef() -> RushingModel {
        RushingModel {
            intercept: 4.5794,
            is_offense_home: -0.0183,
            offense_log_pass_prob: 0.1190,
            defense_log_pass_prob: -0.4182,
            off_def_lpp: 0.1725,
            off_lpp_rz: -0.2463,
            def_lpp_rz: 0.3287,
            off_def_lpp_rz: -0.0764,
            off_lpp_outside_rz: 0.4682,
            off_lpp_inside_rz: -0.1583,
            def_lpp_outside_rz: -0.2351,
            def_lpp_inside_rz: -0.1668,
            off_lpp_pdpm: -0.3604,
            def_lpp_pdpm: 0.1057,
            off_lpp_rz_pdpm: -0.1471,
            def_lpp_rz_pdpm: -0.1866,
            down_1: -0.1182,
            down_2: 0.2080,
            down_3: 0.2716,
            down_4: -0.3798,
            goal_to_go: 1.6671,
            z_ydstogo: -0.1090,
            ydstogo_pct: -0.0729,
            log_ydstogo_pct: 0.2370,
            to_go_1st: 0.7266,
            to_go_2nd: 0.1734,
            to_go_3rd: 0.1909,
            to_go_4th: -0.0162,
            log_to_go_1st: 0.2346,
            log_to_go_2nd: -0.0315,
            log_to_go_3rd: 0.1676,
            log_to_go_4th: -0.1337,
            fp_1st: -0.0931,
            fp_2nd: -0.4563,
            fp_3rd: -0.2508,
            fp_4th: -0.1083,
            yardline_fgsig_4th: 0.0227,
            yardline_puntsig_4th: 0.0102,
            yardline_pct: -0.7171,
            yardline_pct_sq: 0.8195,
            log_yardline_pct: 0.5411,
            fg_sigmoid: 0.0035,
            punt_sigmoid: -0.0012,
            goal_to_go_yardline: -0.0244,
            log_goal_to_go_yardline: 0.5523,
            yards_to_go_yardline: -0.6927,
            log_yards_to_go_yardline: -0.0112,
            yardline_4th: -0.2124,
            log_yardline_4th: 0.2406,
            yardline_not_4th: -0.5047,
            log_yardline_not_4th: 0.3005,
            inside_2m_warning: 0.1275,
            garbage_time_win: -0.0160,
            garbage_time_loss: -0.2525,
            clock_running: -0.0233,
            possdiff_per_minute: -0.2942,
            fgpossdiff_per_minute: -0.2285,
            ol_z: 0.1760,
            dl_z: -0.1028,
            ol_dl_z: -0.0040,
            log_mean_yards: 0.5543,
            log_std_yards: -0.7404,
            yoe_mean: 0.5281,
            yoe_std: -0.0401,
            togo_std: 0.0111,
            yardline_std: -0.0289,
            clock_runs_after: -0.8992,
        }
    }


    pub fn designed_run_pos_yards_coef() -> RushingModel {
        RushingModel {
            intercept: 1.3717,
            is_offense_home: 0.0035,
            offense_log_pass_prob: -0.0001,
            defense_log_pass_prob: -0.0002,
            off_def_lpp: 0.0001,
            off_lpp_rz: -0.0008,
            def_lpp_rz: -0.0005,
            off_def_lpp_rz: 0.0007,
            off_lpp_outside_rz: -0.0050,
            off_lpp_inside_rz: 0.0053,
            def_lpp_outside_rz: -0.0050,
            def_lpp_inside_rz: 0.0052,
            off_lpp_pdpm: 0.0015,
            def_lpp_pdpm: 0.0016,
            off_lpp_rz_pdpm: 0.0015,
            def_lpp_rz_pdpm: 0.0015,
            down_1: -0.0032,
            down_2: 0.0028,
            down_3: 0.0012,
            down_4: -0.0007,
            goal_to_go: -0.0048,
            z_ydstogo: 0.0053,
            ydstogo_pct: 0.0026,
            log_ydstogo_pct: 0.0057,
            to_go_1st: -0.0020,
            to_go_2nd: 0.0016,
            to_go_3rd: -0.0002,
            to_go_4th: -0.0008,
            log_to_go_1st: 0.0025,
            log_to_go_2nd: -0.0020,
            log_to_go_3rd: 0.0028,
            log_to_go_4th: 0.0025,
            fp_1st: -0.0009,
            fp_2nd: 0.0009,
            fp_3rd: 0.0006,
            fp_4th: 0.0000,
            yardline_fgsig_4th: -0.0004,
            yardline_puntsig_4th: 0.0073,
            yardline_pct: 0.0015,
            yardline_pct_sq: -0.0005,
            log_yardline_pct: 0.0146,
            fg_sigmoid: 0.0026,
            punt_sigmoid: 0.0104,
            goal_to_go_yardline: -0.0002,
            log_goal_to_go_yardline: 0.0148,
            yards_to_go_yardline: 0.0017,
            log_yards_to_go_yardline: -0.0002,
            yardline_4th: -0.0001,
            log_yardline_4th: 0.0013,
            yardline_not_4th: 0.0016,
            log_yardline_not_4th: 0.0133,
            inside_2m_warning: 0.0005,
            garbage_time_win: -0.0008,
            garbage_time_loss: -0.0003,
            clock_running: 0.0018,
            possdiff_per_minute: -0.0033,
            fgpossdiff_per_minute: -0.0039,
            ol_z: 0.0182,
            dl_z: -0.0214,
            ol_dl_z: -0.0044,
            log_mean_yards: 0.0040,
            log_std_yards: 0.0050,
            yoe_mean: 0.0169,
            yoe_std: 0.0257,
            togo_std: 0.0080,
            yardline_std: 0.0251,
            clock_runs_after: -0.0348,
        }
    }


    pub fn designed_run_pos_yards_var_coef() -> RushingModel {
        RushingModel {
            intercept: 2.5388,
            is_offense_home: 0.0572,
            offense_log_pass_prob: 0.0031,
            defense_log_pass_prob: 0.0041,
            off_def_lpp: -0.0034,
            off_lpp_rz: -0.0100,
            def_lpp_rz: -0.0066,
            off_def_lpp_rz: 0.0089,
            off_lpp_outside_rz: -0.0274,
            off_lpp_inside_rz: 0.0324,
            def_lpp_outside_rz: -0.0258,
            def_lpp_inside_rz: 0.0317,
            off_lpp_pdpm: -0.0039,
            def_lpp_pdpm: -0.0027,
            off_lpp_rz_pdpm: -0.0053,
            def_lpp_rz_pdpm: -0.0063,
            down_1: 0.0020,
            down_2: -0.0184,
            down_3: 0.0164,
            down_4: -0.0000,
            goal_to_go: -0.0163,
            z_ydstogo: 0.0126,
            ydstogo_pct: 0.0063,
            log_ydstogo_pct: 0.0149,
            to_go_1st: -0.0014,
            to_go_2nd: -0.0106,
            to_go_3rd: 0.0124,
            to_go_4th: -0.0032,
            log_to_go_1st: 0.0064,
            log_to_go_2nd: 0.0165,
            log_to_go_3rd: -0.0226,
            log_to_go_4th: 0.0146,
            fp_1st: 0.0014,
            fp_2nd: -0.0041,
            fp_3rd: 0.0025,
            fp_4th: 0.0018,
            yardline_fgsig_4th: -0.0374,
            yardline_puntsig_4th: 0.0163,
            yardline_pct: -0.0002,
            yardline_pct_sq: -0.0154,
            log_yardline_pct: 0.0537,
            fg_sigmoid: 0.0098,
            punt_sigmoid: 0.0239,
            goal_to_go_yardline: -0.0011,
            log_goal_to_go_yardline: 0.0444,
            yards_to_go_yardline: 0.0009,
            log_yards_to_go_yardline: 0.0093,
            yardline_4th: 0.0012,
            log_yardline_4th: 0.0032,
            yardline_not_4th: -0.0014,
            log_yardline_not_4th: 0.0504,
            inside_2m_warning: -0.0075,
            garbage_time_win: -0.0021,
            garbage_time_loss: -0.0056,
            clock_running: 0.0101,
            possdiff_per_minute: 0.0043,
            fgpossdiff_per_minute: 0.0032,
            ol_z: 0.0322,
            dl_z: -0.0986,
            ol_dl_z: -0.0189,
            log_mean_yards: 0.0272,
            log_std_yards: 0.0328,
            yoe_mean: 0.1149,
            yoe_std: 0.1263,
            togo_std: -0.0194,
            yardline_std: 0.1699,
            clock_runs_after: -0.4345,
        }
    }


    pub fn designed_run_neg_yards_coef() -> RushingModel {
        RushingModel {
            intercept: 0.1280,
            is_offense_home: 0.0002,
            offense_log_pass_prob: 0.0001,
            defense_log_pass_prob: 0.0000,
            off_def_lpp: -0.0001,
            off_lpp_rz: 0.0001,
            def_lpp_rz: -0.0001,
            off_def_lpp_rz: -0.0000,
            off_lpp_outside_rz: -0.0002,
            off_lpp_inside_rz: 0.0003,
            def_lpp_outside_rz: -0.0002,
            def_lpp_inside_rz: 0.0002,
            off_lpp_pdpm: -0.0002,
            def_lpp_pdpm: -0.0003,
            off_lpp_rz_pdpm: -0.0002,
            def_lpp_rz_pdpm: -0.0003,
            down_1: 0.0002,
            down_2: 0.0021,
            down_3: -0.0016,
            down_4: -0.0007,
            goal_to_go: -0.0015,
            z_ydstogo: 0.0040,
            ydstogo_pct: 0.0020,
            log_ydstogo_pct: 0.0082,
            to_go_1st: -0.0002,
            to_go_2nd: 0.0009,
            to_go_3rd: -0.0016,
            to_go_4th: -0.0007,
            log_to_go_1st: 0.0013,
            log_to_go_2nd: 0.0004,
            log_to_go_3rd: 0.0047,
            log_to_go_4th: 0.0018,
            fp_1st: 0.0003,
            fp_2nd: 0.0009,
            fp_3rd: -0.0000,
            fp_4th: -0.0000,
            yardline_fgsig_4th: 0.0022,
            yardline_puntsig_4th: 0.0068,
            yardline_pct: -0.0001,
            yardline_pct_sq: -0.0005,
            log_yardline_pct: 0.0044,
            fg_sigmoid: -0.0007,
            punt_sigmoid: -0.0032,
            goal_to_go_yardline: -0.0000,
            log_goal_to_go_yardline: 0.0070,
            yards_to_go_yardline: -0.0001,
            log_yards_to_go_yardline: -0.0027,
            yardline_4th: -0.0002,
            log_yardline_4th: 0.0013,
            yardline_not_4th: 0.0001,
            log_yardline_not_4th: 0.0030,
            inside_2m_warning: -0.0000,
            garbage_time_win: 0.0003,
            garbage_time_loss: -0.0001,
            clock_running: 0.0012,
            possdiff_per_minute: 0.0005,
            fgpossdiff_per_minute: 0.0003,
            ol_z: 0.0007,
            dl_z: 0.0026,
            ol_dl_z: -0.0021,
            log_mean_yards: -0.0005,
            log_std_yards: 0.0013,
            yoe_mean: -0.0018,
            yoe_std: 0.0074,
            togo_std: 0.0149,
            yardline_std: 0.0025,
            clock_runs_after: -0.0009,
        }
    }

}