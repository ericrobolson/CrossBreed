// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

// External crates

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
pub mod cb_patterns;
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

    // Init RMercury
    let mut input_context_manager = CbInputContextManager::new();

    // Init game interface
    let mut game_interface;
    let mut builder;
    {
        if mode_choice == VOXEL_EDITOR_MODE {
            game_interface = CbSimulationInterface::new(CbSimulationModes::VoxelEditor);
            game_interface.gfx.reset_cursor = false;

            input_context_manager.add_context(cb_input::contexts::VOXEL_EDITOR_CONTEXT_ID);
        } else {
            game_interface = CbSimulationInterface::new(CbSimulationModes::Simulation);
            game_interface.gfx.reset_cursor = true;

            input_context_manager.add_context(cb_input::contexts::SHOOTER_CONTEXT_ID);
        }

        builder = RMercuryBuilder::<CbSimulationInterface, CbGameInput, CbGameState>::new(
            &mut game_interface,
        )
        .with_type(MercuryType::Peer2Peer);
    }

    let mut r_mercury = builder.build();

    loop {
        // Update simulation
        {
            // Get Local Inputs
            if r_mercury.ready_to_run() {
                let current_frame_inputs;
                {
                    current_frame_inputs = input_context_manager
                        .read_os_inputs(r_mercury.get_game_interface_mut().gfx.event_pump_mut());

                    if current_frame_inputs.iter().any(|e| {
                        match e {
                            sdl2::event::Event::KeyDown {
                                timestamp: _,
                                window_id: _,
                                keycode: keycode,
                                scancode: _,
                                keymod: _,
                                repeat: _,
                            } => {
                                if *keycode == Some(sdl2::keyboard::Keycode::Backquote) {
                                    return true;
                                }
                            }
                            _ => {}
                        }
                        return false;
                    }) {
                        r_mercury.get_game_interface_mut().toggle_editor_mode();
                    }
                }

                let hardware_interface = cb_graphics::Sdl2HardwareInterface::from_gfx(
                    &r_mercury.get_game_interface_mut().gfx,
                    &current_frame_inputs,
                );

                let local_input = input_context_manager.get_rmercury_inputs(&hardware_interface);
                r_mercury.add_local_input(&mut vec![local_input]);

                let center_mouse = r_mercury.get_game_interface_mut().gfx.reset_cursor;
                if center_mouse {
                    r_mercury.get_game_interface_mut().gfx.center_mouse();
                }
            }

            let result = r_mercury.execute(); // Always execute, as even if the sim is not run the networking protocols are
        }

        // Run gfx
        {
            r_mercury.get_game_interface_mut().render();
        }
    }
}
