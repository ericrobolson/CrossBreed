extern crate gl;
extern crate sdl2;

pub mod contexts;
pub mod simulation;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

fn main() {
    let sdl_context = sdl2::init().unwrap();
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

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut context_manager = contexts::ContextManager::new();
    // Note: This loop uses the concept of GGPO: 3 frame input delay, and rollback netplay

    'running: loop {
        // TODO: get network inputs
        // TODO: get inputs; sync up networked inputs with local inputs and signal a sim rerun.
        // TODO: Send network information. Only send inputs within a certain context.
        // This means that for example, you don't need to network anything for a player if they are in the options menu.
        // You instead can not send anything, as in the context of the game, nothing is occuring. Easy networking win there.
        // To go even further, you can not rerun those simulations. Only the game simulation needs to be able to rerun in under a half a frame.

        // Update the state
        // TODO: pass in inputs
        let inputs = vec![];

        context_manager = context_manager.tick(inputs);

        // Random GFX example
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();

        if context_manager.has_active_context() == false {
            //TODO: exit game
            break;
        }

        //TODO: figure out
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
