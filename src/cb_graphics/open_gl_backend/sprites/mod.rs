use super::*;

use crate::cb_graphics::mesh;
use mesh::Mesh;

use crate::cb_math;
use cb_math::index_1d_to_3d;

use crate::cb_voxels;
use cb_voxels::*;

use crate::cb_graphics;
use cb_graphics::open_gl_backend::render_gl;

pub struct SpriteRenderer {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    program: render_gl::Program,
}

impl SpriteRenderer {
    pub fn new() -> Self {
        let mut vao: gl::types::GLuint = 0;
        let mut vbo: gl::types::GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
        }

        const VERTEX_SIZE: usize = 4;

        let vertices = vec![
            // Pos    // Tex
            0.0, 1.0, 0.0, 1.0, //
            1.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 0.0, //
            //////////////////////
            0.0, 1.0, 0.0, 1.0, //
            1.0, 1.0, 1.0, 1.0, //
            1.0, 0.0, 1.0, 0.0, //
        ];

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }

        unsafe {
            gl::BindVertexArray(vao);
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                0,
                VERTEX_SIZE as gl::types::GLint,
                gl::FLOAT,
                gl::FALSE,
                (VERTEX_SIZE * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

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
        }

        return Self {
            vao: vao,
            vbo: vbo,
            program: sprite_program,
        };
    }

    pub fn render(&self, camera: &cb_graphics::CbCamera, frame: usize) {
        // Camera / MVP
        let (proj, view) = get_proj_view(camera);

        self.program.set_used();
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }
}

pub fn draw_sprites(backend: &mut OpenGlBackend, camera: &cb_graphics::CbCamera, frame: usize) {
    // Camera / MVP
    let (proj, view) = get_proj_view(camera);
    draw_meshes(0, 0, 0, proj, view, backend, frame);
}

fn draw_meshes(
    x: usize,
    y: usize,
    z: usize,
    proj: CbProjection,
    view: CbView,
    backend: &mut OpenGlBackend,
    frame: usize,
) {
    // Set MVP
    {
        // Model
        const CHUNK_OFFSET: f32 = (cb_voxels::CHUNK_SIZE) as f32 * cb_voxels::VOXEL_SIZE;

        let model_pos = Vector3::new(
            x as f32 * CHUNK_OFFSET,
            y as f32 * CHUNK_OFFSET,
            z as f32 * CHUNK_OFFSET,
        );

        let model_pos = Isometry3::new(model_pos, na::zero());

        let model = model_pos.to_homogeneous() * na::Matrix4::identity();
        // MVP
        let mvp = proj * (view * model);

        unsafe {
            gl::UniformMatrix4fv(backend.mvp_id, 1, gl::FALSE, mvp.as_ptr());
        }
    }

    // Set global light pos
    {
        let light_pos = Vector3::new(1.0 as f32, 2.0 as f32, 0.0 as f32);

        unsafe {
            gl::UniformMatrix3fv(backend.light_id, 1, gl::FALSE, light_pos.as_ptr());
        }
    }

    // Render the meshes
    for i in 0..CHUNKS_CUBED {
        let mut buffer = &mut backend.chunk_mesh_buffers[i];

        // Get the last frame the meshes were updated at
        let most_recent_mesh_update_frame: usize =
            backend.voxel_mesher.meshes[i].mesh.generated_at_frame;
        // Only update the buffers if it's needed
        let mut mesh = None;
        let changed;
        {
            // Only copy over the mesh if it's the first frame or it's been updated
            if most_recent_mesh_update_frame > buffer.last_calculated_frame || frame == 0 {
                let m = &backend.voxel_mesher.meshes[i].mesh;
                changed = true;

                // Store the frame the mesh was generated at
                buffer.last_calculated_frame = m.generated_at_frame;
                buffer.indices_count = m.indices.len();

                mesh = Some(m);
            } else {
                changed = false;
            }
        }

        // Update the buffers with the latest mesh
        if changed && mesh.is_some() {
            let mesh = mesh.unwrap();
            unsafe {
                // Buffer vertices
                gl::BindVertexArray(buffer.vao);
                gl::BindBuffer(gl::ARRAY_BUFFER, buffer.vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (mesh.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    mesh.vertices.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );

                // Buffer indices
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (mesh.indices.len() * std::mem::size_of::<i32>()) as gl::types::GLsizeiptr,
                    mesh.indices.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );

                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                    std::ptr::null(),
                );

                // Vertices
                gl::VertexAttribPointer(
                    0,
                    mesh.vertex_size as gl::types::GLint,
                    gl::FLOAT,
                    gl::FALSE,
                    (mesh.vertex_size * std::mem::size_of::<f32>()) as gl::types::GLint,
                    std::ptr::null(),
                );

                gl::EnableVertexAttribArray(0);

                // Colors
                gl::EnableVertexAttribArray(1);
                gl::BindBuffer(gl::ARRAY_BUFFER, buffer.color_buff);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (mesh.colors.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    mesh.colors.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );
                gl::VertexAttribPointer(
                    1,
                    mesh.color_vertex_size as gl::types::GLint,
                    gl::FLOAT,
                    gl::FALSE,
                    (mesh.color_vertex_size * std::mem::size_of::<f32>()) as gl::types::GLint,
                    std::ptr::null(),
                );

                // Normals
                gl::EnableVertexAttribArray(2);
                gl::BindBuffer(gl::ARRAY_BUFFER, buffer.normal_buff);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (mesh.normals.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    mesh.normals.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );
                gl::VertexAttribPointer(
                    2,
                    mesh.normal_vertex_size as gl::types::GLint,
                    gl::FLOAT,
                    gl::FALSE,
                    (mesh.normal_vertex_size * std::mem::size_of::<f32>()) as gl::types::GLint,
                    std::ptr::null(),
                );

                gl::BindVertexArray(0);
            }
        }

        unsafe {
            //    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }

        // Render
        unsafe {
            gl::BindVertexArray(buffer.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                buffer.indices_count as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
        }

        // End render mesh
    }
}
