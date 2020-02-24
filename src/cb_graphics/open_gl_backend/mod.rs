extern crate gl;
use std::ffi::CString;

use crate::cb_simulation;
use cb_simulation::GameState;

pub mod render_gl;

pub struct OpenGlBackend {
    program: render_gl::Program,
    voxel_vao: u32,
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

        return Self {
            program: shader_program,
            voxel_vao: voxels,
        };
    }

    pub fn render(&mut self, game_state: &GameState) {
        self.program.set_used();
        self.draw_voxel();
        // draw triangle
    }

    fn draw_voxel(&mut self) {
        unsafe {
            gl::BindVertexArray(self.voxel_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 12 * 3);
        }
    }
}
