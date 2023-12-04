pub const FG_SNAP_DISTANCE: u8 = 7;
pub const ENDZONE_LENGTH: u8 = 10;

pub fn fg_distance(yards_to_goal: u8) -> u8 {
    FG_SNAP_DISTANCE + ENDZONE_LENGTH + yards_to_goal
}
