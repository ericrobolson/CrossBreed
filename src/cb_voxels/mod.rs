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

pub const VOXEL_SIZE: f32 = 1.0; //TODO: change from float

pub const CHUNKS: usize = 4;
pub const CHUNKS_SQUARED: usize = CHUNKS * CHUNKS;
pub const CHUNKS_CUBED: usize = CHUNKS * CHUNKS * CHUNKS;

/// (Active, Visible, Type, MiscValues)
/// A voxel contains four pieces of info, whether it's active, whether it is visible, it's type, and some misc values that may be used for different voxel types (such as health remaining on a block).
pub type CbVoxel = (bool, bool, u8, u8);

pub fn voxel_active(voxel: &CbVoxel) -> bool {
    return voxel.0;
}

#[derive(Debug, Clone)]
pub struct CbChunkManager {
    dirty: bool,
    randomizer_index: usize,

    // End goal: Convert to using a 1d array, right now it's just too annoying and bug prone
    pub chunk_array: [[[CbVoxelChunk; CHUNKS]; CHUNKS]; CHUNKS],
}

impl CbChunkManager {
    pub fn new() -> Self {
        let noise = cb_math::Noise::new(CHUNK_SIZE);

        return Self {
            chunk_array: [[[CbVoxelChunk::new(); CHUNKS]; CHUNKS]; CHUNKS],
            randomizer_index: 0,
            dirty: true,
        };
    }

    pub fn get_chunk_width(&self) -> usize {
        return CHUNKS;
    }

    pub fn get_voxel_count_per_chunk(&self) -> usize {
        return CHUNK_SIZE;
    }

    pub fn get_voxel_width(&self) -> usize {
        return self.get_chunk_width() * self.get_voxel_count_per_chunk();
    }

    pub fn get_voxel(&self, x: usize, y: usize, z: usize) -> &CbVoxel {
        let ((cx, cy, cz), (vx, vy, vz)) = get_chunk_and_voxel_indexes(x, y, z, CHUNKS, CHUNK_SIZE);

        return &self.chunk_array[cx][cy][cz].voxels[vx][vy][vz];
    }

    pub fn get_voxel_mut(&mut self, x: usize, y: usize, z: usize, frame: usize) -> &mut CbVoxel {
        let ((cx, cy, cz), (vx, vy, vz)) = get_chunk_and_voxel_indexes(x, y, z, CHUNKS, CHUNK_SIZE);

        self.chunk_array[cx][cy][cz].frame_updated_at = frame;

        return &mut self.chunk_array[cx][cy][cz].voxels[vx][vy][vz];
    }
}

pub const VOXEL_TYPE_DEFAULT: u8 = 0;
pub const VOXEL_TYPE_GRASS: u8 = 1;
pub const VOXEL_TYPE_DIRT: u8 = 2;

#[derive(Debug, Copy, Clone)]
pub struct CbVoxelChunk {
    pub frame_updated_at: usize,
    pub voxels: [[[CbVoxel; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl CbVoxelChunk {
    pub fn new() -> Self {
        let mut chunk = Self {
            voxels: [[[(true, true, VOXEL_TYPE_GRASS, 0); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],

            frame_updated_at: 0,
        };
        return chunk;
    }
}

fn get_chunk_and_voxel_index(i: usize, num_chunks: usize, chunk_size: usize) -> (usize, usize) {
    if i < chunk_size {
        return (0, i);
    } else {
        let chunk_index = i / chunk_size;
        let voxel_index = i - (chunk_index * chunk_size);
        return (chunk_index, voxel_index);
    }
}

fn get_chunk_and_voxel_indexes(
    x: usize,
    y: usize,
    z: usize,
    num_chunks: usize,
    chunk_size: usize,
) -> ((usize, usize, usize), (usize, usize, usize)) {
    let (chunk_x, voxel_x) = get_chunk_and_voxel_index(x, num_chunks, chunk_size);
    let (chunk_y, voxel_y) = get_chunk_and_voxel_index(y, num_chunks, chunk_size);
    let (chunk_z, voxel_z) = get_chunk_and_voxel_index(z, num_chunks, chunk_size);

    return ((chunk_x, chunk_y, chunk_z), (voxel_x, voxel_y, voxel_z));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_chunk_and_voxel_indexes_x0y4z0_n2c4_returns_returns_expected() {
        let x = 0;
        let y = 4;
        let z = 0;
        let num_chunks = 2;
        let chunk_size = 4;

        let actual = get_chunk_and_voxel_indexes(x, y, z, num_chunks, chunk_size);

        let expected = ((0, 1, 0), (0, 0, 0));

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_chunk_and_voxel_indexes_x4y0z0_n2c4_returns_returns_expected() {
        let x = 4;
        let y = 0;
        let z = 0;
        let num_chunks = 2;
        let chunk_size = 4;

        let actual = get_chunk_and_voxel_indexes(x, y, z, num_chunks, chunk_size);

        let expected = ((1, 0, 0), (0, 0, 0));

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_chunk_and_voxel_indexes_x0y0z4_n2c4_returns_expected() {
        let x = 0;
        let y = 0;
        let z = 4;
        let num_chunks = 2;
        let chunk_size = 4;

        let actual = get_chunk_and_voxel_indexes(x, y, z, num_chunks, chunk_size);

        let expected = ((0, 0, 1), (0, 0, 0));

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_chunk_and_voxel_indexes_x0y5z0_n2c4_returns_returns_expected() {
        let x = 0;
        let y = 5;
        let z = 0;
        let num_chunks = 2;
        let chunk_size = 4;

        let actual = get_chunk_and_voxel_indexes(x, y, z, num_chunks, chunk_size);

        let expected = ((0, 1, 0), (0, 1, 0));

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_chunk_and_voxel_indexes_x5y0z0_n2c4_returns_returns_expected() {
        let x = 5;
        let y = 0;
        let z = 0;
        let num_chunks = 2;
        let chunk_size = 4;

        let actual = get_chunk_and_voxel_indexes(x, y, z, num_chunks, chunk_size);

        let expected = ((1, 0, 0), (1, 0, 0));

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_chunk_and_voxel_indexes_x0y0z5_n2c4_returns_expected() {
        let x = 0;
        let y = 0;
        let z = 5;
        let num_chunks = 2;
        let chunk_size = 4;

        let actual = get_chunk_and_voxel_indexes(x, y, z, num_chunks, chunk_size);

        let expected = ((0, 0, 1), (0, 0, 1));

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_chunk_and_voxel_indexes_x0y6z0_n2c4_returns_expected() {
        let x = 0;
        let y = 6;
        let z = 0;
        let num_chunks = 2;
        let chunk_size = 4;

        let actual = get_chunk_and_voxel_indexes(x, y, z, num_chunks, chunk_size);

        let expected = ((0, 1, 0), (0, 2, 0));

        assert_eq!(expected, actual);
    }
}
