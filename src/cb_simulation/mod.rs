use crate::cb_system;
use cb_system::{CbEvent, GameTick};

use crate::cb_voxels;

pub mod assemblages;
pub mod components;

#[derive(Debug, Clone)]
pub struct GameState {
    pub current_tick: GameTick,
    pub voxel_chunk: cb_voxels::CbVoxelChunk,
}

impl GameState {
    pub fn new() -> Self {
        return GameState {
            current_tick: 0,
            voxel_chunk: cb_voxels::CbVoxelChunk::new(),
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
