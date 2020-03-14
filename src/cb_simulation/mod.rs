// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

use crate::cb_system;
use cb_system::{CbEvent, GameTick};

use crate::cb_voxels;

use crate::cb_graphics;

pub mod assemblages;
pub mod components;

extern crate specs;
use specs::prelude::*;

extern crate rmercury;
use rmercury::{RMercuryGameInterface, RMercuryInput};

use crate::cb_input;
use cb_input::CbGameInput;

pub mod world_builder;

pub struct CbSimulationInterface<'a, 'b> {
    game_state: CbGameState,
    mode: CbSimulationModes,
    world: World,
    sim_dispatcher: specs::Dispatcher<'a, 'b>,
    gfx_dispatcher: specs::Dispatcher<'a, 'b>,
    pub gfx: cb_graphics::CbGfx,
}

#[derive(Debug, Copy, Clone)]
pub enum CbSimulationModes {
    VoxelEditor,
    Simulation,
}

impl<'a, 'b> CbSimulationInterface<'a, 'b> {
    /// Create a new CbSimulation
    pub fn new(mode: CbSimulationModes) -> Self {
        let mut dispatcher = DispatcherBuilder::new().build();
        let mut gfx_dispatcher = DispatcherBuilder::new().build();
        let mut world = world_builder::new();

        return Self {
            game_state: CbGameState::new(),
            mode: mode,
            sim_dispatcher: dispatcher,
            gfx_dispatcher: gfx_dispatcher,
            world: world,
            gfx: cb_graphics::CbGfx::new(),
        };
    }

    /// Render the simulation
    pub fn render(&mut self) {
        self.gfx
            .render(&self.game_state, self.game_state.current_tick as usize);
        self.gfx_dispatcher.dispatch(&self.world);
    }
}

