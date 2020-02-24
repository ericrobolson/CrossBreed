pub mod render_gl;
extern crate gl;
extern crate rmercury;
extern crate specs;
use specs::prelude::*;
extern crate sdl2;
use sdl2::video::GLProfile;
use std::ffi::CString;

pub struct CbGfx {
    sdl_context: sdl2::Sdl,
    event_pump: sdl2::EventPump,
    window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
}

impl CbGfx {
    pub fn new() -> Self {
        let mut sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        // Init OpenGL
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 2);

        let mut window = video_subsystem
            .window("Window", 800, 600)
            .opengl()
            .build()
            .unwrap();

        let ctx = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (3, 2));

        return Self {
            sdl_context: sdl_context,
            event_pump: event_pump,
            window: window,
            gl_context: ctx,
        };
    }

    pub fn event_pump(&mut self) -> &mut sdl2::EventPump {
        return &mut self.event_pump;
    }

    pub fn context(&mut self) -> &mut sdl2::Sdl {
        return &mut self.sdl_context;
    }

    pub fn window(&mut self) -> &mut sdl2::video::Window {
        return &mut self.window;
    }
}
