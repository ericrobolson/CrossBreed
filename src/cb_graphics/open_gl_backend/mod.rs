// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate gl;
use std::ffi::CString;

extern crate nalgebra as na;
use na::{Isometry3, Perspective3, Point3, Vector3};

use crate::cb_graphics;
use crate::cb_simulation;
use cb_simulation::{components, CbGameState};

use crate::cb_voxels;

mod r_collada_render;
use r_collada_render::CbColladaRenderer;
mod r_voxel_render;

pub mod render_gl;

use cb_graphics::cb_collada;
use cb_graphics::mesh;
use cb_graphics::sprites::{CbSpriteRenderer, SpriteRenderer};
use std::path::Path;
extern crate specs;
use specs::prelude::*;

pub struct MeshBuffers {
    pub vao: gl::types::GLuint,
    pub vbo: gl::types::GLuint,
    pub ebo: gl::types::GLuint,
    visible: bool,
    pub color_buff: gl::types::GLuint,
    pub normal_buff: gl::types::GLuint,
    pub last_calculated_frame: usize,
    pub indices_count: usize,
}

pub struct OpenGlBackend {
    basic_mesh_program: render_gl::Program,
    chunk_mesh_buffers: Vec<MeshBuffers>,
    sprite_renderer: CbSpriteRenderer,
    mvp_id: i32,
    light_id: i32,
    frame: usize,
    voxel_mesher: cb_graphics::mesh::voxel_mesher::VoxelMesher,
    collada_renderer: CbColladaRenderer,
}

impl OpenGlBackend {
    pub fn new() -> Self {
        // Collada renderer
        let mut collada_renderer = CbColladaRenderer::new();
        collada_renderer.load_collada(&Path::new("./src/assets/monkey.dae"));

        // Basic mesh program
        let mesh_program;
        {
            let vert_shader = render_gl::Shader::from_vert_source(
                &CString::new(include_str!("mesh.vert")).unwrap(),
            )
            .unwrap();

            let frag_shader = render_gl::Shader::from_frag_source(
                &CString::new(include_str!("mesh.frag")).unwrap(),
            )
            .unwrap();
            mesh_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
        }

        mesh_program.set_used();

        // MVP uniform
        let mvp_str = &CString::new("MVP").unwrap();
        let mvp_id;
        unsafe {
            mvp_id = gl::GetUniformLocation(mesh_program.id(), mvp_str.as_ptr());
        }

        // Light uniform
        let light_str = &CString::new("cbLightPos").unwrap();
        let light_id;
        unsafe {
            light_id = gl::GetUniformLocation(mesh_program.id(), light_str.as_ptr());
        }

        // Backface culling
        unsafe {
            gl::Enable(gl::CULL_FACE);
        }

        // Depth
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        return Self {
            sprite_renderer: CbSpriteRenderer::new(),
            basic_mesh_program: mesh_program,
            chunk_mesh_buffers: r_voxel_render::init_voxel_mesh_buffers(),
            mvp_id: mvp_id,
            light_id: light_id,
            frame: 0,
            voxel_mesher: cb_graphics::mesh::voxel_mesher::VoxelMesher::new(),
            collada_renderer: collada_renderer,
        };
    }

    pub fn render(
        renderer: &mut Self,
        camera: &cb_graphics::CbCamera,
        game_state: &CbGameState,
        world: &World,
        frame: usize,
    ) {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Draw sprites
        {
            renderer.sprite_renderer.batch();
            renderer.sprite_renderer.render();
        }

        // Draw collada
        {
            renderer
                .collada_renderer
                .draw(renderer.mvp_id, camera, frame);
        }

        let draw_voxels = true;

        // Draw voxels
        if draw_voxels {
            let voxel_components =
                world.read_storage::<components::voxel_components::VoxelComponent>();

            // First mesh them
            for voxel in (&voxel_components).join() {
                renderer
                    .voxel_mesher
                    .mesh(&voxel.chunk_manager, frame, camera);
            }

            renderer.basic_mesh_program.set_used();
            r_voxel_render::draw_voxel_meshes(renderer, camera, frame);
        }
        renderer.frame += 1;
    }
}

type CbProjection = cb_graphics::open_gl_backend::na::Matrix<
    f32,
    cb_graphics::open_gl_backend::na::U4,
    cb_graphics::open_gl_backend::na::U4,
    cb_graphics::open_gl_backend::na::ArrayStorage<
        f32,
        cb_graphics::open_gl_backend::na::U4,
        cb_graphics::open_gl_backend::na::U4,
    >,
>;

type CbView = cb_graphics::open_gl_backend::na::Matrix<
    f32,
    cb_graphics::open_gl_backend::na::U4,
    cb_graphics::open_gl_backend::na::U4,
    cb_graphics::open_gl_backend::na::ArrayStorage<
        f32,
        cb_graphics::open_gl_backend::na::U4,
        cb_graphics::open_gl_backend::na::U4,
    >,
>;

fn get_proj_view(camera: &cb_graphics::CbCamera) -> (CbProjection, CbView) {
    let proj;
    let view;
    {
        let horizontal_angle = 3.14;
        let vertical_angle = 0.0;
        let mouse_speed = 0.005; // configure to user variable?

        let delta_time = 1.0; //TODO: figure out
                              /*
                                                            let horizontal_angle = horizontal_angle
                                                                + mouse_speed * delta_time * (camera.window_width / 2.0 - camera.cursor_x);
                                                            let vertical_angle = vertical_angle
                                                                + mouse_speed * delta_time * (camera.window_height / 2.0 - camera.cursor_y);

                                                            let mouse_target = Point3::new(
                                                                vertical_angle.cos() * horizontal_angle.sin(),
                                                                vertical_angle.sin(),
                                                                vertical_angle.cos() * horizontal_angle.cos(),
                                                            );

                                                            let target = Point3::new(
                                  mouse_target.x + camera.pos_x,
                                  mouse_target.y + camera.pos_y,
                                  mouse_target.z + camera.pos_z,
                              );
                                                    */

        //TODO: need to figure out camera pitch, yaw, roll
        let target = Point3::new(camera.target_x, camera.target_y, camera.target_z);

        let eye = Point3::new(camera.pos_x, camera.pos_y, camera.pos_z);
        view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        if camera.orthographic_view {
            const BOUNDS: f32 = 22.0;

            let horizontal_bounds = (camera.window_width / camera.window_height) * BOUNDS; // Ensure it's properly scaled for the aspect ratio
            let vertical_bounds = BOUNDS;

            let ortho = na::Orthographic3::new(
                -horizontal_bounds,
                horizontal_bounds,
                -vertical_bounds,
                vertical_bounds,
                0.01,
                10000.0,
            );

            proj = Perspective3::from_matrix_unchecked(*ortho.as_matrix());
        } else {
            proj = Perspective3::new(
                (camera.window_width / camera.window_height), // Ensure it's properly scaled for the aspect ratio
                3.14 / 2.0,
                0.1,
                1000.0,
            );
        }
    }

    let proj: CbProjection = *proj.as_matrix();
    let view = view.to_homogeneous();

    return (proj, view);
}
