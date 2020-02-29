extern crate gl;
extern crate rmercury;
extern crate sdl2;
use sdl2::video::GLProfile;

mod open_gl_backend;
use open_gl_backend::OpenGlBackend;

pub mod mesh;

use crate::cb_simulation;
use cb_simulation::GameState;

pub struct CbCamera {
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub dir_x: f32,
    pub dir_y: f32,
    pub dir_z: f32,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub window_width: f32,
    pub window_height: f32,
}
impl CbCamera {
    fn new(window_width: f32, window_height: f32) -> Self {
        return Self {
            pos_x: -12.5000105,
            pos_y: 15.800025,
            pos_z: 2.0,
            dir_x: 0.0,
            dir_y: 0.0,
            dir_z: 0.0,
            cursor_x: 0.0,
            cursor_y: 0.0,
            window_width: window_width,
            window_height: window_height,
        };
    }
}

#[allow(dead_code)]
pub struct CbGfx {
    sdl_context: sdl2::Sdl,
    event_pump: sdl2::EventPump,
    window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext, // Need this to keep the OpenGL context active
    gl_backend: OpenGlBackend,
    camera: CbCamera,
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

        let window_width = 1920;
        let window_height = 1080;

        let window = video_subsystem
            .window("Window", window_width, window_height)
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
            camera: CbCamera::new(window_width as f32, window_height as f32),
        };
    }

    pub fn event_pump(&mut self) -> &mut sdl2::EventPump {
        return &mut self.event_pump;
    }

    pub fn camera(&mut self) -> &mut CbCamera {
        return &mut self.camera;
    }

    pub fn render(&mut self, game_state: &GameState, frame: usize) {
        let cursor = sdl2::mouse::MouseState::new(self.event_pump());

        self.camera.cursor_x = cursor.x() as f32;
        self.camera.cursor_y = cursor.y() as f32;

        OpenGlBackend::render(&mut self.gl_backend, &self.camera, game_state, frame);
        self.window.gl_swap_window();

        // sdl2::sys::SDL_WarpMouseInWindow(&mut self.window.as_ptr());
    }
}