fn apply_input_contexts(state: &mut CbSimulationInterface, inputs: std::vec::Vec<CbGameInput>) {
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
                    look_x: look_x,
                    look_y: look_y,
                } => {
                    // Camera movement - TODO: abstract out the camera logic
                    {
                        const move_speed: i32 = 5;

                        if move_f == cb_input::input_type::State::On {
                            state.game_state.camera_pos_x -= move_speed;
                        } else if move_b == cb_input::input_type::State::On {
                            state.game_state.camera_pos_x += move_speed;
                        }

                        if move_r == cb_input::input_type::State::On
                            && move_l != cb_input::input_type::State::On
                        {
                            state.game_state.camera_pos_z -= move_speed;
                        } else if move_l == cb_input::input_type::State::On
                            && move_r != cb_input::input_type::State::On
                        {
                            state.game_state.camera_pos_z += move_speed;
                        }

                        if crouching == cb_input::input_type::State::On {
                            state.game_state.camera_pos_y -= move_speed;
                        } else if running == cb_input::input_type::State::On {
                            state.game_state.camera_pos_y += move_speed;
                        }

                        // NOTE: This needs to be modified to set the camera pitch + yaw + roll
                        /*
                        self.game_state.mouse_look_x = look_x.value;
                        self.game_state.mouse_look_y = look_y.value;
                        */
                    }
                }
                cb_input::contexts::CbInputContexts::VoxelEditorContext {
                    networked: _,
                    cursor_x: cursor_x,
                    cursor_y: cursor_y,
                    toggle_orthographic_view: toggle_orthographic_view,
                    front_view: front_view,
                    top_view: top_view,
                    right_view: right_view,
                    left_view: left_view,
                    rotate_camera_up: rotate_camera_up,
                    rotate_camera_down: rotate_camera_down,
                    rotate_camera_left: rotate_camera_left,
                    rotate_camera_right: rotate_camera_right,
                    add_voxel: add_voxel,
                    remove_voxel: remove_voxel,
                } => {
                    const MAX_EDITOR_X: i32 = 2500;
                    const MAX_EDITOR_Y: i32 = 2500;
                    const MAX_EDITOR_Z: i32 = 2500;

                    // Camera Rotations
                    {
                        const MAX_ROTATION: i32 = 100;

                        if rotate_camera_up == cb_input::input_type::Press::Pressed {
                            println!("pressed up");
                            state.game_state.camera_target_y += MAX_ROTATION;
                        } else if rotate_camera_down == cb_input::input_type::Press::Pressed {
                            state.game_state.camera_target_y -= MAX_ROTATION;
                        }
                    }

                    // Camera Positions
                    {
                        if front_view == cb_input::input_type::Press::Pressed {
                            state.game_state.camera_pos_x = MAX_EDITOR_X / 2;
                            state.game_state.camera_pos_y = MAX_EDITOR_Y / 2;
                            state.game_state.camera_pos_z = -MAX_EDITOR_Z;

                            state.game_state.camera_target_x = MAX_EDITOR_X / 2;
                            state.game_state.camera_target_y = MAX_EDITOR_Y / 2;
                            state.game_state.camera_target_z = MAX_EDITOR_Z / 2;
                        } else if left_view == cb_input::input_type::Press::Pressed {
                            state.game_state.camera_pos_x = MAX_EDITOR_X;
                            state.game_state.camera_pos_y = MAX_EDITOR_Y / 2;
                            state.game_state.camera_pos_z = MAX_EDITOR_Z / 2;

                            state.game_state.camera_target_x = MAX_EDITOR_X / 2;
                            state.game_state.camera_target_y = MAX_EDITOR_Y / 2;
                            state.game_state.camera_target_z = MAX_EDITOR_Z / 2;
                        } else if right_view == cb_input::input_type::Press::Pressed {
                            state.game_state.camera_pos_x = -MAX_EDITOR_X;
                            state.game_state.camera_pos_y = MAX_EDITOR_Y / 2;
                            state.game_state.camera_pos_z = MAX_EDITOR_Z / 2;

                            state.game_state.camera_target_x = MAX_EDITOR_X / 2;
                            state.game_state.camera_target_y = MAX_EDITOR_Y / 2;
                            state.game_state.camera_target_z = MAX_EDITOR_Z / 2;
                        } else if top_view == cb_input::input_type::Press::Pressed {
                            state.game_state.camera_pos_x = MAX_EDITOR_X / 2;
                            state.game_state.camera_pos_y = MAX_EDITOR_Y / 2;
                            state.game_state.camera_pos_z = MAX_EDITOR_Z;

                            state.game_state.camera_target_x = MAX_EDITOR_X / 2;
                            state.game_state.camera_target_y = MAX_EDITOR_Y / 2;
                            state.game_state.camera_target_z = MAX_EDITOR_Z / 2;
                        } else if toggle_orthographic_view == cb_input::input_type::Press::Pressed {
                            state.game_state.camera_orthographic_view =
                                !state.game_state.camera_orthographic_view;
                        }

                        // TODO: add/remove voxels
                        if add_voxel == cb_input::input_type::Press::Pressed {
                            state
                                .game_state
                                .chunk_manager
                                .randomize(state.game_state.current_tick as usize);
                        } else if remove_voxel == cb_input::input_type::Press::Pressed {
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

impl<'a, 'b> RMercuryGameInterface<CbGameState, CbGameInput> for CbSimulationInterface<'a, 'b> {
    fn load_game_state(&mut self, _: CbGameState) {
        //unimplemented!()
    }
    fn log_game_state(&self) -> std::string::String {
        //unimplemented!()
        return "hello world!".to_string();
    }
    fn advance_frame(&mut self, inputs: std::vec::Vec<CbGameInput>) {
        apply_input_contexts(self, inputs);
        // Execute world systems + maintain it
        {
            self.sim_dispatcher.dispatch(&mut self.world);
            self.world.maintain();
        }
        self.game_state.current_tick += 1;

        //NOTE: may need a camera for each player? maybe not
    }

    fn current_game_state(&self) -> CbGameState {
        return self.game_state.clone();
    }
}

#[derive(Debug, Clone)]
pub struct CbGameState {
    pub current_tick: GameTick,
    pub chunk_manager: cb_voxels::CbChunkManager,

    pub camera_orthographic_view: bool,

    pub camera_pos_x: i32,
    pub camera_pos_y: i32,
    pub camera_pos_z: i32,

    pub camera_target_x: i32,
    pub camera_target_y: i32,
    pub camera_target_z: i32,

    pub camera_pitch: i32,
    pub camera_yaw: i32,
    pub camera_roll: i32,
}

impl CbGameState {
    pub fn new() -> Self {
        return CbGameState {
            camera_orthographic_view: false,
            current_tick: 0,
            chunk_manager: cb_voxels::CbChunkManager::new(),
            camera_pos_x: 0,
            camera_pos_y: 0,
            camera_pos_z: 0,

            camera_target_x: 0,
            camera_target_y: 0,
            camera_target_z: 0,
            camera_pitch: 0,
            camera_yaw: 0,
            camera_roll: 0,
        };
    }
}
