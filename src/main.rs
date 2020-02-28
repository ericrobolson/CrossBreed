// External crates
extern crate gl;
extern crate rmercury;
extern crate specs;
use specs::prelude::*;
extern crate sdl2;
use std::panic;

// Internal crates
#[macro_use]
pub mod cb_utility;

pub mod cb_graphics;
pub mod cb_input;
pub mod cb_math;
pub mod cb_simulation;
pub mod cb_system;
pub mod cb_voxels;
pub mod contexts;
use cb_system::{GameTick, PlayerId};

pub struct GameSim {}

impl GameSim {
    pub fn new() -> Self {
        return GameSim {};
    }
}

fn main() {
    //NOTE: this is only for dev use, to allow panics to be caught
    main_loop();
    loop {}
}

fn main_loop() {
    // Init gfx
    let mut gfx = cb_graphics::CbGfx::new();

    // Init simulation data
    let mut game_tick: GameTick = 0;
    let player_id: PlayerId = 1;
    let mut game_state = cb_simulation::GameState::new();

    let mut movement_context = cb_input::contexts::shooter_context::ShooterMovementContext::new();

    // Init specs
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().build();
    dispatcher.setup(&mut world);

    //TODO: fix up
    //   cb_simulation::assemblages::rts_assemblages::new_unit(&mut world);

    loop {
        // Get Events
        {
            let os_events = cb_input::get_os_inputs(gfx.event_pump());
            movement_context = cb_input::contexts::shooter_context::get_shooter_movement_context(
                game_tick,
                &os_events,
                &movement_context,
            );

            // Camera movement
            {
                let mut camera = gfx.camera();
                if movement_context.move_forward == cb_input::input_type::State::On {
                    camera.pos_x -= 0.1;
                } else if movement_context.move_backward == cb_input::input_type::State::On {
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

        // Update simulation
        {
            game_state.chunk_manager.mesh(game_tick as usize);

            // Increment game tick
            game_tick += 1;
        }

        // Run gfx
        gfx.render(&game_state);
    }

    // Cleanup
}
