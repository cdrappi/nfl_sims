use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct KickerBoxScore {
    pub fgs_made: Vec<u8>,
    pub fgs_missed: Vec<u8>,
    pub num_punts: u8,
    pub pats_made: u8,
    pub pats_missed: u8,
}

impl KickerBoxScore {
    pub fn new() -> KickerBoxScore {
        KickerBoxScore {
            fgs_made: Vec::new(),
            fgs_missed: Vec::new(),
            num_punts: 0,
            pats_made: 0,
            pats_missed: 0,
        }
    }

    pub fn add_made_fg(&mut self, yards: u8) {
        self.fgs_made.push(yards);
    }

    pub fn add_missed_fg(&mut self, yards: u8) {
        self.fgs_missed.push(yards);
    }

    pub fn add_punt(&mut self) {
        self.num_punts += 1;
    }

    pub fn add_pat_attempt(&mut self, made: bool) {
        match made {
            true => self.pats_made += 1,
            false => self.pats_missed += 1,
        }
    }

    pub fn new_map(home: String, away: String) -> HashMap<String, KickerBoxScore> {
        let mut kickers = HashMap::new();
        kickers.insert(home, KickerBoxScore::new());
        kickers.insert(away, KickerBoxScore::new());
        kickers
    }

    pub fn standard_fantasy_points(&self) -> f32 {
        let mut points = self.pats_made as f32;
        for fg_made_distance in &self.fgs_made {
            let pts = match fg_made_distance {
                0..=39 => 3.0,
                40..=49 => 4.0,
                _ => 5.0,
            };
            points += pts;
        }
        points
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct KickerProjection {
    pub fgs_made: f32,
    pub fgs_attempted: f32,
    pub fg_made_u30: f32,
    pub fg_made_30_39: f32,
    pub fg_made_40_49: f32,
    pub fg_made_50o: f32,
    pub pats_made: f32,
    pub pats_attempted: f32,
    pub punts: f32,
}

impl KickerProjection {
    pub fn new() -> KickerProjection {
        KickerProjection {
            fgs_made: 0.0,
            fgs_attempted: 0.0,
            fg_made_u30: 0.0,
            fg_made_30_39: 0.0,
            fg_made_40_49: 0.0,
            fg_made_50o: 0.0,
            pats_made: 0.0,
            pats_attempted: 0.0,
            punts: 0.0,
        }
    }
}
