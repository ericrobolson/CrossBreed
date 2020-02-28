use super::*;

pub fn init_voxel_mesh_buffers() -> Vec<MeshBuffers> {
    let mut chunk_buffers = vec![];

    // For each voxel chunk, add a buffer to write to
    for _ in 0..cb_voxels::NUM_CHUNKS {
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

        chunk_buffers.push(MeshBuffers {
            vao: vao,
            vbo: vbo,
            ebo: ebo,
            color_buff: color_buff,
            last_calculated_frame: 0,
        });
    }

    return chunk_buffers;
}

pub fn draw_voxel_meshes(
    backend: &mut OpenGlBackend,
    camera: &cb_graphics::CbCamera,
    game_state: &GameState,
) {
    // Camera / MVP
    let (proj, view) = get_proj_view(camera);

    let mut chunk_buff_index = 0;
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
                        gl::UniformMatrix4fv(backend.mvp_id, 1, gl::FALSE, mvp.as_ptr());
                    }
                }

                // Render the mesh
                {
                    // Load the buffers
                    let buffers = &mut backend.chunk_mesh_buffers[chunk_buff_index];
                    chunk_buff_index += 1;

                    // Only update the buffers if it's needed
                    let changed;
                    {
                        if mesh.generated_at_frame > buffers.last_calculated_frame {
                            changed = true;

                            // Store the frame the mesh was generated at
                            buffers.last_calculated_frame = mesh.generated_at_frame;
                        } else {
                            changed = false;
                        }
                    }

                    // Update the buffers with the latest mesh
                    if changed {
                        unsafe {
                            // Buffer vertices
                            gl::BindVertexArray(buffers.vao);
                            gl::BindBuffer(gl::ARRAY_BUFFER, buffers.vbo);
                            gl::BufferData(
                                gl::ARRAY_BUFFER,
                                (mesh.vertices.len() * std::mem::size_of::<f32>())
                                    as gl::types::GLsizeiptr,
                                mesh.vertices.as_ptr() as *const gl::types::GLvoid,
                                gl::STATIC_DRAW,
                            );

                            // Buffer indices
                            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffers.ebo);
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
                            gl::BindBuffer(gl::ARRAY_BUFFER, buffers.color_buff);
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
                        }
                    }

                    // Misc draw functions
                    {
                        if mesh.wire_frame {
                            unsafe {
                                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                            }
                        }

                        if mesh.disable_polygon_smooth {
                            unsafe {
                                gl::Disable(gl::POLYGON_SMOOTH);
                            }
                        }
                    }

                    // Render
                    unsafe {
                        gl::BindVertexArray(buffers.vao);
                        gl::DrawElements(
                            gl::TRIANGLES,
                            mesh.indices.len() as i32,
                            gl::UNSIGNED_INT,
                            std::ptr::null(),
                        );
                        gl::BindVertexArray(0);
                    }

                    // End render mesh
                }
            }
        }
    }
}
