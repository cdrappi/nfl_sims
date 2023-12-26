use crate::util::stats::{double_truncated_poisson, random_bool};

use crate::models::features::PlaycallFeatures;
use crate::sim::{play_result::PlayResult, GameSim};

pub mod coef;

pub const PLAYCLOCK: u8 = 40;

const MAX_NEXT_PLAY_CLOCK: u8 = 30;
const MIN_NEXT_PLAY_CLOCK: u8 = 1;

const MIN_TIME_TO_SPOT: u8 = 2;
const MAX_TIME_TO_SPOT_PAUSED: u8 = 4;

pub const MAX_PLAY_DURATION: u8 = 15;
pub const MIN_PLAY_DURATION: u8 = 3;

#[derive(Debug)]
pub struct ClockModel {
    intercept: f32,
    play_type_field_goal: f32,
    play_type_kickoff: f32,
    play_type_no_play: f32,
    play_type_pass: f32,
    play_type_punt: f32,
    play_type_run: f32,
    yards_gained_pct: f32,
    yards_gained_pct_sq: f32,
    play_type_pass_yards_pct: f32,
    play_type_pass_yards_pct_sq: f32,
    play_type_run_yards_pct: f32,
    play_type_run_yards_pct_sq: f32,
    clock_running: f32,
    inv_half_minutes: f32,
    log_inv_half_minutes: f32,
    inv_game_minutes: f32,
    log_inv_game_minutes: f32,
    possession_diff: f32,
    fg_possession_diff: f32,
    possdiff_per_minute: f32,
    fgpossdiff_per_minute: f32,
    clock_runs_pdpm: f32,
    clock_runs_fgpdpm: f32,
    clock_runs_pdpm2: f32,
    clock_runs_fgpdpm2: f32,
    off_timeouts_remaining_0: f32,
    off_timeouts_remaining_1: f32,
    off_timeouts_remaining_2: f32,
    off_timeouts_remaining_3: f32,
    clock_runs_pdpm_off0to: f32,
    clock_runs_pdpm_off1to: f32,
    clock_runs_pdpm_off2to: f32,
    clock_runs_pdpm_off3to: f32,
    def_timeouts_remaining_0: f32,
    def_timeouts_remaining_1: f32,
    def_timeouts_remaining_2: f32,
    def_timeouts_remaining_3: f32,
    clock_runs_pdpm_def0to: f32,
    clock_runs_pdpm_def1to: f32,
    clock_runs_pdpm_def2to: f32,
    clock_runs_pdpm_def3to: f32,
    play_duration: f32,
    pace_z: f32,
}
impl ClockModel {
    pub fn sim_play_duration(sim: &GameSim, play_result: &PlayResult) -> u16 {
        match play_result {
            PlayResult::PointAfterTouchdown(_) => 0,
            PlayResult::Timeout(_) => 0,
            PlayResult::QbKneel(_) => 3,
            PlayResult::QbSpike(_) => 1,
            _ => ClockModel::sample_play_duration(sim, play_result),
        }
    }

    fn sample_play_duration(sim: &GameSim, play_result: &PlayResult) -> u16 {
        let f = ClockModel::features(sim, play_result, 0);
        let coefs = ClockModel::play_duration_coefs();

        let z = ClockModel::get_z(&f, &coefs)
            .exp()
            .min(MAX_PLAY_DURATION as f32);

        double_truncated_poisson(z, MIN_PLAY_DURATION, MAX_PLAY_DURATION + 1) as u16
    }

    fn gen_clock_model(c: ClockModel, f: &ClockModel, min_inclusive: u8, max_exclusive: u8) -> u8 {
        let z = ClockModel::get_z(f.clone(), &c)
            .max(min_inclusive as f32)
            .min(max_exclusive as f32);

        double_truncated_poisson(z, min_inclusive, max_exclusive)
    }

    pub fn sim_runoff(sim: &GameSim, play_result: &PlayResult, play_duration: u16) -> u16 {
        let f = ClockModel::features(sim, play_result, play_duration);
        let coefs = ClockModel::running_next_play_clock_coefs();
        let next_play_clock =
            ClockModel::gen_clock_model(coefs, &f, MIN_NEXT_PLAY_CLOCK, MAX_NEXT_PLAY_CLOCK);
        (PLAYCLOCK - next_play_clock) as u16
    }

