extern crate gl;
extern crate rmercury;
extern crate sdl2;
use sdl2::video::GLProfile;

mod open_gl_backend;
use open_gl_backend::OpenGlBackend;

use crate::cb_simulation;
use cb_simulation::GameState;

#[allow(dead_code)]
pub struct CbGfx {
    sdl_context: sdl2::Sdl,
    event_pump: sdl2::EventPump,
    window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext, // Need this to keep the OpenGL context active
    gl_backend: OpenGlBackend,
}

impl CbGfx {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

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

        let gl_backend = OpenGlBackend::new();

        return Self {
            sdl_context: sdl_context,
            event_pump: event_pump,
            window: window,
            gl_context: ctx,
            gl_backend: gl_backend,
        };
    }

    pub fn event_pump(&mut self) -> &mut sdl2::EventPump {
        return &mut self.event_pump;
    }

    pub fn render(&mut self, game_state: &GameState) {
        self.gl_backend.render(game_state);
        self.window.gl_swap_window();
    }
}
