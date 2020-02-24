extern crate gl;
use std::ffi::CString;

use crate::cb_simulation;
use cb_simulation::GameState;

pub mod render_gl;

pub struct OpenGlBackend {
    program: render_gl::Program,
    vao: u32,
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

        let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
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
            let num_vertices = 3;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                num_vertices,
                gl::FLOAT,
                gl::FALSE,
                (num_vertices as usize * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        return Self {
            program: shader_program,
            vao: vao,
        };
    }

    pub fn render(&mut self, game_state: &GameState) {
        self.program.set_used();

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
