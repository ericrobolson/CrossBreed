use std::mem;
extern crate gl;
use super::*;
use open_gl_backend::render_gl;
use std::ffi::CString;
extern crate nalgebra as na;
use na::{Isometry3, Perspective3, Point3, Vector3};

pub trait SpriteRenderer {
    fn new() -> Self;
    fn batch(&mut self);
    fn render(&self);
}

pub struct CbSpriteRenderer {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    model_uniform_id: i32,
    color_uniform_id: i32,
    sprite_program: open_gl_backend::render_gl::Program,
}

impl SpriteRenderer for CbSpriteRenderer {
    fn new() -> Self {
        let sprite_program;
        {
            let vert_shader = render_gl::Shader::from_vert_source(
                &CString::new(include_str!("sprites.vert")).unwrap(),
            )
            .unwrap();

            let frag_shader = render_gl::Shader::from_frag_source(
                &CString::new(include_str!("sprites.frag")).unwrap(),
            )
            .unwrap();
            sprite_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
            sprite_program.set_used();
        }

        let model_str = &CString::new("model").unwrap();
        let model_uniform_id;
        unsafe {
            model_uniform_id = gl::GetUniformLocation(sprite_program.id(), model_str.as_ptr());
        }

        let color_str = &CString::new("spriteColor").unwrap();
        let color_uniform_id;
        unsafe {
            color_uniform_id = gl::GetUniformLocation(sprite_program.id(), color_str.as_ptr());
        }

        let mut vao: gl::types::GLuint = 0;
        let mut vbo: gl::types::GLuint = 0;

        const VALUES_IN_VERTEX: i32 = 4;
        let mut vertices = [
            // Pos      // Tex
            0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
        ];

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(vao);
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                0,
                VALUES_IN_VERTEX,
                gl::FLOAT,
                gl::FALSE,
                (VALUES_IN_VERTEX as usize * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        return Self {
            vao: vao,
            vbo: vbo,
            sprite_program: sprite_program,
            model_uniform_id: model_uniform_id,
            color_uniform_id: color_uniform_id,
        };
    }

    fn batch(&mut self) {}
    fn render(&self) {
        // for reference....
        // https://learnopengl.com/In-Practice/2D-Game/Rendering-Sprites

        self.sprite_program.set_used();

        let x = 10; // replace with component stuff
        let y = 10; // replace with component stuff
        let color = Vector3::new(1 as f32, 1 as f32, 1.0);

        let model_pos = Vector3::new(x as f32, y as f32, 0.0);
        let model_pos = Isometry3::new(model_pos, na::zero());

        //TODO: rotation, scaling

        let model = model_pos.to_homogeneous() * na::Matrix4::identity();

        unsafe {
            gl::UniformMatrix4fv(self.model_uniform_id, 1, gl::FALSE, model.as_ptr());
            gl::Uniform3fv(self.color_uniform_id, 1, color.as_ptr());
        }

        // DO TEXTURE STUFF
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
        }

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }
}
