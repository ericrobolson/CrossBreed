extern crate gl;
use std::ffi::CString;

extern crate nalgebra as na;
use na::{Isometry3, Perspective3, Point3, Vector3};

use crate::cb_simulation;
use cb_simulation::GameState;

use crate::cb_voxels;

pub mod render_gl;

pub struct OpenGlBackend {
    program: render_gl::Program,
    voxel_vao: u32,
    mvp_id: i32,
    frame: usize,
}

fn init_cube_buffers() -> gl::types::GLuint {
    let vertices: Vec<f32> = vec![
        -1.0, -1.0, -1.0, // triangle 1 : begin
        -1.0, -1.0, 1.0, // triangle 1: mid
        -1.0, 1.0, 1.0, // triangle 1 : end
        1.0, 1.0, -1.0, // triangle 2 : begin
        -1.0, -1.0, -1.0, //
        -1.0, 1.0, -1.0, // triangle 2 : end
        1.0, -1.0, 1.0, // tri3
        -1.0, -1.0, -1.0, //
        1.0, -1.0, -1.0, // tri3
        1.0, 1.0, -1.0, // tri4
        1.0, -1.0, -1.0, //
        -1.0, -1.0, -1.0, // tri4
        -1.0, -1.0, -1.0, // tri begin
        -1.0, 1.0, 1.0, //
        -1.0, 1.0, -1.0, // tri end
        1.0, -1.0, 1.0, // tri begin
        -1.0, -1.0, 1.0, //
        -1.0, -1.0, -1.0, // tri end
        -1.0, 1.0, 1.0, // tri begin
        -1.0, -1.0, 1.0, //
        1.0, -1.0, 1.0, // tri end
        1.0, 1.0, 1.0, // tri begin
        1.0, -1.0, -1.0, //
        1.0, 1.0, -1.0, // tri end
        1.0, -1.0, -1.0, // tri begin
        1.0, 1.0, 1.0, //
        1.0, -1.0, 1.0, // tri end
        1.0, 1.0, 1.0, // tri begin
        1.0, 1.0, -1.0, //
        -1.0, 1.0, -1.0, // tri end
        1.0, 1.0, 1.0, // tri begin
        -1.0, 1.0, -1.0, //
        -1.0, 1.0, 1.0, // tri end
        1.0, 1.0, 1.0, // tri begin
        -1.0, 1.0, 1.0, //
        1.0, -1.0, 1.0, // tri end
    ];

    let mut vbo_tri: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_tri);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_tri);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        let num_vertices = 12 * 3; // 12 triangles, 3 vertices each
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_tri);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    return vao;
}

impl OpenGlBackend {
    pub fn new() -> Self {
        let vert_shader = render_gl::Shader::from_vert_source(
            &CString::new(include_str!("triangle.vert")).unwrap(),
        )
        .unwrap();

        let frag_shader = render_gl::Shader::from_frag_source(
            &CString::new(include_str!("triangle.frag")).unwrap(),
        )
        .unwrap();

        let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
        shader_program.set_used();

        let voxels = init_cube_buffers();

        // MVP uniform
        let mvp_str = &CString::new("MVP").unwrap();
        let mut mvp_id;
        unsafe {
            mvp_id = gl::GetUniformLocation(shader_program.id(), mvp_str.as_ptr());
        }

        // Wireframes?
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }

