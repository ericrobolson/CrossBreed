use crate::cb_system;
use cb_system::{CbEvent, GameTick};

use crate::cb_voxels;

pub mod assemblages;
pub mod components;

#[derive(Debug, Copy, Clone)]
pub struct GameState {
    pub current_tick: GameTick,
    voxel: cb_voxels::CbVoxel,
}

impl GameState {
    pub fn new() -> Self {
        return GameState {
            current_tick: 0,
            voxel: cb_voxels::CbVoxel::new(),
        };
    }
}

pub fn update_simulation(
    current_tick: GameTick,
    events: &Vec<CbEvent<bool>>,
    state: &GameState,
) -> GameState {
    return state.clone();
}
