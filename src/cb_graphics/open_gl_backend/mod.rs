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
        gl::BindVertexArray(vao);
        /*
        let num_vertices = 12 * 3; // 12 triangles, 3 vertices each
        gl::GenVertexArrays(1, &mut vao);
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
        */
    }

    return vao;
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

        //let meshes = init_mesh_buffers();
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
            gl::CullFace(gl::BACK);
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
            gl::ClearColor(0.0, 1.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.basic_mesh_program.set_used();
        self.draw_meshes(camera, game_state);
        //  self.draw_voxel_old(camera, game_state);

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

        let mut draw_count = 0;

        // Set MVP
        {
            // Model
            let model_pos = Vector3::new(0.0, 0.0, 0.0);

            let model_pos = Isometry3::new(model_pos, na::zero());

            let model = model_pos.to_homogeneous() * na::Matrix4::identity();
            // MVP
            let mvp = proj * (view * model);

            unsafe {
                gl::UniformMatrix4fv(self.mvp_id, 1, gl::FALSE, mvp.as_ptr());
            }
        }
        let meshes = game_state.voxel_chunk.get_last_mesh();

        // http://nercury.github.io/rust/opengl/tutorial/2018/02/11/opengl-in-rust-from-scratch-04-triangle.html
        // https://learnopengl.com/Getting-started/Hello-Triangle

        // flatten all the data so it can be put into one buffer + draw call; look into this. Need to ensure that vertex number counts are the exact same.
        //TODO: Ensure vertex counts are the same, if not need to figure out
        let mut vertices = vec![];
        let mut indices = vec![];

        for (i, mesh) in meshes.iter().enumerate() {
            let mut start_vertex_index = vertices.len() as i32 - 1;

            if start_vertex_index < 1 {
                start_vertex_index = 0;
            }

            for vertex in mesh.vertices.iter() {
                vertices.push(vertex);
            }

            for index in mesh.indices.iter() {
                indices.push(index + start_vertex_index);
            }
        }

        // Render the data; iterates over each mesh; naive implementation
        for mesh in meshes.iter() {
            if mesh.wire_frame {
                // Wireframes?
                unsafe {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
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
                    (mesh.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    mesh.vertices.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );

                // 3
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.voxel_ebo);
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
                    (mesh.colors.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    mesh.colors.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );
                gl::VertexAttribPointer(
                    1,
                    mesh.color_vertex_size as gl::types::GLint,
                    gl::FLOAT,
                    gl::FALSE,
                    0 as gl::types::GLint,
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
                /*
                                // ..:: Initialization code :: ..
                // 1. bind Vertex Array Object
                glBindVertexArray(VAO);
                // 2. copy our vertices array in a vertex buffer for OpenGL to use
                glBindBuffer(GL_ARRAY_BUFFER, VBO);
                glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
                // 3. copy our index array in a element buffer for OpenGL to use
                glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, EBO);
                glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_STATIC_DRAW);
                // 4. then set the vertex attributes pointers
                glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);
                glEnableVertexAttribArray(0);

                [...]

                // ..:: Drawing code (in render loop) :: ..
                glUseProgram(shaderProgram);
                glBindVertexArray(VAO);
                glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0)
                glBindVertexArray(0);
                */
            }
        }
        /*
                for mesh in meshes.iter() {
                    // bind buffers
                    {
                        let mut ebo: gl::types::GLuint = 0;
                        unsafe {
                            gl::GenBuffers(1, &mut ebo);
                            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                            gl::BufferData(
                                gl::ELEMENT_ARRAY_BUFFER,
                                (mesh.indices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                                mesh.indices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                                gl::STATIC_DRAW,                                   // usage
                            );
                            // glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_STATIC_DRAW);
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
                    }
                }
        */
    }

    fn draw_voxel_old(&mut self, camera: &cb_graphics::CbCamera, game_state: &GameState) {
        // Camera
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

        let mut draw_count = 0;

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

            draw_count += 1;
        }

        println!("draw counts - cubes: {}", draw_count);
    }
}
