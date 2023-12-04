#[derive(Debug, PartialEq, Hash, Clone, Copy, serde::Serialize)]
pub enum Quarter {
    First,
    Second,
    Third,
    Fourth,
    Overtime,
    EndOfGame,
}

impl std::fmt::Display for Quarter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Quarter::First => write!(f, "Q1"),
            Quarter::Second => write!(f, "Q2"),
            Quarter::Third => write!(f, "Q3"),
            Quarter::Fourth => write!(f, "Q4"),
            Quarter::Overtime => write!(f, "OT"),
            Quarter::EndOfGame => write!(f, "End"),
        }
    }
}

impl Quarter {
    pub fn next(&self, is_ot_eligible: bool) -> Quarter {
        match self {
            Quarter::First => Quarter::Second,
            Quarter::Second => Quarter::Third,
            Quarter::Third => Quarter::Fourth,
            Quarter::Fourth => match is_ot_eligible {
                true => Quarter::Overtime,
                false => Quarter::EndOfGame,
            },
            Quarter::Overtime => Quarter::EndOfGame,
            Quarter::EndOfGame => panic!("Cannot advance past end of game"),
        }
    }
}

#[derive(Debug)]
pub struct GameClock {
    pub quarter: Quarter,
    pub seconds_remaining: u16,
    pub running: bool,
    pub home_ot_drive: bool,
    pub away_ot_drive: bool,
}

impl std::fmt::Display for GameClock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let minutes = self.seconds_remaining / 60;
        let seconds = self.seconds_remaining % 60;
        match self.quarter {
            Quarter::EndOfGame => write!(f, "End of game"),
            _ => write!(f, "{}: {:02}:{:02}", self.quarter, minutes, seconds),
        }
    }
}

impl GameClock {
    pub fn new() -> GameClock {
        GameClock {
            quarter: Quarter::First,
            seconds_remaining: 15 * 60,
            running: false,
            home_ot_drive: false,
            away_ot_drive: false,
        }
    }

    fn is_end_of_half(&self) -> bool {
        match self.quarter {
            Quarter::First => false,
            Quarter::Second => true,
            Quarter::Third => false,
            Quarter::Fourth => true,
            Quarter::Overtime => true,
            Quarter::EndOfGame => false,
        }
    }

    /// return true if clock stops
    pub fn advance(
        &mut self,
        seconds: u16,
        is_ot_eligible: bool,
        is_ot_ender: bool,
        is_runoff: bool,
    ) -> (bool, bool) {
        if is_runoff && self.is_end_of_half() {
            if (seconds > 120) && (seconds - self.seconds_remaining <= 120) {
                // 2 minute warning
                self.seconds_remaining = 120;
                return (true, false);
            }
        }

        match seconds >= self.seconds_remaining {
            false => {
                self.seconds_remaining -= seconds;
                if self.quarter == Quarter::Overtime {
                    let both_had_ball = self.home_ot_drive & self.away_ot_drive;
                    if !is_ot_eligible & (is_ot_ender || both_had_ball) {
                        self.quarter = Quarter::EndOfGame;
                        self.seconds_remaining = 0;
                        return (true, false);
                    }
                }
                return (false, false);
            }
            true => {
                self.quarter = self.quarter.next(is_ot_eligible);
                let reset_to_kickoff = match self.quarter {
                    Quarter::EndOfGame => {
                        self.seconds_remaining = 0;
                        false
                    }
                    Quarter::Overtime => {
                        self.seconds_remaining = 10 * 60;
                        true
                    }
                    _ => {
                        self.seconds_remaining = 15 * 60;
                        self.quarter == Quarter::Third
                    }
                };
                return (true, reset_to_kickoff);
            }
        }
    }

    pub fn game_minutes_remaining(&self) -> f32 {
        let this_quarter_seconds_remaining = (self.seconds_remaining + 1) as f32;
        let quarters_remaining = match self.quarter {
            Quarter::First => 3.0,
            Quarter::Second => 2.0,
            Quarter::Third => 1.0,
            _ => 0.0,
        };
        15.0 * quarters_remaining + this_quarter_seconds_remaining / 60.0
    }

    pub fn half_minutes_remaining(&self) -> f32 {
        let game_minutes_remaining = self.game_minutes_remaining();
        match self.quarter {
            Quarter::First | Quarter::Second => game_minutes_remaining - 30.0,
            _ => game_minutes_remaining,
        }
    }
}
