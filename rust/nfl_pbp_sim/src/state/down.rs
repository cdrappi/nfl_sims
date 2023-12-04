use crate::state::yards_to_goal::YardsToGoal;
use crate::{game_loop::field_goals::fg_distance, sim::play_result::ClockStatus, start::HomeAway};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Down {
    First,
    Second,
    Third,
    Fourth,
}

impl std::fmt::Display for Down {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Down::First => write!(f, "1st"),
            Down::Second => write!(f, "2nd"),
            Down::Third => write!(f, "3rd"),
            Down::Fourth => write!(f, "4th"),
        }
    }
}

impl Down {
    /// true iff there's a change of possession
    pub fn next(&self) -> (Down, bool) {
        match self {
            Down::First => (Down::Second, false),
            Down::Second => (Down::Third, false),
            Down::Third => (Down::Fourth, false),
            Down::Fourth => (Down::First, true),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ToGo {
    Yards(u8),
    Goal,
}

impl ToGo {
    pub fn first_to_go(yards_to_goal: YardsToGoal) -> ToGo {
        match yards_to_goal.0 > 10 {
            true => ToGo::Yards(10),
            false => ToGo::Goal,
        }
    }
}

pub enum RushingSituation {
    OneYardToGo,
    GreenZone,
    Normal,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DownToGo {
    pub possession: HomeAway,
    pub down: Down,
    pub to_go: ToGo,
    pub yards_to_goal: YardsToGoal,
}

impl std::fmt::Display for DownToGo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.to_go {
            ToGo::Yards(yards) => {
                write!(f, "{} & {} from {}", self.down, yards, self.yards_to_goal)
            }
            ToGo::Goal => write!(f, "{} & Goal from {}", self.down, self.yards_to_goal),
        }
    }
}

impl DownToGo {
    pub fn first_and_ten(possession: HomeAway, yards_to_goal: YardsToGoal) -> DownToGo {
        DownToGo {
            possession,
            down: Down::First,
            to_go: ToGo::first_to_go(yards_to_goal),
            yards_to_goal,
        }
    }

    pub fn touchback(possession: HomeAway) -> DownToGo {
        DownToGo::first_and_ten(possession, YardsToGoal::new(75))
    }

    pub fn yards_for_first(&self) -> u8 {
        match self.to_go {
            ToGo::Yards(yards) => yards,
            ToGo::Goal => self.yards_to_goal.0,
        }
    }

    pub fn rushing_situation(&self) -> RushingSituation {
        match (self.yards_for_first(), self.yards_to_goal.0) {
            (0..=1, _) => RushingSituation::OneYardToGo,
            (_, 0..=10) => RushingSituation::GreenZone,
            _ => RushingSituation::Normal,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlayState {
    Down(DownToGo),
    // kicking team
    Kickoff(HomeAway),
    // team = kicking team
    PointAfterTouchdown(HomeAway),
    PuntAfterSafety(HomeAway),
}

impl std::fmt::Display for PlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PlayState::Down(down_to_go) => write!(f, "{}", down_to_go),
            PlayState::Kickoff(kicking_team) => write!(f, "Kickoff by {}", kicking_team),
            PlayState::PointAfterTouchdown(team) => write!(f, "PAT for {}", team),
            PlayState::PuntAfterSafety(team) => write!(f, "Punt after safety by {}", team),
        }
    }
}

impl PlayState {
    pub fn expect_downtogo(&self) -> DownToGo {
        match self {
            PlayState::Down(down_to_go) => return *down_to_go,
            // hack to model 2 point conversions as 4th and goal from the 2
            PlayState::PointAfterTouchdown(possession) => DownToGo {
                possession: *possession,
                down: Down::Fourth,
                to_go: ToGo::Goal,
                yards_to_goal: YardsToGoal(2),
            },
            _ => panic!("Expected down state. Found: {:?}", self),
        }
    }

    pub fn possession(&self) -> HomeAway {
        match self {
            PlayState::Down(down_to_go) => down_to_go.possession,
            PlayState::Kickoff(kicking_team) => *kicking_team,
            PlayState::PointAfterTouchdown(team) => *team,
            PlayState::PuntAfterSafety(team) => *team,
        }
    }

    pub fn down_possession(&self) -> Option<HomeAway> {
        match self {
            PlayState::Down(down_to_go) => Some(down_to_go.possession),
            PlayState::Kickoff(_) => None,
            PlayState::PointAfterTouchdown(_) => None,
            PlayState::PuntAfterSafety(_) => None,
        }
    }

    pub fn defense(&self) -> HomeAway {
        self.possession().flip()
    }

    pub fn kicking_team(&self) -> HomeAway {
        match self {
            PlayState::Kickoff(kicking_team) => *kicking_team,
            PlayState::PuntAfterSafety(kicking_team) => *kicking_team,
            PlayState::Down(dtg) => dtg.possession,
            PlayState::PointAfterTouchdown(kicking_team) => *kicking_team,
        }
    }

    pub fn returning_team(&self) -> HomeAway {
        self.kicking_team().flip()
    }

    pub fn field_goal_distance(&self) -> u8 {
        match self {
            PlayState::Down(down_to_go) => fg_distance(down_to_go.yards_to_goal.0),
            PlayState::PointAfterTouchdown(_) => fg_distance(15),
            _ => panic!("Cannot expect field goal distance in non-down state"),
        }
    }

    pub fn yards_for_safety(&self) -> i8 {
        // how many yards would we have to lose to get safetied? always negative
        -1 * self.expect_downtogo().yards_to_goal.flip().0 as i8
    }

    pub fn yards_for_touchdown(&self) -> i8 {
        self.expect_downtogo().yards_to_goal.0 as i8
    }

    pub fn safe_yards_for_first(&self) -> u8 {
        match self {
            PlayState::Down(down_to_go) => down_to_go.yards_for_first(),
            _ => 0,
        }
    }

    pub fn safe_yards_for_touchdown(&self) -> u8 {
        match self {
            PlayState::Down(down_to_go) => down_to_go.yards_to_goal.0,
            _ => 50,
        }
    }

    pub fn goal_to_go(&self) -> bool {
        match self {
            PlayState::Down(down_to_go) => match down_to_go.to_go {
                ToGo::Goal => true,
                ToGo::Yards(_) => false,
            },
            _ => false,
        }
    }

    pub fn adjusted_offensive_penalty_yards(&self, yards: u8) -> u8 {
        yards.min(self.expect_downtogo().yards_to_goal.flip().0 / 2)
    }

    pub fn offensive_penalty(&self, yards: u8, loss_of_down: bool) -> (PlayState, ClockStatus) {
        // TODO: intentional grounding in the endzone = safety
        // log::info!("ytg: {:?}", self.expect_downtogo().yards_to_goal);
        let penalty_yards = self.adjusted_offensive_penalty_yards(yards);
        let play_state = match (self, loss_of_down) {
            (PlayState::Down(down_to_go), true) => {
                let (next_down, turnover_on_downs) = down_to_go.down.next();
                match turnover_on_downs {
                    true => PlayState::Down(DownToGo::first_and_ten(
                        self.possession().flip(),
                        (down_to_go.yards_to_goal + penalty_yards).flip(),
                    )),
                    false => PlayState::Down(DownToGo {
                        possession: self.possession(),
                        down: next_down,
                        to_go: match down_to_go.to_go {
                            ToGo::Yards(yards_to_go) => ToGo::Yards(yards_to_go + penalty_yards),
                            ToGo::Goal => ToGo::Goal,
                        },
                        yards_to_goal: down_to_go.yards_to_goal + penalty_yards,
                    }),
                }
            }
            (PlayState::Down(down_to_go), false) => PlayState::Down(DownToGo {
                possession: down_to_go.possession,
                down: down_to_go.down,
                to_go: match down_to_go.to_go {
                    ToGo::Yards(yards_to_go) => ToGo::Yards(yards_to_go + penalty_yards),
                    ToGo::Goal => ToGo::Goal,
                },
                yards_to_goal: down_to_go.yards_to_goal + penalty_yards,
            }),
            _ => panic!("Cannot have offensive penalty in non-down state"),
        };
        (play_state, ClockStatus::Stopped)
    }

    pub fn adjusted_defensive_penalty_yards(
        &self,
        penalty_yards: u8,
        ignore_half_distance: bool,
    ) -> u8 {
        let yards_to_goal = self.expect_downtogo().yards_to_goal;
        match ignore_half_distance {
            true => penalty_yards,
            false => {
                let half_distance_to_goal = yards_to_goal.0 / 2;
                penalty_yards.min(half_distance_to_goal)
            }
        }
    }

    pub fn defensive_penalty(
        &self,
        penalty_yards: u8,
        automatic_first_down: bool,
        ignore_half_distance: bool,
    ) -> (PlayState, ClockStatus) {
        let DownToGo {
            possession,
            down,
            to_go,
            yards_to_goal,
        } = self.expect_downtogo();

        let adj_penalty_yards =
            self.adjusted_defensive_penalty_yards(penalty_yards, ignore_half_distance);
        let next_yards_to_goal = yards_to_goal - adj_penalty_yards;

        let (new_down, new_to_go) = match (automatic_first_down, to_go) {
            (true, ToGo::Goal) => (Down::First, ToGo::Goal),
            (false, ToGo::Goal) => (down, ToGo::Goal),
            (true, ToGo::Yards(prev_yards_to_go)) => (
                // automatic first down, but need to decide if we have goal to go
                Down::First,
                match (down, next_yards_to_goal.0) {
                    // we had a first down and non-goal already,
                    // so this shortens the yardage and it's still first down
                    (Down::First, _) => {
                        match adj_penalty_yards >= prev_yards_to_go {
                            // the penalty's yardage gives us a fresh first down
                            true => ToGo::first_to_go(next_yards_to_goal),
                            // we don't get a fresh first down, so only yardage shortens
                            false => ToGo::Yards(prev_yards_to_go - adj_penalty_yards),
                        }
                    }
                    // we received an automatic first down and we're within 10 yards of the goal, so goal to go
                    (_, 0..=10) => ToGo::Goal,
                    // we received an automatic first down, but we're not within 10 yards of the goal,
                    // so we now have first and 10
                    (_, _) => ToGo::Yards(10),
                },
            ),
            (false, ToGo::Yards(prev_yards_to_go)) => {
                match adj_penalty_yards >= prev_yards_to_go {
                    // the penalty's yardage gives us a first down
                    true => (Down::First, ToGo::first_to_go(next_yards_to_goal)),
                    // we don't get an automatic first down, so only yardage shortens
                    false => (down, ToGo::Yards(prev_yards_to_go - adj_penalty_yards)),
                }
            }
        };

        let new_down_to_go = DownToGo {
            possession,
            down: new_down,
            to_go: new_to_go,
            yards_to_goal: next_yards_to_goal,
        };

        (PlayState::Down(new_down_to_go), ClockStatus::Stopped)
    }

    pub fn intentional_grounding(&self, spot_of_foul: YardsToGoal) -> PlayState {
        if spot_of_foul.0 >= 100 {
            // NOTE: this isn't activated yet...
            // intentional grounding in your own endzone is a safety
            return PlayState::PuntAfterSafety(self.possession());
        }
        let down_to_go = self.expect_downtogo();
        let (next_down, turnover_on_downs) = down_to_go.down.next();
        let (possession, to_go, yards_to_goal) = match turnover_on_downs {
            true => (
                down_to_go.possession.flip(),
                ToGo::first_to_go(spot_of_foul.flip()),
                spot_of_foul.flip(),
            ),
            false => {
                let to_go = match down_to_go.to_go {
                    ToGo::Goal => ToGo::Goal,
                    ToGo::Yards(yards_to_go) => {
                        let yards_lost = down_to_go.yards_to_goal.0.saturating_sub(spot_of_foul.0);
                        ToGo::Yards(yards_to_go + yards_lost)
                    }
                };
                (down_to_go.possession, to_go, spot_of_foul)
            }
        };
        let next_down_to_go = DownToGo {
            possession,
            down: next_down,
            to_go,
            yards_to_goal,
        };
        PlayState::Down(next_down_to_go)
    }
}
