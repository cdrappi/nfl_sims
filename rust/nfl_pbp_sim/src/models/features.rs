use crate::{
    sim::GameSim,
    start::HomeAway,
    state::down::{Down, PlayState},
};

pub const EPSILON: f32 = 1e-6;
pub const PROB_PASS: f32 = 0.61;

#[derive(Debug)]
pub struct PlaycallFeatures {
    pub clock_running: f32,
    pub inv_half_minutes: f32,
    pub log_inv_half_minutes: f32,
    pub inv_game_minutes: f32,
    pub log_inv_game_minutes: f32,
    pub possession_diff: f32,
    pub fg_possession_diff: f32,
    pub possdiff_per_minute: f32,
    pub fgpossdiff_per_minute: f32,
    pub clock_runs_pdpm: f32,
    pub clock_runs_fgpdpm: f32,
    pub clock_runs_pdpm2: f32,
    pub clock_runs_fgpdpm2: f32,
    pub off_timeouts_remaining_0: f32,
    pub off_timeouts_remaining_1: f32,
    pub off_timeouts_remaining_2: f32,
    pub off_timeouts_remaining_3: f32,
    pub def_timeouts_remaining_0: f32,
    pub def_timeouts_remaining_1: f32,
    pub def_timeouts_remaining_2: f32,
    pub def_timeouts_remaining_3: f32,
    pub clock_runs_pdpm_off0to: f32,
    pub clock_runs_pdpm_off1to: f32,
    pub clock_runs_pdpm_off2to: f32,
    pub clock_runs_pdpm_off3to: f32,
    pub clock_runs_pdpm_def0to: f32,
    pub clock_runs_pdpm_def1to: f32,
    pub clock_runs_pdpm_def2to: f32,
    pub clock_runs_pdpm_def3to: f32,
    pub is_offense_home: f32,
    pub offense_log_pass_prob: f32,
    pub defense_log_pass_prob: f32,
    pub off_lpp_rz: f32,
    pub def_lpp_rz: f32,
    pub off_def_lpp: f32,
    pub off_def_lpp_rz: f32,
    pub off_lpp_pdpm: f32,
    pub def_lpp_pdpm: f32,
    pub off_lpp_rz_pdpm: f32,
    pub def_lpp_rz_pdpm: f32,
    pub off_lpp_inside_rz: f32,
    pub off_lpp_outside_rz: f32,
    pub def_lpp_inside_rz: f32,
    pub def_lpp_outside_rz: f32,
    pub offense_penalty_z: f32,
    pub defense_penalty_z: f32,
    pub down_1: f32,
    pub down_2: f32,
    pub down_3: f32,
    pub down_4: f32,
    pub goal_to_go: f32,
    pub z_ydstogo: f32,
    pub ydstogo_pct: f32,
    pub log_ydstogo_pct: f32,
    pub to_go_1st: f32,
    pub to_go_2nd: f32,
    pub to_go_3rd: f32,
    pub to_go_4th: f32,
    pub log_to_go_1st: f32,
    pub log_to_go_2nd: f32,
    pub log_to_go_3rd: f32,
    pub log_to_go_4th: f32,
    pub fp_1st: f32,
    pub fp_2nd: f32,
    pub fp_3rd: f32,
    pub fp_4th: f32,
    pub fg_sigmoid: f32,
    pub punt_sigmoid: f32,
    pub yardline_pct: f32,
    pub yardline_pct_sq: f32,
    pub log_yardline_pct: f32,
    pub yardline_fgsig_4th: f32,
    pub yardline_puntsig_4th: f32,
    pub goal_to_go_yardline: f32,
    pub log_goal_to_go_yardline: f32,
    pub yards_to_go_yardline: f32,
    pub log_yards_to_go_yardline: f32,
    pub yardline_4th: f32,
    pub log_yardline_4th: f32,
    pub yardline_not_4th: f32,
    pub log_yardline_not_4th: f32,
    pub inside_2m_warning: f32,
    pub garbage_time_win: f32,
    pub garbage_time_loss: f32,
    pub oline_rushing_z: f32,
    pub dline_rushing_z: f32,
    pub offense_pace_z: f32,
    pub offense_pass_rush_z: f32,
    pub defense_pass_rush_z: f32,
    pub qb_scramble_rate: f32,
    pub qb_prob_sack_given_hit: f32,
    pub defense_completion_z: f32,
    pub defense_interception_z: f32,
}

