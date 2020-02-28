use crate::cb_system;
use cb_system::{CbEvent, GameTick};

use crate::cb_voxels;

pub mod assemblages;
pub mod components;

#[derive(Debug)]
pub struct GameState {
    pub current_tick: GameTick,
    pub chunk_manager: cb_voxels::CbChunkManager,
}

impl GameState {
    pub fn new() -> Self {
        return GameState {
            current_tick: 0,
            chunk_manager: cb_voxels::CbChunkManager::new(),
        };
    }
    pub fn update_simulation(
        &mut self,
        current_tick: GameTick,
        events: &Vec<CbEvent<bool>>,
        state: &GameState,
    ) {
    }
}
