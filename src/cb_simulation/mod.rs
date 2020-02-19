use crate::cb_system;
use cb_system::{CbEvent, GameTick};

pub mod assemblages;
pub mod components;

#[derive(Debug, Copy, Clone)]
pub struct GameState {
    pub current_tick: GameTick,
}

impl GameState {
    pub fn new() -> Self {
        return GameState { current_tick: 0 };
    }
}

pub fn update_simulation(
    current_tick: GameTick,
    events: &Vec<CbEvent<bool>>,
    state: &GameState,
) -> GameState {
    return state.clone();
}