fn fg_make_prob(yardline_100: u8) -> f32 {
    if yardline_100 > 60 {
        // prevent overflow
        return 0.0;
    }
    let distance = yardline_100 + 17;
    match distance {
        0..=34 => 1.0,
        35..=39 => 0.9,
        40..=44 => 0.8,
        45..=49 => 0.7,
        50..=54 => 0.6,
        55..=59 => 0.45,
        60..=64 => 0.3,
        65..=69 => 0.1,
        _ => 0.0,
    }
}

fn punt_value(yardline_100: u8) -> f32 {
    match yardline_100 {
        0..=39 => 0.0,
        40..=49 => 0.2,
        50..=54 => 0.4,
        55..=59 => 0.6,
        60..=64 => 0.8,
        _ => 1.0,
    }
}

impl PlaycallFeatures {
    pub fn new(sim: &GameSim) -> PlaycallFeatures {
        let game_state = &sim.game_state;

        let possession = game_state.play.possession();
        let offense = sim.game_params.get_team(possession);
        let defense = sim.game_params.get_team(possession.flip());

        let clock_running = match game_state.clock.running {
            true => 1.0,
            false => 0.0,
        };

        let off_to_remaining = game_state.timeouts_used.timeouts_remaining(possession);
        let def_to_remaining = game_state
            .timeouts_used
            .timeouts_remaining(possession.flip());

        let off_timeouts_remaining_0 = if off_to_remaining == 0 { 1.0 } else { 0.0 };
        let off_timeouts_remaining_1 = if off_to_remaining == 1 { 1.0 } else { 0.0 };
        let off_timeouts_remaining_2 = if off_to_remaining == 2 { 1.0 } else { 0.0 };
        let off_timeouts_remaining_3 = if off_to_remaining == 3 { 1.0 } else { 0.0 };

        let def_timeouts_remaining_0 = if def_to_remaining == 0 { 1.0 } else { 0.0 };
        let def_timeouts_remaining_1 = if def_to_remaining == 1 { 1.0 } else { 0.0 };
        let def_timeouts_remaining_2 = if def_to_remaining == 2 { 1.0 } else { 0.0 };
        let def_timeouts_remaining_3 = if def_to_remaining == 3 { 1.0 } else { 0.0 };

        let (down_1, down_2, down_3, down_4) = match game_state.play {
            PlayState::Down(down_to_go) => match down_to_go.down {
                Down::First => (1.0, 0.0, 0.0, 0.0),
                Down::Second => (0.0, 1.0, 0.0, 0.0),
                Down::Third => (0.0, 0.0, 1.0, 0.0),
                Down::Fourth => (0.0, 0.0, 0.0, 1.0),
            },
            _ => (0.0, 0.0, 0.0, 0.0),
        };

        let is_offense_home = match sim.game_params.neutral_field {
            true => 0.5,
            false => match possession {
                HomeAway::Home => 1.0,
                HomeAway::Away => 0.0,
            },
        };

        let goal_to_go = match game_state.play.goal_to_go() {
            true => 1.0,
            false => 0.0,
        };
        let yards_to_go = game_state.play.safe_yards_for_first() as f32;
        let yardline_100 = game_state.play.safe_yards_for_touchdown();

        let yardline_pct = yardline_100 as f32 / 100.0;
        let z_ydstogo = (yards_to_go - 10.0) / 5.0;
        let ydstogo_pct = yards_to_go / 10.0;
        let ydstogo_sigmoid = (-0.1 * yards_to_go).exp();

        let possession_diff = game_state.score.possession_diff(possession) as f32;
        let fg_possession_diff = game_state.score.fg_possession_diff(possession) as f32;

        let to_go_1st = down_1 * ydstogo_sigmoid;
        let to_go_2nd = down_2 * ydstogo_sigmoid;
        let to_go_3rd = down_3 * ydstogo_sigmoid;
        let to_go_4th = down_4 * ydstogo_sigmoid;

        let log_ydstogo_pct = ydstogo_pct.max(0.1).ln();
        let log_to_go_1st = down_1 * log_ydstogo_pct;
        let log_to_go_2nd = down_2 * log_ydstogo_pct;
        let log_to_go_3rd = down_3 * log_ydstogo_pct;
        let log_to_go_4th = down_4 * log_ydstogo_pct;

        let yardline_pct_sq = yardline_pct * yardline_pct;
        let log_yardline_pct = yardline_pct.max(0.01).ln();

        let fp_1st = to_go_1st * ydstogo_pct;
        let fp_2nd = to_go_2nd * ydstogo_pct;
        let fp_3rd = to_go_3rd * ydstogo_pct;
        let fp_4th = to_go_4th * ydstogo_pct;

        let fg_sigmoid = (fg_make_prob(yardline_100) + EPSILON).ln();
        let punt_sigmoid = (punt_value(yardline_100) + EPSILON).ln();

        // let _yardline_fgsig_4th = down_4 * fg_sigmoid;
        // let _yardline_puntsig_4th = down_4 * punt_sigmoid;
        let game_minutes_remaining = game_state.clock.game_minutes_remaining();
        let half_minutes_remaining = game_state.clock.half_minutes_remaining();

        let inv_half_minutes = (1.0 / half_minutes_remaining).max(0.0).min(6.0);
        let inv_game_minutes = (1.0 / game_minutes_remaining).max(0.0).min(6.0);

        let possdiff_per_minute = possession_diff * inv_game_minutes;
        let fgpossdiff_per_minute = fg_possession_diff * inv_game_minutes;
        let pdpm2 = possdiff_per_minute * possdiff_per_minute.abs();
        let fgpdpm2 = fg_possession_diff * fg_possession_diff.abs();

        let garbage_time = match (game_minutes_remaining < 5.0, possession_diff.abs() >= 3.0) {
            (true, true) => inv_game_minutes.max(1.0).min(3.0),
            _ => 0.0,
        };

        let offense_log_pass_prob = (PROB_PASS + offense.team.offense_proe).ln();
        let defense_log_pass_prob = (PROB_PASS + defense.team.defense_proe).ln();

        let off_lpp_rz = (PROB_PASS + offense.team.offense_rz_proe).ln();
        let def_lpp_rz = (PROB_PASS + defense.team.defense_rz_proe).ln();

        let inside_rz = match yardline_100 {
            0..=20 => 1.0,
            _ => 0.0,
        };

        PlaycallFeatures {
            clock_running,
            inv_half_minutes,
            log_inv_half_minutes: inv_half_minutes.ln(),
            inv_game_minutes,
            log_inv_game_minutes: inv_game_minutes.ln(),
            possession_diff,
            fg_possession_diff,
            possdiff_per_minute,
            clock_runs_pdpm: clock_running * possdiff_per_minute,
            clock_runs_fgpdpm: clock_running * fgpossdiff_per_minute,
            clock_runs_pdpm2: clock_running * pdpm2,
            clock_runs_fgpdpm2: clock_running * fgpdpm2,
            off_timeouts_remaining_0,
            off_timeouts_remaining_1,
            off_timeouts_remaining_2,
            off_timeouts_remaining_3,
            def_timeouts_remaining_0,
            def_timeouts_remaining_1,
            def_timeouts_remaining_2,
            def_timeouts_remaining_3,
            clock_runs_pdpm_off0to: clock_running
                * possdiff_per_minute
                * off_timeouts_remaining_0 as f32,
            clock_runs_pdpm_off1to: clock_running
                * possdiff_per_minute
                * off_timeouts_remaining_1 as f32,
            clock_runs_pdpm_off2to: clock_running
                * possdiff_per_minute
                * off_timeouts_remaining_2 as f32,
            clock_runs_pdpm_off3to: clock_running
                * possdiff_per_minute
                * off_timeouts_remaining_3 as f32,
            clock_runs_pdpm_def0to: clock_running
                * possdiff_per_minute
                * def_timeouts_remaining_0 as f32,
            clock_runs_pdpm_def1to: clock_running
                * possdiff_per_minute
                * def_timeouts_remaining_1 as f32,
            clock_runs_pdpm_def2to: clock_running
                * possdiff_per_minute
                * def_timeouts_remaining_2 as f32,
            clock_runs_pdpm_def3to: clock_running
                * possdiff_per_minute
                * def_timeouts_remaining_3 as f32,
            is_offense_home,
            offense_log_pass_prob,
            defense_log_pass_prob,
            off_lpp_rz,
            def_lpp_rz,
            off_def_lpp: offense_log_pass_prob * defense_log_pass_prob,
            off_def_lpp_rz: off_lpp_rz * def_lpp_rz,
            off_lpp_pdpm: offense_log_pass_prob * possdiff_per_minute,
            def_lpp_pdpm: defense_log_pass_prob * possdiff_per_minute,
            off_lpp_rz_pdpm: off_lpp_rz * possdiff_per_minute,
            def_lpp_rz_pdpm: def_lpp_rz * possdiff_per_minute,
            off_lpp_outside_rz: offense_log_pass_prob * (1.0 - inside_rz),
            def_lpp_outside_rz: defense_log_pass_prob * (1.0 - inside_rz),
            off_lpp_inside_rz: off_lpp_rz * inside_rz,
            def_lpp_inside_rz: def_lpp_rz * inside_rz,
            down_1,
            down_2,
            down_3,
            down_4,
            goal_to_go,
            z_ydstogo,
            ydstogo_pct,
            log_ydstogo_pct: ydstogo_pct.max(0.01).ln(),
            to_go_1st,
            to_go_2nd,
            to_go_3rd,
            to_go_4th,
            log_to_go_1st,
            log_to_go_2nd,
            log_to_go_3rd,
            log_to_go_4th,
            fp_1st,
            fp_2nd,
            fp_3rd,
            fp_4th,
            fg_sigmoid,
            punt_sigmoid,
            yardline_pct,
            yardline_pct_sq,
            log_yardline_pct,
            yardline_fgsig_4th: down_4 * fg_sigmoid,
            yardline_puntsig_4th: down_4 * punt_sigmoid,
            fgpossdiff_per_minute,
            goal_to_go_yardline: goal_to_go * yardline_pct,
            log_goal_to_go_yardline: goal_to_go * log_yardline_pct,
            yards_to_go_yardline: (1.0 - goal_to_go) * yardline_pct,
            log_yards_to_go_yardline: (1.0 - goal_to_go) * log_yardline_pct,
            offense_penalty_z: offense.team.offense_penalty_z,
            defense_penalty_z: defense.team.defense_penalty_z,
            yardline_4th: down_4 * yardline_pct,
            log_yardline_4th: down_4 * log_yardline_pct,
            yardline_not_4th: (1.0 - down_4) * yardline_pct,
            log_yardline_not_4th: (1.0 - down_4) * log_yardline_pct,
            inside_2m_warning: match half_minutes_remaining < 2.0 {
                true => 1.0,
                false => 0.0,
            },
            garbage_time_win: garbage_time * ((possession_diff >= 3.0) as i8 as f32),
            garbage_time_loss: garbage_time * ((possession_diff <= -3.0) as i8 as f32),
            oline_rushing_z: offense.team.oline_rushing_z,
            dline_rushing_z: defense.team.dline_rushing_z,
            offense_pace_z: offense.team.pace_z,
            offense_pass_rush_z: offense.team.offense_pass_rush_z,
            defense_pass_rush_z: defense.team.defense_pass_rush_z,
            qb_scramble_rate: offense.qbs[0].scramble_rate,
            qb_prob_sack_given_hit: offense.qbs[0].prob_sack_given_hit,
            defense_completion_z: defense.team.defense_completion_z,
            defense_interception_z: defense.team.defense_interception_z,
        }
    }
}