    pub fn sim_paused_runoff(sim: &GameSim, play_result: &PlayResult, play_duration: u16) -> u16 {
        let f = ClockModel::features(sim, play_result, play_duration);
        let time_to_spot = match random_bool(0.8) {
            true => {
                // this case is for when the ball carrier goes out of bounds and is going *forwards*.
                // the play clock starts immediately, but the game clock stops until the ball is re-spotted.
                let tts_c = ClockModel::time_to_spot_coefs();
                ClockModel::gen_clock_model(tts_c, &f, MIN_TIME_TO_SPOT, MAX_TIME_TO_SPOT_PAUSED)
                    as u8
            }
            false => {
                // if the ball carrier goes out of bounds and is going *backwards*,
                // then game clock stops until the ball is re-spotted,
                // but the play clock keeps running
                0
            }
        };

        let coefs = ClockModel::paused_next_play_clock_coefs();
        let next_play_clock =
            ClockModel::gen_clock_model(coefs, &f, MIN_NEXT_PLAY_CLOCK, MAX_NEXT_PLAY_CLOCK);

        (PLAYCLOCK - time_to_spot - next_play_clock) as u16
    }

    fn get_z(f: &ClockModel, c: &ClockModel) -> f32 {
        let mut z = c.intercept;
        z += c.play_type_field_goal * f.play_type_field_goal;
        z += c.play_type_kickoff * f.play_type_kickoff;
        z += c.play_type_no_play * f.play_type_no_play;
        z += c.play_type_pass * f.play_type_pass;
        z += c.play_type_punt * f.play_type_punt;
        z += c.play_type_run * f.play_type_run;
        z += c.yards_gained_pct * f.yards_gained_pct;
        z += c.yards_gained_pct_sq * f.yards_gained_pct_sq;
        z += c.play_type_pass_yards_pct * f.play_type_pass_yards_pct;
        z += c.play_type_pass_yards_pct_sq * f.play_type_pass_yards_pct_sq;
        z += c.play_type_run_yards_pct * f.play_type_run_yards_pct;
        z += c.play_type_run_yards_pct_sq * f.play_type_run_yards_pct_sq;
        z += c.clock_running * f.clock_running;
        z += c.inv_half_minutes * f.inv_half_minutes;
        z += c.log_inv_half_minutes * f.log_inv_half_minutes;
        z += c.inv_game_minutes * f.inv_game_minutes;
        z += c.log_inv_game_minutes * f.log_inv_game_minutes;
        z += c.possession_diff * f.possession_diff;
        z += c.fg_possession_diff * f.fg_possession_diff;
        z += c.possdiff_per_minute * f.possdiff_per_minute;
        z += c.fgpossdiff_per_minute * f.fgpossdiff_per_minute;
        z += c.clock_runs_pdpm * f.clock_runs_pdpm;
        z += c.clock_runs_fgpdpm * f.clock_runs_fgpdpm;
        z += c.clock_runs_pdpm2 * f.clock_runs_pdpm2;
        z += c.clock_runs_fgpdpm2 * f.clock_runs_fgpdpm2;
        z += c.off_timeouts_remaining_0 * f.off_timeouts_remaining_0;
        z += c.off_timeouts_remaining_1 * f.off_timeouts_remaining_1;
        z += c.off_timeouts_remaining_2 * f.off_timeouts_remaining_2;
        z += c.off_timeouts_remaining_3 * f.off_timeouts_remaining_3;
        z += c.clock_runs_pdpm_off0to * f.clock_runs_pdpm_off0to;
        z += c.clock_runs_pdpm_off1to * f.clock_runs_pdpm_off1to;
        z += c.clock_runs_pdpm_off2to * f.clock_runs_pdpm_off2to;
        z += c.clock_runs_pdpm_off3to * f.clock_runs_pdpm_off3to;
        z += c.def_timeouts_remaining_0 * f.def_timeouts_remaining_0;
        z += c.def_timeouts_remaining_1 * f.def_timeouts_remaining_1;
        z += c.def_timeouts_remaining_2 * f.def_timeouts_remaining_2;
        z += c.def_timeouts_remaining_3 * f.def_timeouts_remaining_3;
        z += c.clock_runs_pdpm_def0to * f.clock_runs_pdpm_def0to;
        z += c.clock_runs_pdpm_def1to * f.clock_runs_pdpm_def1to;
        z += c.clock_runs_pdpm_def2to * f.clock_runs_pdpm_def2to;
        z += c.clock_runs_pdpm_def3to * f.clock_runs_pdpm_def3to;
        z += c.play_duration * f.play_duration;
        z += c.pace_z * f.pace_z;

        z
    }

