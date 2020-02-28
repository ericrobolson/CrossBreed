extern crate gl;
use std::ffi::CString;

extern crate nalgebra as na;
use na::{Isometry3, Perspective3, Point3, Vector3};

use crate::cb_simulation;
use cb_simulation::GameState;

use crate::cb_graphics;

use crate::cb_voxels;

pub mod render_gl;

pub struct OpenGlBackend {
    basic_mesh_program: render_gl::Program,
    //mesh_vao: u32,
    voxel_vao: u32,
    voxel_vbo: u32,
    voxel_ebo: u32,
    voxel_color_buff: u32,
    mvp_id: i32,
    frame: usize,
}

fn init_mesh_buffers() -> (
    gl::types::GLuint,
    gl::types::GLuint,
    gl::types::GLuint,
    gl::types::GLuint,
) {
    let mut vao: gl::types::GLuint = 0;
    let mut vbo: gl::types::GLuint = 0;
    let mut ebo: gl::types::GLuint = 0;
    let mut color_buff: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        gl::GenBuffers(1, &mut color_buff);
    }

    return (vao, vbo, ebo, color_buff);
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

        let (voxels, vbo, ebo, color_buff) = init_mesh_buffers();

        // MVP uniform
        let mvp_str = &CString::new("MVP").unwrap();
        let mut mvp_id;
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
            voxel_vao: voxels,
            voxel_vbo: vbo,
            voxel_ebo: ebo,
            voxel_color_buff: color_buff,
            mvp_id: mvp_id,
            frame: 0,
        };
    }

    pub fn render(&mut self, camera: &cb_graphics::CbCamera, game_state: &GameState) {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.basic_mesh_program.set_used();
        self.draw_meshes(camera, game_state);

        self.frame += 1;
    }

    fn draw_meshes(&mut self, camera: &cb_graphics::CbCamera, game_state: &GameState) {
        // Camera / MVP

        let horizontal_angle = 3.14;
        let vertical_angle = 0.0;
        let initial_fov = 45.0;
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
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
        let view = view.to_homogeneous();

        let proj = Perspective3::new(4.0 / 3.0, 3.14 / 2.0, 0.1, 100.0);
        let proj = proj.as_matrix();

        for (x, row) in game_state.chunk_manager.chunks.iter().enumerate() {
            for (y, column) in row.iter().enumerate() {
                for (z, chunk) in column.iter().enumerate() {
                    let mesh = chunk.get_last_mesh();

                    // Set MVP
                    {
                        // Model
                        const CHUNK_OFFSET: f32 =
                            (cb_voxels::CHUNK_SIZE) as f32 * cb_voxels::VOXEL_SIZE;

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
                            gl::UniformMatrix4fv(self.mvp_id, 1, gl::FALSE, mvp.as_ptr());
                        }
                    }

                    // Render the mesh
                    {
                        // http://nercury.github.io/rust/opengl/tutorial/2018/02/11/opengl-in-rust-from-scratch-04-triangle.html
                        // https://learnopengl.com/Getting-started/Hello-Triangle

                        // TODO: look into flattening meshes/ using one buffer
                        // TODO: look into only reloading buffers if changed
                        // flatten all the data so it can be put into one buffer + draw call; look into this. Need to ensure that vertex number counts are the exact same.
                        //TODO: Ensure vertex counts are the same, if not need to figure out

                        // Render the data; iterates over each mesh; naive implementation
                        if mesh.wire_frame {
                            // Wireframes?
                            unsafe {
                                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                            }
                        }

                        if mesh.disable_polygon_smooth {
                            unsafe {
                                gl::Disable(gl::POLYGON_SMOOTH);
                            }
                        }

                        unsafe {
                            // Optimization: only update the buffer if it's needed
                            // 1
                            gl::BindVertexArray(self.voxel_vao);

                            // 2
                            gl::BindBuffer(gl::ARRAY_BUFFER, self.voxel_vbo);
                            gl::BufferData(
                                gl::ARRAY_BUFFER,
                                (mesh.vertices.len() * std::mem::size_of::<f32>())
                                    as gl::types::GLsizeiptr,
                                mesh.vertices.as_ptr() as *const gl::types::GLvoid,
                                gl::STATIC_DRAW,
                            );

                            // 3
                            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.voxel_ebo);
                            gl::BufferData(
                                gl::ELEMENT_ARRAY_BUFFER,
                                (mesh.indices.len() * std::mem::size_of::<i32>())
                                    as gl::types::GLsizeiptr,
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

                            // 4 vertices
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
                            gl::BindBuffer(gl::ARRAY_BUFFER, self.voxel_color_buff);
                            gl::BufferData(
                                gl::ARRAY_BUFFER,
                                (mesh.colors.len() * std::mem::size_of::<f32>())
                                    as gl::types::GLsizeiptr,
                                mesh.colors.as_ptr() as *const gl::types::GLvoid,
                                gl::STATIC_DRAW,
                            );
                            gl::VertexAttribPointer(
                                1,
                                mesh.color_vertex_size as gl::types::GLint,
                                gl::FLOAT,
                                gl::FALSE,
                                (mesh.color_vertex_size * std::mem::size_of::<f32>())
                                    as gl::types::GLint,
                                std::ptr::null(),
                            );

                            // Render
                            {
                                gl::BindVertexArray(self.voxel_vao);
                                gl::DrawElements(
                                    gl::TRIANGLES,
                                    mesh.indices.len() as i32,
                                    gl::UNSIGNED_INT,
                                    std::ptr::null(),
                                );
                                gl::BindVertexArray(0);
                            }
                        }

                        // End render mesh
                    }
                }
            }
        }
    }
}
