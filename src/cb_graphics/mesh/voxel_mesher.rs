// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

use crate::cb_simulation;
use cb_simulation::CbGameState;

use crate::cb_graphics;

extern crate nalgebra as na;
use na::Vector3;

use crate::cb_math;
use cb_math::index_1d_to_3d;

extern crate rayon;
use rayon::prelude::*;

use super::greedy_mesher::calculate_greedy_mesh;
use super::*;

use crate::cb_voxels;
use cb_voxels::*;

pub struct VoxelMeshWrapper {
    pub mesh: Mesh,
    pub lod_scale: usize,
}

impl VoxelMeshWrapper {
    pub fn new(lod_scale: usize, mesh: Mesh) -> Self {
        return Self {
            lod_scale: lod_scale,
            mesh: mesh,
        };
    }
}

pub struct VoxelMesher {
    pub mesh: Mesh,
    first_frame: bool,
    pub meshes: Vec<VoxelMeshWrapper>,
}

impl VoxelMesher {
    pub fn new() -> Self {
        let mut voxel_vec = Vec::with_capacity(CHUNK_SIZE_CUBED);
        for _ in 0..CHUNK_SIZE_CUBED {
            voxel_vec.push(VoxelMeshWrapper::new(
                1,
                Mesh::new(3, vec![], vec![], 3, vec![], 3, vec![], 0),
            ));
        }

        return Self {
            mesh: Mesh::new(3, vec![], vec![], 3, vec![], 3, vec![], 0),
            meshes: voxel_vec,
            first_frame: true,
        };
    }

    pub fn mesh(
        &mut self,
        chunk_manager: &cb_voxels::CbChunkManager,
        frame: usize,
        camera: &cb_graphics::CbCamera,
    ) {
        /*
        XXXXXXXXXXXXXXXXXXXXXXX
        NOTE: CURRENTLY SUPPORTS A SINGLE VOXEL CHUNK, DOES NOT TAKE INTO ACCOUNT POSITION/VELOCITY COMPONENTS YET
        XXXXXXXXXXXXXXXXXXXXXXX
        */

        let first_frame = self.first_frame; // NOTE:

        let camera_coords = Vector3::new(camera.pos_x, camera.pos_y, camera.pos_z);

        let max_voxel_coordinates = CHUNKS as f32 * CHUNK_SIZE as f32 * VOXEL_SIZE;

        // Calculate the acceptable lods
        let mut available_lods = vec![];
        {
            available_lods.push(1);
            let mut i = 2;

            while i < CHUNKS {
                available_lods.push(i);
                i = i * 2;
            }
        }

        let available_lods = available_lods;

        let lod_enabled = false;

        // Go through and rebuild meshes that have changed
        {
            self.meshes
                .par_iter_mut()
                .enumerate()
                .zip(chunk_manager.chunks.par_iter())
                .for_each(|((i, mesh), chunk)| {
                    // Calculate lod to scale at; note: should be done by comparing to camera position
                    let scaling_factor;
                    {
                        if lod_enabled {
                            let chunk_coords = index_1d_to_3d(i, CHUNKS, CHUNKS);

                            let chunk_coords = Vector3::new(
                                (chunk_coords.0 * CHUNK_SIZE) as f32 * VOXEL_SIZE,
                                (chunk_coords.1 * CHUNK_SIZE) as f32 * VOXEL_SIZE,
                                (chunk_coords.2 * CHUNK_SIZE) as f32 * VOXEL_SIZE,
                            );

                            let dist = camera_coords - chunk_coords;
                            let d = dist.norm();

                            if d > max_voxel_coordinates {
                                scaling_factor = *available_lods.last().unwrap();
                            } else {
                                // map scaling factor

                                let lod = d / max_voxel_coordinates;

                                let mut index = (available_lods.len() as f32 * lod * 0.8) as usize;

                                if index > available_lods.len() {
                                    index = 0;
                                }

                                scaling_factor = available_lods[index];

                                //scaling_factor = 1;
                            }
                        } else {
                            scaling_factor = 1;
                        }
                    }

                    let chunk_updated = chunk.frame_updated_at >= mesh.mesh.generated_at_frame;
                    let lod_changed = mesh.lod_scale != scaling_factor;

                    if chunk_updated || lod_changed || first_frame {
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

                                        let values: Vec<(bool, u8, u8)> = indexes
                                            .iter()
                                            .map(|(x, y, z)| {
                                                let i1 =
                                                    x + y * CHUNK_SIZE + z * CHUNK_SIZE_SQUARED;

                                                return voxels[i1];
                                            })
                                            .collect();

                                        let voxel_types: Vec<u8> =
                                            values.iter().map(|(_, value, _)| *value).collect();

                                        let active_count: Vec<bool> = values
                                            .iter()
                                            .filter_map(|(active, _, _)| {
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

                                        working_vec.push((avg_active, avg_type, 0));
                                    }
                                }
                            }

                            sampled_voxels = &working_vec;
                        } else {
                            // No sampling, so just regular voxels
                            sampled_voxels = voxels;
                        }

                        let mut greedy_mesh = calculate_greedy_mesh(
                            &sampled_voxels,
                            frame,
                            CHUNK_SIZE / scaling_factor,
                        );

                        // Scale verts
                        {
                            // Modify the offsets so that the mesh is done properly
                            const X_VALUE: usize = 0;
                            const Y_VALUE: usize = 1;
                            const Z_VALUE: usize = 2;

                            // convert 1d array index to 3d index
                            let (x, y, z) = index_1d_to_3d(i, CHUNKS, CHUNKS);

                            const CHUNK_VERTEX_OFFSET: f32 = VOXEL_SIZE * CHUNK_SIZE as f32;

                            let values_in_vertex = greedy_mesh.vertex_size;
                            greedy_mesh.vertices.par_iter_mut().enumerate().for_each(
                                |(i, mut vert)| {
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
                                },
                            );
                        }

                        mesh.lod_scale = scaling_factor;
                        mesh.mesh = greedy_mesh;
                    }
                });
        }
        self.first_frame = false;
    }
}
