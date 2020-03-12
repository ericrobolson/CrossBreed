// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

// External crates
extern crate gl;
extern crate specs;

use specs::prelude::*;
extern crate sdl2;

extern crate rmercury;
use rmercury::{MercuryType, RMercuryBuilder, RMercuryExecutionResults};

// Internal crates
#[macro_use]
pub mod cb_utility;
pub mod cb_cmd_line;
pub mod cb_data_structures;
pub mod cb_graphics;
pub mod cb_input;
pub mod cb_math;
pub mod cb_simulation;
pub mod cb_system;
pub mod cb_voxels;

use cb_cmd_line::CbCmdMenu;
use cb_input::{CbGameInput, CbInputContextManager};
use cb_simulation::{CbGameState, CbSimulationInterface, CbSimulationModes};
use cb_system::PlayerId;

fn get_top_level_menu_choice(
    top_level_menu: CbCmdMenu,
    voxel_editor_mode: &str,
    simulation_mode: &str,
) -> String {
    top_level_menu.print();

    let mut mode_choice = "-1".to_string();
    let mut done = false;
    while !done {
        mode_choice = top_level_menu.get_menu_choice();

        if mode_choice == voxel_editor_mode {
            println!("Do Voxel Editor stuff");
            done = true;
        } else if mode_choice == simulation_mode {
            println!("Do Sim stuff");
            done = true;
        } else {
            println!("Invalid choice! Try again.");
        }
    }

    return mode_choice;
}

fn main() {
    let top_level_menu = CbCmdMenu::root(
        "CrossBreed.exe - Dev Kit",
        vec!["Voxel Model Editor", "Begin Simulation"],
    );

    const VOXEL_EDITOR_MODE: &str = "1";
    const SIMULATION_MODE: &str = "2";

    let mode_choice = get_top_level_menu_choice(top_level_menu, VOXEL_EDITOR_MODE, SIMULATION_MODE);

    // Init gfx
    let mut gfx = cb_graphics::CbGfx::new();

    // Init RMercury
    let mut game_interface;
    let mut builder;
    {
        if mode_choice == VOXEL_EDITOR_MODE {
            game_interface = CbSimulationInterface::new(CbSimulationModes::VoxelEditor);
        } else {
            game_interface = CbSimulationInterface::new(CbSimulationModes::Simulation);
        }

        builder = RMercuryBuilder::<CbSimulationInterface, CbGameInput, CbGameState>::new(
            &mut game_interface,
        )
        .with_type(MercuryType::Peer2Peer);
    }

    let mut r_mercury = builder.build();

    // Init simulation data
    let player_id: PlayerId = 1;
    let mut game_state = r_mercury.get_game_state();

    let mut input_context_manager = CbInputContextManager::new();

    let mut movement_context;

    // Init specs
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().build();
    dispatcher.setup(&mut world);

    loop {
        // Update simulation
        {
            // Get Local Inputs
            {
                if r_mercury.ready_to_run() {
                    let os_events = input_context_manager
                        .get_os_inputs(r_mercury.get_current_tick(), gfx.event_pump());

                    movement_context = input_context_manager.get_shooter_movement_context();

                    // Camera movement - TODO: divorce this and put it in the simulation/abstract out the camera logic
                    {
                        let mut camera = gfx.camera();
                        if movement_context.move_forward == cb_input::input_type::State::On {
                            camera.pos_x -= 0.1;
                        } else if movement_context.move_backward == cb_input::input_type::State::On
                        {
                            camera.pos_x += 0.1;
                        }

                        if movement_context.move_right == cb_input::input_type::State::On
                            && movement_context.move_left != cb_input::input_type::State::On
                        {
                            camera.pos_z -= 0.1;
                        } else if movement_context.move_left == cb_input::input_type::State::On
                            && movement_context.move_right != cb_input::input_type::State::On
                        {
                            camera.pos_z += 0.1;
                        }

                        if movement_context.crouching == cb_input::input_type::State::On {
                            camera.pos_y -= 0.1;
                        } else if movement_context.running == cb_input::input_type::State::On {
                            camera.pos_y += 0.1;
                        }
                    }
                }

                let mut local_input = input_context_manager.get_rmercury_inputs();
                r_mercury.add_local_input(&mut local_input);
            }

            let result = r_mercury.execute(); // Always execute, as even if the sim is not run the networking protocols are
            if result == RMercuryExecutionResults::Executed {
                // Update game state
                game_state = r_mercury.get_game_state();
            }
        }

        // Run gfx
        gfx.render(&game_state, r_mercury.get_current_tick());
    }
}
