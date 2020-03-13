// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

use crate::cb_system;
use cb_system::{CbEvent, GameTick};

use crate::cb_voxels;

pub mod assemblages;
pub mod components;

extern crate rmercury;
use rmercury::{RMercuryGameInterface, RMercuryInput};

use crate::cb_input;
use cb_input::CbGameInput;

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
    fn advance_frame(&mut self, inputs: std::vec::Vec<CbGameInput>) {
        for input in inputs.iter() {
            for ctx in input.context_manager.get_contexts().iter() {
                if ctx.is_none() {
                    continue;
                }
                let ctx = ctx.unwrap();

                match ctx {
                    cb_input::contexts::CbInputContexts::ShooterContext {
                        networked: _,
                        jump: jump,
                        crouching: crouching,
                        running: running,
                        prone: prone,
                        move_forward: move_f,
                        move_backward: move_b,
                        move_left: move_l,
                        move_right: move_r,
                    } => {
                        // Camera movement - TODO: divorce this and put it in the simulation/abstract out the camera logic
                        {
                            if move_f == cb_input::input_type::State::On {
                                println!("moved forward");
                                self.game_state.camera_pos_x -= 1;
                            } else if move_b == cb_input::input_type::State::On {
                                self.game_state.camera_pos_x += 1;
                            }

                            if move_r == cb_input::input_type::State::On
                                && move_l != cb_input::input_type::State::On
                            {
                                self.game_state.camera_pos_z -= 1;
                            } else if move_l == cb_input::input_type::State::On
                                && move_r != cb_input::input_type::State::On
                            {
                                self.game_state.camera_pos_z += 1;
                            }

                            if crouching == cb_input::input_type::State::On {
                                self.game_state.camera_pos_y -= 1;
                            } else if running == cb_input::input_type::State::On {
                                self.game_state.camera_pos_y += 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn current_game_state(&self) -> CbGameState {
        return self.game_state.clone();
    }
}

#[derive(Debug, Clone)]
pub struct CbGameState {
    pub current_tick: GameTick,
    pub chunk_manager: cb_voxels::CbChunkManager,
    pub camera_pos_x: i32,
    pub camera_pos_y: i32,
    pub camera_pos_z: i32,
}

impl CbGameState {
    pub fn new() -> Self {
        return CbGameState {
            current_tick: 0,
            chunk_manager: cb_voxels::CbChunkManager::new(),
            camera_pos_x: 0,
            camera_pos_y: 0,
            camera_pos_z: 0,
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
