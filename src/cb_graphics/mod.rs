// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate specs;
use specs::prelude::*;

extern crate gl;
extern crate sdl2;
use sdl2::video::GLProfile;

pub mod open_gl_backend;
use open_gl_backend::OpenGlBackend;

pub mod cb_collada;
pub mod mesh;
pub mod sprites;

use crate::cb_patterns;
use cb_patterns::presenter::{Presenter, SliderPresenter, View};

use crate::cb_simulation;
use cb_simulation::CbGameState;

/// A class that is meant to congregate all platform specific code that interacts with the logic layers, so that it can easily be refactored/swapped out later on.
/// Eventually will switch over to a trait based system when cross-platform begins.
pub struct Sdl2HardwareInterface<'a> {
    pub events: &'a Vec<sdl2::event::Event>,
    pub pump: &'a sdl2::EventPump,
    pub window_width: i32,
    pub window_height: i32,
    pub reset_cursor: bool,
}

impl<'a> Sdl2HardwareInterface<'a> {
    pub fn from_gfx(gfx: &'a CbGfx, events: &'a Vec<sdl2::event::Event>) -> Self {
        return Self {
            events: events,
            pump: &gfx.event_pump,
            window_width: gfx.window_width,
            window_height: gfx.window_height,
            reset_cursor: gfx.reset_cursor,
        };
    }
}

pub struct CbCamera {
    pub orthographic_view: bool,

    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,

    pub target_x: f32,
    pub target_y: f32,
    pub target_z: f32,

    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,

    pub window_width: f32,
    pub window_height: f32,
}
impl CbCamera {
    fn new(window_width: f32, window_height: f32) -> Self {
        return Self {
            orthographic_view: false,

            pos_x: -12.5000105,
            pos_y: 15.800025,
            pos_z: 2.0,
            target_x: 0.0,
            target_y: 0.0,
            target_z: 0.0,

            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            window_width: window_width,
            window_height: window_height,
        };
    }
}

#[allow(dead_code)]
pub struct CbGfx {
    window_width: i32,
    window_height: i32,
    sdl_context: sdl2::Sdl,
    event_pump: sdl2::EventPump,
    window: sdl2::video::Window,
    video_subsystem: sdl2::VideoSubsystem,
    editor_window: sdl2::render::Canvas<sdl2::video::Window>,
    editor_visible: bool,

    gl_context: sdl2::video::GLContext, // Need this to keep the OpenGL context active
    gl_backend: OpenGlBackend,
    camera: CbCamera,
    pub reset_cursor: bool,
}

impl<'a> CbGfx {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        // Init OpenGL
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 2);

        let window_width = 1280;
        let window_height = 720;

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

        let mut editor_window = video_subsystem.window("Editor", 640, 480).build().unwrap();

        let mut canvas = editor_window.into_canvas().build().unwrap();

        return Self {
            reset_cursor: true,
            video_subsystem: video_subsystem,
            window_width: window_width as i32,
            window_height: window_height as i32,
            sdl_context: sdl_context,
            event_pump: event_pump,
            window: window,
            editor_window: canvas,
            editor_visible: true,
            gl_context: ctx,
            gl_backend: gl_backend,
            camera: CbCamera::new(window_width as f32, window_height as f32),
        };
    }

    pub fn toggle_editor_window(&mut self) {
        //UNIMPLEMENTED!();
    }

    pub fn event_pump_mut(&mut self) -> &mut sdl2::EventPump {
        return &mut self.event_pump;
    }

    pub fn center_mouse(&mut self) {
        let x = self.window_width / 2;
        let y = self.window_height / 2;
        let null_ptr = std::ptr::null_mut();
        unsafe {
            sdl2::sys::SDL_WarpMouseInWindow(null_ptr, x, y);
        }
    }

    pub fn camera(&mut self) -> &mut CbCamera {
        return &mut self.camera;
    }

    pub fn render(&mut self, game_state: &CbGameState, world: &World, frame: usize) {
        const SIZE_SCALING_FACTOR: f32 = 100.0;
        const DEGREES_SCALING_FACTOR: f32 = 10.0;

        let camera_components =
            world.read_storage::<cb_simulation::components::gfx_components::CameraComponent>();

        // NOTE: There really should only be one active camera at a time perhaps?
        for camera in (&camera_components).join() {
            self.camera.orthographic_view = camera.camera_orthographic_view;

            self.camera.pitch = (camera.camera_pitch as f32) / DEGREES_SCALING_FACTOR;
            self.camera.roll = (camera.camera_roll as f32) / DEGREES_SCALING_FACTOR;
            self.camera.yaw = (camera.camera_yaw as f32) / DEGREES_SCALING_FACTOR;

            self.camera.pos_x = (camera.camera_pos_x as f32) / SIZE_SCALING_FACTOR;
            self.camera.pos_y = (camera.camera_pos_y as f32) / SIZE_SCALING_FACTOR;
            self.camera.pos_z = (camera.camera_pos_z as f32) / SIZE_SCALING_FACTOR;

            self.camera.target_x = (camera.camera_target_x as f32) / SIZE_SCALING_FACTOR;
            self.camera.target_y = (camera.camera_target_y as f32) / SIZE_SCALING_FACTOR;
            self.camera.target_z = (camera.camera_target_z as f32) / SIZE_SCALING_FACTOR;
        }

        OpenGlBackend::render(&mut self.gl_backend, &self.camera, game_state, world, frame);
        self.window.gl_swap_window();

        // Draw presenters/views
        {
            // Clear canvas
            let canvas = &mut self.editor_window;
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.clear();

            let presenter_components =
                world.read_storage::<cb_simulation::components::RangePresentableTestComponent>();

            for presenter_component in (&presenter_components).join() {
                let presenter = &presenter_component.presenter;
                let view_position = presenter.get_view_position();

                let view_boundary = 5;

                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 210, 0));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        (view_position.x as i32),
                        view_position.y as i32,
                        (view_position.width + view_boundary) as u32,
                        (view_position.height + view_boundary) as u32,
                    ))
                    .unwrap();

                // View rendering
                {
                    let view = presenter.get_view();
                    let view_objects = view.get_view_objects();

                    for view_object in view_objects.iter() {
                        match view_object.object_type {
                            cb_patterns::presenter::ViewObjectTypes::Rectangle => {
                                canvas.set_draw_color(sdl2::pixels::Color::RGB(210, 210, 210));
                                canvas
                                    .fill_rect(sdl2::rect::Rect::new(
                                        (view_object.view_position.x as i32),
                                        view_object.view_position.y as i32,
                                        (view_object.view_position.width) as u32,
                                        (view_object.view_position.height) as u32,
                                    ))
                                    .unwrap();
                            }
                        }
                    }
                }
            }

            // Render
            canvas.present();
        }
    }
}
