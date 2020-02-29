use crate::cb_math;
use cb_math::index_1d_to_3d;

extern crate rayon;
use rayon::prelude::*;

use time::{Duration, Instant};

// NOTE: Voxel size is about 6 inches
// Human is about 6ft, or 12 voxels

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_SIZE_SQUARED: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_SIZE_CUBED: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const MAX_CHUNK_INDEX: usize = CHUNK_SIZE - 1;

pub const VOXEL_SIZE: f32 = 0.05;

pub const CHUNKS: usize = 32;
pub const CHUNKS_SQUARED: usize = CHUNKS * CHUNKS;
pub const CHUNKS_CUBED: usize = CHUNKS * CHUNKS * CHUNKS;

/// A voxel contains two pieces of info, whether it's active, and it's type.
pub type CbVoxel = (bool, u8);

#[derive(Debug)]
pub struct CbChunkManager {
    dirty: bool,
    pub chunks: Vec<CbVoxelChunk>,
}

impl CbChunkManager {
    pub fn new() -> Self {
        println!("Chunks init");
        let start = Instant::now();

        let mut chunks: Vec<CbVoxelChunk> = (0..CHUNKS_CUBED)
            .collect::<Vec<usize>>()
            .par_iter()
            .enumerate()
            .map(|(i, _)| {
                return CbVoxelChunk::new();
            })
            .collect();

        let end = Instant::now() - start;

        println!("Chunks created in: {:?}", end);
        return Self {
            chunks: chunks,
            dirty: true,
        };
    }
}

pub const VOXEL_TYPE_DEFAULT: u8 = 0;
pub const VOXEL_TYPE_GRASS: u8 = 1;
pub const VOXEL_TYPE_DIRT: u8 = 2;

#[derive(Debug, Clone)]
pub struct CbVoxelChunk {
    dirty: bool,
    pub voxel_vec: Vec<CbVoxel>,
}

impl CbVoxelChunk {
    pub fn new() -> Self {
        let voxel_vec = (0..CHUNK_SIZE_CUBED)
            .collect::<Vec<usize>>()
            .iter()
            .map(|_| {
                return (true, VOXEL_TYPE_GRASS);
            })
            .collect();

        let mut chunk = Self {
            voxel_vec: voxel_vec,
            dirty: true,
        };
        return chunk;
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }
}
