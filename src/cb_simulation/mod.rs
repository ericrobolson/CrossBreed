use crate::cb_system;
use cb_system::{CbEvent, GameTick};

use crate::cb_voxels;

pub mod assemblages;
pub mod components;

extern crate rmercury;
use rmercury::{RMercuryGameInterface, RMercuryInput};

#[derive(Debug, Copy, Clone)]
pub struct CbGameState {}

#[derive(Debug, Copy, Clone)]
pub struct CbGameInterface {
    game_state: CbGameState,
}

#[derive(Debug, Copy, Clone)]
pub struct CbGameInput {}

impl RMercuryInput for CbGameInput {
    fn get_player_id(&self) -> usize {
        //unimplemented!()
        return 0;
    }
    fn set_player_id(&mut self, _: usize) {
        unimplemented!()
    }
    fn to_bits(&self) -> std::vec::Vec<u8> {
        unimplemented!()
    }
    fn from_bits(_: std::vec::Vec<u8>) -> Self {
        unimplemented!()
    }
}

impl CbGameInterface {
    pub fn new() -> Self {
        return Self {
            game_state: CbGameState {},
        };
    }
}

impl RMercuryGameInterface<CbGameState, CbGameInput> for CbGameInterface {
    fn load_game_state(&mut self, _: CbGameState) {
        unimplemented!()
    }
    fn log_game_state(&self) -> std::string::String {
        unimplemented!()
    }
    fn advance_frame(&mut self, _: std::vec::Vec<CbGameInput>) {
        unimplemented!()
    }
    fn current_game_state(&self) -> CbGameState {
        return self.game_state;
    }
}

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
