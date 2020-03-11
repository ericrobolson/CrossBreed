// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

use crate::cb_system;
use cb_system::{CbEvent, GameTick};

use crate::cb_voxels;

pub mod assemblages;
pub mod components;

extern crate rmercury;
use rmercury::{RMercuryGameInterface, RMercuryInput};

#[derive(Debug, Copy, Clone)]
pub struct CbGameInput {}

impl RMercuryInput for CbGameInput {
    fn get_player_id(&self) -> usize {
        //unimplemented!()
        return 0;
    }
    fn set_player_id(&mut self, _: usize) {
        //unimplemented!()
    }
    fn to_bits(&self) -> std::vec::Vec<u8> {
        return vec![];
    }
    fn from_bits(_: std::vec::Vec<u8>) -> Self {
        //unimplemented!()
        return Self {};
    }
}

#[derive(Debug, Clone)]
pub struct CbSimulationInterface {
    game_state: CbGameState,
    mode: CbSimulationModes,
}

#[derive(Debug, Copy, Clone)]
pub enum CbSimulationModes {
    VoxelEditor,
    Simulation,
}

impl CbSimulationInterface {
    pub fn new(mode: CbSimulationModes) -> Self {
        return Self {
            game_state: CbGameState::new(),
            mode: mode,
        };
    }
}

impl RMercuryGameInterface<CbGameState, CbGameInput> for CbSimulationInterface {
    fn load_game_state(&mut self, _: CbGameState) {
        //unimplemented!()
    }
    fn log_game_state(&self) -> std::string::String {
        //unimplemented!()
        return "hello world!".to_string();
    }
    fn advance_frame(&mut self, _: std::vec::Vec<CbGameInput>) {
        //unimplemented!()
    }
    fn current_game_state(&self) -> CbGameState {
        return self.game_state.clone();
    }
}

#[derive(Debug, Clone)]
pub struct CbGameState {
    pub current_tick: GameTick,
    pub chunk_manager: cb_voxels::CbChunkManager,
}

impl CbGameState {
    pub fn new() -> Self {
        return CbGameState {
            current_tick: 0,
            chunk_manager: cb_voxels::CbChunkManager::new(),
        };
    }
    pub fn update_simulation(
        &mut self,
        current_tick: GameTick,
        events: &Vec<CbEvent<bool>>,
        state: &CbGameState,
    ) {
    }
}