    fn features(sim: &GameSim, play_result: &PlayResult, play_duration: u16) -> ClockModel {
        let f = PlaycallFeatures::new(sim);

        let play_type_field_goal = play_result.is_field_goal();
        let play_type_kickoff = play_result.is_kickoff();
        let play_type_no_play = play_result.is_no_play();
        let play_type_pass = play_result.is_pass();
        let play_type_punt = play_result.is_punt();
        let play_type_run = play_result.is_run();

        let yards_gained = play_result.yards_gained(sim.game_state.play.safe_yards_for_touchdown());
        let yards_gained_pct = yards_gained as f32 / 100.0;
        let yards_gained_pct_sq = yards_gained_pct.powi(2);

        ClockModel {
            intercept: 1.0,
            play_type_field_goal,
            play_type_kickoff,
            play_type_no_play,
            play_type_pass,
            play_type_punt,
            play_type_run,
            yards_gained_pct,
            yards_gained_pct_sq,
            play_type_pass_yards_pct: play_type_pass * yards_gained_pct,
            play_type_run_yards_pct: play_type_run * yards_gained_pct,
            play_type_pass_yards_pct_sq: play_type_pass * yards_gained_pct_sq,
            play_type_run_yards_pct_sq: play_type_run * yards_gained_pct_sq,
            clock_running: f.clock_running,
            inv_half_minutes: f.inv_half_minutes,
            log_inv_half_minutes: f.log_inv_half_minutes,
            inv_game_minutes: f.inv_game_minutes,
            log_inv_game_minutes: f.log_inv_game_minutes,
            possession_diff: f.possession_diff,
            fg_possession_diff: f.fg_possession_diff,
            possdiff_per_minute: f.possdiff_per_minute,
            fgpossdiff_per_minute: f.fgpossdiff_per_minute,
            clock_runs_pdpm: f.clock_runs_pdpm,
            clock_runs_fgpdpm: f.clock_runs_fgpdpm,
            clock_runs_pdpm2: f.clock_runs_pdpm2,
            clock_runs_fgpdpm2: f.clock_runs_fgpdpm2,
            off_timeouts_remaining_0: f.off_timeouts_remaining_0,
            off_timeouts_remaining_1: f.off_timeouts_remaining_1,
            off_timeouts_remaining_2: f.off_timeouts_remaining_2,
            off_timeouts_remaining_3: f.off_timeouts_remaining_3,
            clock_runs_pdpm_off0to: f.clock_runs_pdpm_off0to,
            clock_runs_pdpm_off1to: f.clock_runs_pdpm_off1to,
            clock_runs_pdpm_off2to: f.clock_runs_pdpm_off2to,
            clock_runs_pdpm_off3to: f.clock_runs_pdpm_off3to,
            def_timeouts_remaining_0: f.def_timeouts_remaining_0,
            def_timeouts_remaining_1: f.def_timeouts_remaining_1,
            def_timeouts_remaining_2: f.def_timeouts_remaining_2,
            def_timeouts_remaining_3: f.def_timeouts_remaining_3,
            clock_runs_pdpm_def0to: f.clock_runs_pdpm_def0to,
            clock_runs_pdpm_def1to: f.clock_runs_pdpm_def1to,
            clock_runs_pdpm_def2to: f.clock_runs_pdpm_def2to,
            clock_runs_pdpm_def3to: f.clock_runs_pdpm_def3to,
            play_duration: play_duration as f32,
            pace_z: f.offense_pace_z,
        }
    }
}