        return Self {
            program: shader_program,
            voxel_vao: voxels,
            mvp_id: mvp_id,
            frame: 0,
        };
    }

    pub fn render(&mut self, game_state: &GameState) {
        unsafe {
            gl::ClearColor(0.0, 1.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.program.set_used();
        self.draw_voxel(game_state);

        self.frame += 1;
    }

    fn draw_voxel(&mut self, game_state: &GameState) {
        // Camera
        let mapped_pos = self.frame as f32 * 0.01;

        let eye = Point3::new(4.0, 3.0, mapped_pos);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
        let view = view.to_homogeneous();

        // Projection
        let proj = Perspective3::new(4.0 / 3.0, 3.14 / 2.0, 0.1, 100.0);
        let proj = proj.as_matrix();

        let mut draw_calls = 0;

        for ((x, y, z), voxel) in game_state
            .voxel_chunk
            .voxels
            .iter()
            .filter(|((_, _, _), voxel)| voxel.active)
        {
            let x = *x;
            let y = *y;
            let z = *z;

            // if not visible, skip, but always render the outer most voxels
            if x != 0
                && x != cb_voxels::MAX_CHUNK_INDEX
                && y != 0
                && y != cb_voxels::MAX_CHUNK_INDEX
                && z != 0
                && z != cb_voxels::MAX_CHUNK_INDEX
            {
                // same layer
                let n1 = game_state.voxel_chunk.voxel_3d_index(x - 1, y, z);
                let n2 = game_state.voxel_chunk.voxel_3d_index(x + 1, y, z);
                let n3 = game_state.voxel_chunk.voxel_3d_index(x, y - 1, z);
                let n4 = game_state.voxel_chunk.voxel_3d_index(x, y + 1, z);
                let n5 = game_state.voxel_chunk.voxel_3d_index(x + 1, y + 1, z);
                let n6 = game_state.voxel_chunk.voxel_3d_index(x + 1, y - 1, z);
                let n7 = game_state.voxel_chunk.voxel_3d_index(x - 1, y - 1, z);
                let n8 = game_state.voxel_chunk.voxel_3d_index(x - 1, y + 1, z);

                let obscured_by_same_layer = n1.active
                    && n2.active
                    && n3.active
                    && n4.active
                    && n5.active
                    && n6.active
                    && n7.active
                    && n8.active;

                // top layer
                let n1 = game_state.voxel_chunk.voxel_3d_index(x - 1, y, z - 1);
                let n2 = game_state.voxel_chunk.voxel_3d_index(x + 1, y, z - 1);
                let n3 = game_state.voxel_chunk.voxel_3d_index(x, y - 1, z - 1);
                let n4 = game_state.voxel_chunk.voxel_3d_index(x, y + 1, z - 1);
                let n5 = game_state.voxel_chunk.voxel_3d_index(x + 1, y + 1, z - 1);
                let n6 = game_state.voxel_chunk.voxel_3d_index(x + 1, y - 1, z - 1);
                let n7 = game_state.voxel_chunk.voxel_3d_index(x - 1, y - 1, z - 1);
                let n8 = game_state.voxel_chunk.voxel_3d_index(x - 1, y + 1, z - 1);
                let n9 = game_state.voxel_chunk.voxel_3d_index(x, y, z - 1);

                let obscured_by_top_layer = n1.active
                    && n2.active
                    && n3.active
                    && n4.active
                    && n5.active
                    && n6.active
                    && n7.active
                    && n8.active
                    && n9.active;

                // bottom layer
                let n1 = game_state.voxel_chunk.voxel_3d_index(x - 1, y, z + 1);
                let n2 = game_state.voxel_chunk.voxel_3d_index(x + 1, y, z + 1);
                let n3 = game_state.voxel_chunk.voxel_3d_index(x, y - 1, z + 1);
                let n4 = game_state.voxel_chunk.voxel_3d_index(x, y + 1, z + 1);
                let n5 = game_state.voxel_chunk.voxel_3d_index(x + 1, y + 1, z + 1);
                let n6 = game_state.voxel_chunk.voxel_3d_index(x + 1, y - 1, z + 1);
                let n7 = game_state.voxel_chunk.voxel_3d_index(x - 1, y - 1, z + 1);
                let n8 = game_state.voxel_chunk.voxel_3d_index(x - 1, y + 1, z + 1);
                let n9 = game_state.voxel_chunk.voxel_3d_index(x, y, z + 1);

                let obscured_by_bot_layer = n1.active
                    && n2.active
                    && n3.active
                    && n4.active
                    && n5.active
                    && n6.active
                    && n7.active
                    && n8.active
                    && n9.active;

                if obscured_by_same_layer && obscured_by_top_layer && obscured_by_bot_layer {
                    continue;
                }
            }

            let x = x as f32;
            let y = y as f32;
            let z = z as f32;

            unsafe {
                // Model
                let model_pos = Vector3::new(x, y, z);
                let model_pos = Isometry3::new(model_pos, na::zero());

                let model = model_pos.to_homogeneous() * na::Matrix4::identity();
                // MVP
                let mvp = proj * (view * model);

                gl::UniformMatrix4fv(self.mvp_id, 1, gl::FALSE, mvp.as_ptr());

                gl::BindVertexArray(self.voxel_vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 12 * 3);
            }

            draw_calls += 1;
        }

        println!("draw calls: {}", draw_calls);
    }
}
