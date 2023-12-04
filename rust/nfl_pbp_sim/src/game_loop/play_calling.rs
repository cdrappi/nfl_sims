use crate::{
    models::playcall::PlaycallModel,
    sim::{play_result::PlaycallResult, GameSim},
    state::down::PlayState,
};

pub enum PlayType {
    Down(PlaycallResult),
    Kickoff,
    PuntAfterSafety,
    PointAfterTouchdown,
}

impl PlayType {
    pub fn consumes_clock(&self) -> bool {
        match &self {
            PlayType::PointAfterTouchdown => false,
            _ => true,
        }
    }
}

pub fn choose_playcall(sim: &GameSim) -> PlayType {
    match sim.game_state.play {
        PlayState::Down(_) => PlayType::Down(PlaycallModel::sample_playcall(sim)),
        PlayState::Kickoff(_) => PlayType::Kickoff,
        PlayState::PointAfterTouchdown(_) => PlayType::PointAfterTouchdown,
        PlayState::PuntAfterSafety(_) => PlayType::PuntAfterSafety,
    }
}
