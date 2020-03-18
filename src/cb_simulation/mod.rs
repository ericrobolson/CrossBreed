// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

use crate::cb_system;
use cb_system::{CbEvent, GameTick};

use crate::cb_voxels;

use crate::cb_patterns;
use cb_patterns::presenter::{Presenter, SliderPresenter};

use crate::cb_graphics;

mod systems;
use systems::{editor_system::EditorSystem, voxel_editor_system};

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

pub mod world_builder;

// NOTE: GAME UNITS are 1 = 1mm, using i32s

#[derive(Default)]
pub struct CbSystemValues {
    pub world_inputs: CbWorldInputs,
    pub frame: usize,
}

impl CbSystemValues {
    pub fn new() -> Self {
        return Self {
            world_inputs: vec![],
            frame: 0,
        };
    }

    pub fn from(world_inputs: CbWorldInputs, frame: usize) -> Self {
        return Self {
            world_inputs: world_inputs,
            frame: frame,
        };
    }
}

pub type CbWorldInputs = std::vec::Vec<CbGameInput>;

pub struct CbSimulationInterface<'a, 'b> {
    game_state: CbGameState,
    world: World,
    in_editor_mode: bool,
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

        let editor_dispatcher = DispatcherBuilder::new()
            .with(EditorSystem, "editor system", &[])
            .build();

        let mut gfx_dispatcher = DispatcherBuilder::new().build();
        let mut world = world_builder::new(mode);

        return Self {
            game_state: CbGameState::new(),
            sim_dispatcher: dispatcher,
            editor_dispatcher: editor_dispatcher,
            gfx_dispatcher: gfx_dispatcher,
            world: world,
            gfx: cb_graphics::CbGfx::new(),
            in_editor_mode: true,
        };
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
        self.world.insert(CbSystemValues::from(
            inputs,
            self.game_state.current_tick as usize,
        ));

        if self.in_editor_mode {
            self.editor_dispatcher.dispatch(&mut self.world);
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
