extern crate gl;
use std::ffi::CString;

extern crate nalgebra as na;
use na::{Isometry3, Perspective3, Point3, Vector3};

use crate::cb_simulation;
use cb_simulation::GameState;

use crate::cb_graphics;

use crate::cb_voxels;

mod r_voxel_render;

pub mod render_gl;

use cb_graphics::mesh;

pub struct MeshBuffers {
    pub vao: gl::types::GLuint,
    pub vbo: gl::types::GLuint,
    pub ebo: gl::types::GLuint,
    pub color_buff: gl::types::GLuint,
    pub normal_buff: gl::types::GLuint,
    pub last_calculated_frame: usize,
}

pub struct OpenGlBackend {
    basic_mesh_program: render_gl::Program,
    chunk_mesh_buffers: MeshBuffers,
    mvp_id: i32,
    frame: usize,
    voxel_mesher: cb_graphics::mesh::voxel_mesher::VoxelMesher,
}

impl OpenGlBackend {
    pub fn new() -> Self {
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

        // Backface culling
        unsafe {
            gl::Enable(gl::CULL_FACE);
        }

        // Depth
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        return Self {
            basic_mesh_program: mesh_program,
            chunk_mesh_buffers: r_voxel_render::init_voxel_mesh_buffers(),
            mvp_id: mvp_id,
            frame: 0,
            voxel_mesher: cb_graphics::mesh::voxel_mesher::VoxelMesher::new(),
        };
    }

    pub fn render(
        renderer: &mut Self,
        camera: &cb_graphics::CbCamera,
        game_state: &GameState,
        frame: usize,
    ) {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Draw voxels
        {
            // First mesh them
            renderer.voxel_mesher.mesh(&game_state.chunk_manager, frame);

            renderer.basic_mesh_program.set_used();
            r_voxel_render::draw_voxel_meshes(renderer, camera);
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

        let horizontal_angle = horizontal_angle
            + mouse_speed * delta_time * (camera.window_width / 2.0 - camera.cursor_x);
        let vertical_angle = vertical_angle
            + mouse_speed * delta_time * (camera.window_height / 2.0 - camera.cursor_y);

        let mouse_target = Point3::new(
            vertical_angle.cos() * horizontal_angle.sin(),
            vertical_angle.sin(),
            vertical_angle.cos() * horizontal_angle.cos(),
        );

        let eye = Point3::new(camera.pos_x, camera.pos_y, camera.pos_z);
        let target = Point3::new(
            mouse_target.x + camera.pos_x,
            mouse_target.y + camera.pos_y,
            mouse_target.z + camera.pos_z,
        );
        view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
        proj = Perspective3::new(4.0 / 3.0, 3.14 / 2.0, 0.1, 100.0);
    }

    let proj: CbProjection = *proj.as_matrix();
    let view = view.to_homogeneous();

    return (proj, view);
}
