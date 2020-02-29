use crate::cb_simulation;
use cb_simulation::GameState;

use super::greedy_mesher::calculate_greedy_mesh;
use super::*;

use crate::cb_voxels;
use cb_voxels::*;

pub struct VoxelMesher {
    mesh: Mesh,
    dirty: bool,
}

impl VoxelMesher {
    pub fn new() -> Self {
        return Self {
            mesh: Mesh::new(0, vec![], vec![], 0, vec![], 0),
            dirty: true,
        };
    }

    pub fn mesh(&mut self, game_state: &GameState, frame: usize) -> Mesh {
        //TODO: note, it's expensive to mesh every chunk. Instead, start with the outside chunks of the chunk manager, then mesh those.
        // Afterwards, spiral inward to determine which other chunks to mesh if they're visible.

        let mut meshes = vec![];

        if self.dirty {
            self.dirty = false;
            let mesh = calculate_greedy_mesh(
                &game_state.chunk_manager.chunks[0][0][0].voxels,
                frame,
                CHUNK_SIZE,
            );

            meshes.push(mesh);

            let mesh = Mesh::merge(meshes);

            self.mesh = mesh;
        }

        return self.mesh.clone();

        // Iterate through the chunks top down, only meshing if a chunk is visible
        /*
        let parallel_execution = false;

        self.chunks[0][0][0].mesh(frame);

        if parallel_execution {
            self.chunks.par_iter_mut().enumerate().for_each(|(xi, x)| {
                x.par_iter_mut().enumerate().for_each(|(yi, y)| {
                    y.par_iter_mut().enumerate().for_each(|(zi, chunk)| {
                        if yi == 0 {
                            chunk.mesh(frame);
                        } else if xi == 0 || xi == CHUNKS - 1 {
                            chunk.mesh(frame);
                        } else if zi == 0 || zi == CHUNKS - 1 {
                            chunk.mesh(frame);
                        }
                    })
                })
            });
        }

        self.dirty = false;
        /*
                self.chunks
                    .par_iter_mut()
                    .flatten()
                    .flatten()
                    .for_each(|chunk| {
                        chunk.mesh(frame);
                    });
        */
        println!("individual chunk meshing end");
        let mut meshes = vec![];

        println!("super chunk meshing begin");
        for (ix, x) in self.chunks.iter().enumerate() {
            for (iy, y) in x.iter().enumerate() {
                for (iz, chunk) in y.iter().enumerate() {
                    let mesh = chunk.get_last_mesh();

                    if mesh.is_none() {
                        continue;
                    }

                    let mut mesh = mesh.unwrap();

                    // Modify the offsets so that the mesh is done properly
                    const X_VALUE: usize = 0;
                    const Y_VALUE: usize = 1;
                    const Z_VALUE: usize = 2;

                    const CHUNK_VERTEX_OFFSET: f32 = VOXEL_SIZE * CHUNK_SIZE as f32;

                    const VALUES_IN_VERTEX: usize = 3;
                    mesh.vertices
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, mut vert)| {
                            let value_type = i % VALUES_IN_VERTEX;

                            if value_type == X_VALUE {
                                // add modified x offset
                                *vert += CHUNK_VERTEX_OFFSET * ix as f32;
                            } else if value_type == Y_VALUE {
                                // add modified y offset
                                *vert += CHUNK_VERTEX_OFFSET * iy as f32;
                            } else if value_type == Z_VALUE {
                                // add modified z offset
                                *vert += CHUNK_VERTEX_OFFSET * iz as f32;
                            }
                        });

                    meshes.push(mesh);
                }
            }
        }
        println!("super chunk meshing end");
        println!("super chunk meshing merging begin");

        let mut mesh = Mesh::merge(meshes);
        println!("super chunk meshing merging end");
        self.gen_mesh = Some(mesh);
        */
    }
}
