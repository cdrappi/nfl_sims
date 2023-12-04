use crate::util::stats::random_bool;
use serde::Deserialize;

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash, Deserialize)]
pub enum HomeAway {
    Home,
    Away,
}

impl std::fmt::Display for HomeAway {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HomeAway::Home => write!(f, "Home"),
            HomeAway::Away => write!(f, "Away"),
        }
    }
}

impl HomeAway {
    pub fn home_if_true(condition: bool) -> HomeAway {
        match condition {
            true => HomeAway::Home,
            false => HomeAway::Away,
        }
    }

    pub fn flip(&self) -> HomeAway {
        match self {
            HomeAway::Home => HomeAway::Away,
            HomeAway::Away => HomeAway::Home,
        }
    }
}

pub struct GameStart {
    pub coin_toss_heads: bool,
    pub received_h1: HomeAway,
    pub home_q1_goal1: bool,
}

impl GameStart {
    pub fn new() -> GameStart {
        let coin_toss_heads = random_bool(0.5);
        let received_h1 = HomeAway::home_if_true(coin_toss_heads);
        GameStart {
            coin_toss_heads,
            received_h1,
            home_q1_goal1: coin_toss_heads,
        }
    }

    pub fn receive_second_half(&self) -> HomeAway {
        self.received_h1.flip()
    }
}
