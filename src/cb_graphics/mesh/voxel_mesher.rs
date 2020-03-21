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
            let mut mesh_index = 0;

            for xchunk in 0..chunk_manager.get_chunk_width() {
                for ychunk in 0..chunk_manager.get_chunk_width() {
                    for zchunk in 0..chunk_manager.get_chunk_width() {
                        let mesh = &mut self.meshes[mesh_index];
                        let chunk = &chunk_manager.chunk_array[xchunk][ychunk][zchunk];

                        // Calculate lod to scale at; note: should be done by comparing to camera position
                        let chunk_updated = chunk.frame_updated_at >= mesh.mesh.generated_at_frame;

                        if chunk_updated || first_frame {
                            let mut greedy_mesh = calculate_greedy_mesh(
                                &chunk.voxels,
                                xchunk,
                                ychunk,
                                zchunk,
                                frame,
                                CHUNK_SIZE / 1,
                            );

                            //TODO: scale voxels based on chunks?

                            mesh.lod_scale = 1;
                            mesh.mesh = greedy_mesh;
                        }

                        mesh_index += 1;
                    }
                }
            }
        }

        self.first_frame = false;
    }
}
