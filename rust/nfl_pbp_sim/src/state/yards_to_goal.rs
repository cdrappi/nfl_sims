#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct YardsToGoal(pub u8);

impl YardsToGoal {
    pub fn new(value: u8) -> YardsToGoal {
        assert!(value >= 1 && value <= 99, "Value ({}) out of bounds", value);
        YardsToGoal(value)
    }

    pub fn flip(self) -> YardsToGoal {
        YardsToGoal(100 - self.0)
    }

    pub fn cannot_move_positive(&self) -> bool {
        self.0 <= 1
    }

    pub fn touchback() -> YardsToGoal {
        YardsToGoal::new(80)
    }

    pub fn kickoff_touchback() -> YardsToGoal {
        YardsToGoal::new(75)
    }

    pub fn cannot_move_negative(&self) -> bool {
        self.0 >= 99
    }

    pub fn forced_yards_sign_pos(&self) -> Option<bool> {
        if self.cannot_move_positive() {
            return Some(false);
        }
        if self.cannot_move_positive() {
            return Some(true);
        }
        None
    }
}

impl std::fmt::Display for YardsToGoal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Overloading for u8
impl std::ops::Add<u8> for YardsToGoal {
    type Output = YardsToGoal;

    fn add(self, rhs: u8) -> YardsToGoal {
        let result = self.0 + rhs;
        assert!(
            result <= 99,
            "Overflow detected: {} + {} = {}",
            self.0,
            rhs,
            result
        );
        YardsToGoal(result)
    }
}

impl std::ops::Sub<u8> for YardsToGoal {
    type Output = YardsToGoal;

    fn sub(self, rhs: u8) -> YardsToGoal {
        assert!(
            self.0 > rhs,
            "Underflow detected: {} - {} = {}",
            self.0,
            rhs,
            self.0 - rhs
        );
        YardsToGoal(self.0 - rhs)
    }
}

// Overloading for i8
impl std::ops::Add<i8> for YardsToGoal {
    type Output = YardsToGoal;

    fn add(self, rhs: i8) -> YardsToGoal {
        if rhs >= 0 {
            self + rhs as u8
        } else {
            self - (-rhs) as u8
        }
    }
}

impl std::ops::Sub<i8> for YardsToGoal {
    type Output = YardsToGoal;

    fn sub(self, rhs: i8) -> YardsToGoal {
        if rhs >= 0 {
            self - rhs as u8
        } else {
            self + (-rhs) as u8
        }
    }
}
