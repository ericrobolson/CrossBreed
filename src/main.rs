// External crates
extern crate gl;
extern crate rmercury;
extern crate specs;
use specs::prelude::*;
extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};

// Internal crates
#[macro_use]
pub mod external_libs;

pub mod cb_graphics;
pub mod cb_input;
pub mod cb_simulation;
pub mod cb_system;
pub mod contexts;
use cb_system::{CbEvent, GameTick, PlayerId};

pub struct GameSim {}

impl GameSim {
    pub fn new() -> Self {
        return GameSim {};
    }
}

fn main() {
    // Init SDL
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Init OpenGL
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 2);

    let window = video_subsystem
        .window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap();

    let ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 2));

    // Init simulation data
    let mut game_tick: GameTick = 0;
    let mut events = vec![];
    let player_id: PlayerId = 1;
    let mut game_state = cb_simulation::GameState::new();

    let mut movement_context = cb_input::contexts::shooter_context::ShooterMovementContext::new();

    // Init specs
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().build();
    dispatcher.setup(&mut world);

    cb_simulation::assemblages::rts_assemblages::new_unit(&mut world);

    loop {
        // Get Events
        {
            let os_events = cb_input::get_os_inputs(&mut event_pump);

            movement_context = cb_input::contexts::shooter_context::get_shooter_movement_context(
                game_tick,
                &os_events,
                &movement_context,
            );

            let input_event = CbEvent {
                tick: game_tick + cb_system::FRAMEDELAY,
                value: movement_context,
            };

            //TODO: translate to game events// implement a 3 tick delay for networking purposes, à la GGPO
            //TODO: networking events
        }

        // Update simulation + pump events into simulation
        {
            game_state = cb_simulation::update_simulation(game_tick, &events, &game_state);

            // Clear events and increment game tick
            events.clear();
            game_tick += 1;
        }

        // Run gfx
        {
            //TODO
            // Random GFX example
            unsafe {
                gl::ClearColor(0.6, 0.0, 0.8, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            window.gl_swap_window();
        }
    }

    // Cleanup
}
