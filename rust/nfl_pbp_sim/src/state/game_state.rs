use crate::start::{GameStart, HomeAway};
use crate::state::clock::{GameClock, Quarter};
use crate::state::down::{DownToGo, PlayState, RushingSituation};

#[derive(Debug, Clone)]
pub struct Score {
    pub home: u8,
    pub away: u8,
}

impl Score {
    pub fn update(&mut self, score: &Score) {
        self.home = score.home;
        self.away = score.away;
    }

    pub fn new() -> Score {
        Score { home: 0, away: 0 }
    }

    fn is_game_tied(&self) -> bool {
        self.home == self.away
    }

    pub fn possession_diff(&self, possession: HomeAway) -> i8 {
        self.possdiff_score_n(possession, 0)
    }

    pub fn possdiff_score_n(&self, possession: HomeAway, n: i8) -> i8 {
        let (off_score, def_score) = match possession {
            HomeAway::Home => (n + self.home as i8, self.away as i8),
            HomeAway::Away => (n + self.away as i8, self.home as i8),
        };
        if off_score == def_score {
            return 0;
        }
        let diff = off_score - def_score;
        diff / 8 + if diff > 0 { 1 } else { -1 }
    }

    pub fn fg_possession_diff(&self, possession: HomeAway) -> i8 {
        self.possdiff_score_n(possession, 3)
    }
}

#[derive(Debug, Clone)]
pub struct TeamPlays {
    pub total: u8,
    pub run: u8,
    pub run_1ytg: u8,
    pub run_gz: u8,
    pub dropbacks: u8,
    pub targets: u8,
    pub targets_rz: u8,
}

impl TeamPlays {
    pub fn new() -> TeamPlays {
        TeamPlays {
            total: 0,
            run: 0,
            run_1ytg: 0,
            run_gz: 0,
            dropbacks: 0,
            targets: 0,
            targets_rz: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Plays {
    pub home: TeamPlays,
    pub away: TeamPlays,
}

impl Plays {
    pub fn new() -> Plays {
        Plays {
            home: TeamPlays::new(),
            away: TeamPlays::new(),
        }
    }

    pub fn increment(
        &mut self,
        possession: HomeAway,
        is_pass: bool,
        is_target: bool,
        down_to_go: DownToGo,
    ) {
        let team_plays = match possession {
            HomeAway::Home => &mut self.home,
            HomeAway::Away => &mut self.away,
        };
        team_plays.total += 1;
        match is_pass {
            true => {
                if is_target {
                    team_plays.targets += 1;
                    if down_to_go.is_redzone() {
                        team_plays.targets_rz += 1;
                    }
                }
                team_plays.dropbacks += 1;
            }
            false => {
                team_plays.run += 1;
                match down_to_go.rushing_situation() {
                    RushingSituation::OneYardToGo => {
                        team_plays.run_1ytg += 1;
                    }
                    RushingSituation::GreenZone => {
                        team_plays.run_gz += 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct TimeoutsUsed {
    pub home: u8,
    pub away: u8,
}

impl TimeoutsUsed {
    pub fn new() -> TimeoutsUsed {
        TimeoutsUsed { home: 0, away: 0 }
    }

    pub fn use_timeout(&mut self, home: bool) {
        if home {
            self.home += 1;
        } else {
            self.away += 1;
        }
    }

    pub fn can_use_timeout(&self, possession: HomeAway) -> bool {
        match possession {
            HomeAway::Home => self.home < 3,
            HomeAway::Away => self.away < 3,
        }
    }

    pub fn timeouts_remaining(&self, possession: HomeAway) -> u8 {
        match possession {
            HomeAway::Home => 3 - self.home,
            HomeAway::Away => 3 - self.away,
        }
    }
}

#[derive(Debug)]
pub struct GameState {
    pub score: Score,
    pub timeouts_used: TimeoutsUsed,
    pub clock: GameClock,
    pub play: PlayState,
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.clock, self.play)
    }
}

impl GameState {
    pub fn new(game_start: GameStart) -> GameState {
        let kicking_team = game_start.received_h1.flip();
        GameState {
            score: Score::new(),
            timeouts_used: TimeoutsUsed::new(),
            clock: GameClock::new(),
            play: PlayState::Kickoff(kicking_team),
        }
    }

    pub fn advance_clock(
        &mut self,
        seconds: u16,
        is_ot_ender: bool,
        is_runoff: bool,
    ) -> (bool, bool) {
        self.clock
            .advance(seconds, self.is_overtime_eligible(), is_ot_ender, is_runoff)
    }

    pub fn is_overtime_eligible(&self) -> bool {
        self.clock.quarter == Quarter::Fourth && self.score.is_game_tied()
    }

    pub fn charge_timeout(&mut self, team: HomeAway) {
        if !self.timeouts_used.can_use_timeout(team) {
            panic!("Team has used all of its timeouts for the half");
        }
        match team {
            HomeAway::Home => self.timeouts_used.home += 1,
            HomeAway::Away => self.timeouts_used.away += 1,
        }
    }

    pub fn update_play(&mut self, state: PlayState) {
        if self.clock.quarter == Quarter::Overtime {
            let prev_dp = self.play.down_possession();
            let new_dp: Option<HomeAway> = state.down_possession();
            if let Some(dp) = prev_dp {
                // if the drive is over, then the team that just gave up the ball
                // should be marked as having an overtime drive
                if prev_dp != new_dp {
                    match dp {
                        HomeAway::Home => self.clock.home_ot_drive = true,
                        HomeAway::Away => self.clock.away_ot_drive = true,
                    }
                }
            }
        }

        self.play = state;
    }
}
