use core::panic;
use std::collections::HashMap;

use crate::sim::{box_score::BoxScore, play_result::TurnoverOutcome};

#[derive(Debug, Clone)]
pub struct DefenseBoxScore {
    pub sacks: u8,
    pub interceptions: u8,
    pub touchdowns: u8,
    pub fumble_recoveries: u8,
    pub safeties: u8,
    pub blocked_kicks: u8,
    pub points_allowed: u8,
    pub offensive_penalties: u8,
    pub offensive_penalty_yards: u8,
    pub defensive_penalties: u8,
    pub defensive_penalty_yards: u8,
}

impl DefenseBoxScore {
    pub fn new() -> DefenseBoxScore {
        DefenseBoxScore {
            sacks: 0,
            interceptions: 0,
            touchdowns: 0,
            fumble_recoveries: 0,
            safeties: 0,
            blocked_kicks: 0,
            points_allowed: 0,
            offensive_penalties: 0,
            offensive_penalty_yards: 0,
            defensive_penalties: 0,
            defensive_penalty_yards: 0,
        }
    }

    pub fn new_map(home: String, away: String) -> HashMap<String, DefenseBoxScore> {
        let mut defenses = HashMap::new();
        defenses.insert(home, DefenseBoxScore::new());
        defenses.insert(away, DefenseBoxScore::new());
        defenses
    }

    pub fn add_sack(&mut self) {
        self.sacks += 1;
    }

    pub fn add_safety(&mut self) {
        self.safeties += 1;
    }

    pub fn add_interception(&mut self, turnover_outcome: &TurnoverOutcome) {
        self.interceptions += 1;
        self.handle_turnover_outcome(turnover_outcome);
    }

    pub fn add_fumble_recovery(&mut self, turnover_outcome: &TurnoverOutcome) {
        self.fumble_recoveries += 1;
        self.handle_turnover_outcome(turnover_outcome);
    }

    fn handle_turnover_outcome(&mut self, turnover_outcome: &TurnoverOutcome) {
        if let TurnoverOutcome::Touchdown = turnover_outcome {
            self.touchdowns += 1;
        }
        if let TurnoverOutcome::DefensiveSafetyReturn = turnover_outcome {
            self.safeties += 1;
        }
    }

    pub fn add_kick_block(&mut self, turnover_outcome: &TurnoverOutcome) {
        self.blocked_kicks += 1;
        self.handle_turnover_outcome(turnover_outcome);
    }

    pub fn standard_fantasy_points(&self) -> f32 {
        let mut points = match self.points_allowed {
            0 => 10.0,
            1..=6 => 7.0,
            7..=13 => 4.0,
            14..=20 => 1.0,
            21..=27 => 0.0,
            28..=34 => -1.0,
            _ => -4.0,
        };

        points += 1.0 * (self.sacks as f32);
        points += 2.0 * (self.interceptions as f32);
        points += 2.0 * (self.fumble_recoveries as f32);
        points += 6.0 * (self.touchdowns as f32);
        points += 2.0 * (self.safeties as f32);
        points += 2.0 * (self.blocked_kicks as f32);
        points
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DefenseProjection {
    pub sacks: f32,
    pub interceptions: f32,
    pub touchdowns: f32,
    pub fumble_recoveries: f32,
    pub safeties: f32,
    pub blocked_kicks: f32,
    pub points_allowed: f32,
    pub penalties: f32,
    pub penalty_yards: f32,
}

impl DefenseProjection {
    pub fn new() -> DefenseProjection {
        DefenseProjection {
            sacks: 0.0,
            interceptions: 0.0,
            touchdowns: 0.0,
            fumble_recoveries: 0.0,
            safeties: 0.0,
            blocked_kicks: 0.0,
            points_allowed: 0.0,
            penalties: 0.0,
            penalty_yards: 0.0,
        }
    }

    pub fn add(&mut self, stats: &DefenseBoxScore, n_sims: f32) {
        self.sacks += stats.sacks as f32 / n_sims;
        self.interceptions += stats.interceptions as f32 / n_sims;
        self.fumble_recoveries += stats.fumble_recoveries as f32 / n_sims;
        self.touchdowns += stats.touchdowns as f32 / n_sims;
        self.safeties += stats.safeties as f32 / n_sims;
        self.blocked_kicks += stats.blocked_kicks as f32 / n_sims;
        self.points_allowed += stats.points_allowed as f32 / n_sims;

        self.penalties += stats.defensive_penalties as f32 / n_sims;
        self.penalty_yards += stats.defensive_penalty_yards as f32 / n_sims;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OffenseProjection {
    pub points: f32,
    pub points_sampled: Vec<u8>,
    pub plays_called: f32,
    pub rushes: f32,
    pub dropbacks: f32,
    pub targets: f32,
    pub penalties: f32,
    pub penalty_yards: f32,
}

impl OffenseProjection {
    pub fn new() -> OffenseProjection {
        OffenseProjection {
            points: 0.0,
            points_sampled: vec![],
            plays_called: 0.0,
            rushes: 0.0,
            dropbacks: 0.0,
            targets: 0.0,
            penalties: 0.0,
            penalty_yards: 0.0,
        }
    }

    pub fn add(&mut self, team: &String, box_score: &BoxScore, n_sims: f32) {
        let (points, plays) = match (team == &box_score.home, team == &box_score.away) {
            (true, false) => (box_score.score.home, &box_score.plays.home),
            (false, true) => (box_score.score.away, &box_score.plays.away),
            _ => panic!("team does not match"),
        };
        self.points += points as f32 / n_sims;
        self.points_sampled.push(points);

        self.plays_called += plays.total as f32 / n_sims;
        self.rushes += plays.run as f32 / n_sims;
        self.dropbacks += plays.dropbacks as f32 / n_sims;
        self.targets += plays.targets as f32 / n_sims;

        let def_box_score = box_score.defenses.get(team).unwrap();
        self.penalties += def_box_score.offensive_penalties as f32 / n_sims;
        self.penalty_yards += def_box_score.offensive_penalty_yards as f32 / n_sims;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpecialTeamsProjection {
    pub fg_attempts: f32,
    pub fg_made: f32,
    pub punts: f32,
}

impl SpecialTeamsProjection {
    pub fn new() -> SpecialTeamsProjection {
        SpecialTeamsProjection {
            fg_attempts: 0.0,
            fg_made: 0.0,
            punts: 0.0,
        }
    }

    pub fn add(&mut self, team: &String, box_score: &BoxScore, n_sims: f32) {
        let kickers = box_score.kickers.get(team).unwrap();

        let fgs_made = kickers.fgs_made.len() as f32;
        let fgs_missed = kickers.fgs_missed.len() as f32;
        self.fg_attempts += (fgs_made + fgs_missed) / n_sims;
        self.fg_made += fgs_made / n_sims;
        self.punts += kickers.num_punts as f32 / n_sims;
    }
}
