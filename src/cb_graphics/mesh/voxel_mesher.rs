use crate::cb_simulation;
use cb_simulation::GameState;

use crate::cb_math;
use cb_math::index_1d_to_3d;

extern crate rayon;
use rayon::prelude::*;

use super::greedy_mesher::calculate_greedy_mesh;
use super::*;

use crate::cb_voxels;
use cb_voxels::*;

pub struct VoxelMesher {
    pub mesh: Mesh,
    dirty: bool,
    pub meshes: Vec<Mesh>,
    chunk_x_index: usize,
    chunk_y_index: usize,
    chunk_z_index: usize,
    lod_scale: usize,
}

impl VoxelMesher {
    pub fn new() -> Self {
        let empty_mesh = Mesh::new(3, vec![], vec![], 3, vec![], 0);

        let mut voxel_vec = Vec::with_capacity(CHUNK_SIZE_CUBED);
        for _ in 0..CHUNK_SIZE_CUBED {
            voxel_vec.push(Mesh::new(3, vec![], vec![], 3, vec![], 0));
        }

        return Self {
            mesh: empty_mesh,
            meshes: voxel_vec,
            dirty: true,
            chunk_x_index: 0,
            chunk_y_index: 0,
            chunk_z_index: 0,
            lod_scale: CHUNKS,
        };
    }

    pub fn mesh(&mut self, chunk_manager: &cb_voxels::CbChunkManager, frame: usize) -> &Mesh {
        //TODO: note, it's expensive to mesh every chunk. Instead, start with the outside chunks of the chunk manager, then mesh those.
        // Afterwards, spiral inward to determine which other chunks to mesh if they're visible.

        self.lod_scale = 1;

        if self.dirty {
            //TODO: look into splitting out down sampling + meshing. In that if the a down sampled chunk is totally obscured, don't mesh it.

            self.meshes = chunk_manager
                .chunks
                .par_iter()
                .enumerate()
                .map(|(index, chunk)| {
                    let scaling_factor = self.lod_scale;
                    let voxels = &chunk.voxel_vec;

                    // Downsample voxels
                    let sampled_voxels;
                    let mut working_vec = vec![];

                    if scaling_factor != 1 {
                        let range = CHUNK_SIZE / scaling_factor;
                        for x in 0..range {
                            for y in 0..range {
                                for z in 0..range {
                                    // Get the avg values

                                    let range = 0..scaling_factor;
                                    let indexes: Vec<(usize, usize, usize)> =
                                        range.map(|i| (x + i, y + i, z + i)).collect();

                                    let values: Vec<(bool, u8)> = indexes
                                        .iter()
                                        .map(|(x, y, z)| {
                                            let i1 = x + y * CHUNK_SIZE + z * CHUNK_SIZE_SQUARED;

                                            return voxels[i1];
                                        })
                                        .collect();

                                    let voxel_types: Vec<u8> =
                                        values.iter().map(|(_, value)| *value).collect();

                                    let active_count: Vec<bool> = values
                                        .iter()
                                        .filter_map(|(active, _)| {
                                            if *active {
                                                return Some(true);
                                            }

                                            return None;
                                        })
                                        .collect();

                                    // Unlikely to happen, but switch to a usize to prevent overflows
                                    let avg_type: usize =
                                        voxel_types.iter().map(|u| *u as usize).sum();
                                    let avg_type = (avg_type / voxel_types.len()) as u8;

                                    // If any voxels are active, draw it
                                    let avg_active = active_count.len() > 0;

                                    working_vec.push((avg_active, avg_type));
                                }
                            }
                        }

                        sampled_voxels = &working_vec;
                    } else {
                        // No sampling, so just regular voxels
                        sampled_voxels = &voxels;
                    }

                    let mut mesh =
                        calculate_greedy_mesh(&sampled_voxels, frame, CHUNK_SIZE / scaling_factor);

                    // Scale verts
                    {
                        // Modify the offsets so that the mesh is done properly
                        const X_VALUE: usize = 0;
                        const Y_VALUE: usize = 1;
                        const Z_VALUE: usize = 2;

                        // convert 1d array index to 3d index
                        let (x, y, z) = index_1d_to_3d(index, CHUNKS, CHUNKS);

                        const CHUNK_VERTEX_OFFSET: f32 = VOXEL_SIZE * CHUNK_SIZE as f32;

                        let values_in_vertex = mesh.vertex_size;
                        mesh.vertices
                            .par_iter_mut()
                            .enumerate()
                            .for_each(|(i, mut vert)| {
                                let value_type = i % values_in_vertex;

                                *vert *= scaling_factor as f32;

                                if value_type == X_VALUE {
                                    // add modified x offset
                                    *vert += CHUNK_VERTEX_OFFSET * x as f32;
                                } else if value_type == Y_VALUE {
                                    // add modified y offset
                                    *vert += CHUNK_VERTEX_OFFSET * y as f32;
                                } else if value_type == Z_VALUE {
                                    // add modified z offset
                                    *vert += CHUNK_VERTEX_OFFSET * z as f32;
                                }
                            });
                    }

                    return mesh;
                })
                .collect();

            //self.meshes = meshes;
            self.dirty = false;

            if self.lod_scale == 1 {
                self.dirty = false;
            } else {
                self.lod_scale = self.lod_scale / 2;
            }

            let mut mesh = Mesh::merge(&self.meshes);
            self.mesh = mesh;
        }

        return &self.mesh;
    }
}
