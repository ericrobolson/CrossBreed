// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

use crate::cb_math;
use cb_math::{index_1d_to_3d, index_3d_to_1d};

extern crate rayon;
use rayon::prelude::*;

use time::{Duration, Instant};

// NOTE: Voxel size is about 1 foot
// Human is about 6ft, or 6 voxels

pub const CHUNK_SIZE: usize = 4;
pub const CHUNK_SIZE_SQUARED: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_SIZE_CUBED: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const MAX_CHUNK_INDEX: usize = CHUNK_SIZE - 1;

pub const VOXEL_SIZE: f32 = 1.0;

pub const CHUNKS: usize = 4;
pub const CHUNKS_SQUARED: usize = CHUNKS * CHUNKS;
pub const CHUNKS_CUBED: usize = CHUNKS * CHUNKS * CHUNKS;

/// (Active, Visible, Type, MiscValues)
/// A voxel contains four pieces of info, whether it's active, whether it is visible, it's type, and some misc values that may be used for different voxel types (such as health remaining on a block).
pub type CbVoxel = (bool, bool, u8, u8);

#[derive(Debug, Clone)]
pub struct CbChunkManager {
    dirty: bool,
    randomizer_index: usize,
    pub chunks: Vec<CbVoxelChunk>,
}

impl CbChunkManager {
    pub fn new() -> Self {
        let mut chunks: Vec<CbVoxelChunk> = (0..CHUNKS_CUBED)
            .collect::<Vec<usize>>()
            .par_iter()
            .enumerate()
            .map(|(i, _)| {
                return CbVoxelChunk::new();
            })
            .collect();

        let noise = cb_math::Noise::new(CHUNK_SIZE);

        return Self {
            randomizer_index: 0,
            chunks: chunks,
            dirty: true,
        };
    }

    pub fn get_voxel_width(&self) -> usize {
        return CHUNKS * CHUNK_SIZE;
    }

    pub fn get_voxel_mut(&mut self, x: usize, y: usize, z: usize, frame: usize) -> &mut CbVoxel {
        // NOTE: only developing single chunks for simplicity right now

        // Get the proper chunk index
        let chunk_x = x % CHUNKS;
        let chunk_y = y % CHUNKS;
        let chunk_z = z % CHUNKS;

        let chunk_index = index_3d_to_1d(chunk_x, chunk_y, chunk_z, CHUNKS);

        // Get the proper voxel index; NOTE: Need to figure out what to do when chunk_ * CHUNK_SIZE > value
        let voxel_x;
        {
            let voxels_to_remove = chunk_x * CHUNK_SIZE;

            if voxels_to_remove > x {
                voxel_x = 0;
            } else {
                voxel_x = x - voxels_to_remove;
            }
        }
        let voxel_y;
        {
            let voxels_to_remove = chunk_y * CHUNK_SIZE;

            if voxels_to_remove > y {
                voxel_y = 0;
            } else {
                voxel_y = y - voxels_to_remove;
            }
        }

        let voxel_z;
        {
            let voxels_to_remove = chunk_z * CHUNK_SIZE;

            if voxels_to_remove > z {
                voxel_z = 0;
            } else {
                voxel_z = z - voxels_to_remove;
            }
        }

        let voxel_index = index_3d_to_1d(voxel_x, voxel_y, voxel_z, CHUNK_SIZE);

        self.chunks[chunk_index].frame_updated_at = frame;

        return &mut self.chunks[chunk_index].voxel_vec[voxel_index];
    }

    pub fn randomize(&mut self, tick: usize) {
        self.chunks
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, chunk)| {
                chunk.frame_updated_at = tick;
                let (chunk_x, chunk_y, chunk_z) = index_1d_to_3d(i, CHUNKS, CHUNKS);

                chunk.voxel_vec.iter_mut().enumerate().for_each(
                    |(i, (voxel_active, _, voxel_type, _))| {
                        let (voxel_x, voxel_y, voxel_z) = index_1d_to_3d(i, CHUNK_SIZE, CHUNK_SIZE);
                        if (voxel_x + voxel_y + voxel_z + tick) % 7 == 0 {
                            *voxel_active = !*voxel_active;
                        }
                    },
                );
            });
    }
}

pub const VOXEL_TYPE_DEFAULT: u8 = 0;
pub const VOXEL_TYPE_GRASS: u8 = 1;
pub const VOXEL_TYPE_DIRT: u8 = 2;

#[derive(Debug, Clone)]
pub struct CbVoxelChunk {
    pub frame_updated_at: usize,
    pub voxel_vec: Vec<CbVoxel>,
}

impl CbVoxelChunk {
    pub fn new() -> Self {
        let voxel_vec = (0..CHUNK_SIZE_CUBED)
            .collect::<Vec<usize>>()
            .iter()
            .map(|i| {
                return (false, true, VOXEL_TYPE_GRASS, 0);
            })
            .collect();

        let mut chunk = Self {
            frame_updated_at: 0,
            voxel_vec: voxel_vec,
        };
        return chunk;
    }
}
