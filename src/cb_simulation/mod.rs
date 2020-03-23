// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

use crate::cb_system;
use cb_system::{CbEvent, GameTick};

use crate::cb_voxels;

use crate::cb_graphics;

mod systems;
use systems::{actor_input_system, editor_system::EditorSystem, ik_system, voxel_editor_system};

pub mod assemblages;
pub mod components;
use components::{
    gfx_components::CameraComponent, physics_components::TransformComponent,
    physics_components::VelocityComponent, voxel_components::VoxelComponent,
};

extern crate specs;
use specs::prelude::*;

extern crate rmercury;
use rmercury::{RMercuryGameInterface, RMercuryInput};

use crate::cb_input;
use cb_input::CbGameInput;

use crate::cb_menu;
use cb_menu::{menu_events, Form};

pub mod world_builder;

// NOTE: GAME UNITS are 1 = 1mm, using i32s

#[derive(Default)]
pub struct CbSystemValues {
    pub events: Vec<(menu_events::EventId, menu_events::Events)>,
    pub world_inputs: CbWorldInputs,
    pub databinding_changes: Vec<(menu_events::EventId, menu_events::Events)>,
    current_player_id: usize,
    pub frame: usize,
}

impl CbSystemValues {
    pub fn new() -> Self {
        return Self {
            events: vec![],
            world_inputs: vec![],
            frame: 0,
            current_player_id: 0,
            databinding_changes: vec![],
        };
    }

    pub fn get_current_player_id(&self) -> usize {
        return self.current_player_id;
    }

    pub fn from(world_inputs: CbWorldInputs, current_player_id: usize, frame: usize) -> Self {
        return Self {
            current_player_id: current_player_id,
            events: vec![],
            world_inputs: world_inputs,
            frame: frame,
            databinding_changes: vec![],
        };
    }
}

pub type CbWorldInputs = std::vec::Vec<CbGameInput>;

pub struct CbSimulationInterface<'a, 'b> {
    game_state: CbGameState,
    world: World,
    in_editor_mode: bool,
    current_player_id: usize,
    sim_dispatcher: specs::Dispatcher<'a, 'b>,
    editor_dispatcher: specs::Dispatcher<'a, 'b>,
    gfx_dispatcher: specs::Dispatcher<'a, 'b>,
    pub gfx: cb_graphics::CbGfx,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CbSimulationModes {
    VoxelEditor,
    Simulation,
}

impl<'a, 'b> CbSimulationInterface<'a, 'b> {
    /// Create a new CbSimulation
    pub fn new(mode: CbSimulationModes) -> Self {
        let dispatcher;
        {
            let mut builder = DispatcherBuilder::new();

            if mode == CbSimulationModes::VoxelEditor {
                builder = builder.with(
                    voxel_editor_system::VoxelEditorSystem,
                    "voxel editor system",
                    &[],
                );
            }

            dispatcher = builder.build();
        }

        let game_system_dispatcher;
        {
            game_system_dispatcher = DispatcherBuilder::new()
                .with(actor_input_system::ActorInputSystem, "actor input", &[])
                .with_barrier()
                .with(ik_system::IkSystem, "inverse kinematics", &[])
                .build();
        }

        let editor_dispatcher = DispatcherBuilder::new()
            .with(EditorSystem, "editor system", &[])
            .build();

        let mut gfx_dispatcher = DispatcherBuilder::new().build();
        let mut world = world_builder::new(mode);

        return Self {
            current_player_id: 0,
            game_state: CbGameState::new(),
            sim_dispatcher: game_system_dispatcher,
            editor_dispatcher: editor_dispatcher,
            gfx_dispatcher: gfx_dispatcher,
            world: world,
            gfx: cb_graphics::CbGfx::new(),
            in_editor_mode: true,
        };
    }

    pub fn set_local_player_id(&mut self, current_player_id: usize) {
        self.current_player_id = current_player_id;
    }

    pub fn get_local_player_id(&self) -> usize {
        return self.current_player_id;
    }

    pub fn toggle_editor_mode(&mut self) {
        self.in_editor_mode = !self.in_editor_mode;
        self.gfx.toggle_editor_window();
        println!("Editor Mode: {}", self.in_editor_mode);
    }

    /// Render the simulation; only updates the graphics systems
    pub fn render(&mut self) {
        self.gfx.render(
            &self.game_state,
            &self.world,
            self.game_state.current_tick as usize,
        );

        self.gfx_dispatcher.dispatch(&self.world);
    }
}

impl<'a, 'b> RMercuryGameInterface<CbGameState, CbGameInput> for CbSimulationInterface<'a, 'b> {
    fn load_game_state(&mut self, _: CbGameState) {
        //unimplemented!()
        //NOTE: use spec's world serialization/deserialization
    }
    fn log_game_state(&self) -> std::string::String {
        //unimplemented!()
        return "hello world!".to_string();
    }
    fn advance_frame(&mut self, inputs: std::vec::Vec<CbGameInput>) {
        let mut sys_values = CbSystemValues::from(
            inputs,
            self.get_local_player_id(),
            self.game_state.current_tick as usize,
        );
        sys_values.events = self.gfx.editor_gui_env.get_events();

        self.world.insert(sys_values);

        if self.in_editor_mode {
            self.gfx.build_menus(&mut self.world);

            self.editor_dispatcher.dispatch(&mut self.world);

            // Handle databinding changes
            let updated_sys_values: (Read<CbSystemValues>) = self.world.system_data();

            let databinding_changes = updated_sys_values.databinding_changes.clone();

            self.gfx.handle_databinding_changes(&databinding_changes);
        }
        //else
        {
            // Execute simulation systems

            {
                // Execute world systems + maintain it
                self.sim_dispatcher.dispatch(&mut self.world);
                self.world.maintain();
            }

            self.game_state.current_tick += 1;
        }
    }

    fn current_game_state(&self) -> CbGameState {
        //NOTE: use spec's world serialization/deserialization

        return self.game_state.clone();
    }
}

#[derive(Debug, Clone)]
pub struct CbGameState {
    pub current_tick: GameTick,
}

impl CbGameState {
    pub fn new() -> Self {
        return CbGameState { current_tick: 0 };
    }
}
