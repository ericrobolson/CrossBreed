use crate::cb_math;
use cb_math::index_1d_to_3d;

extern crate rayon;
use rayon::prelude::*;

use time::{Duration, Instant};

// NOTE: Voxel size is about 1 foot
// Human is about 6ft, or 6 voxels

extern crate rand;
use rand::Rng; // TODO: replace with deterministic one

pub const CHUNK_SIZE: usize = 64;
pub const CHUNK_SIZE_SQUARED: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_SIZE_CUBED: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const MAX_CHUNK_INDEX: usize = CHUNK_SIZE - 1;

pub const VOXEL_SIZE: f32 = 1.0;

pub const CHUNKS: usize = 1;
pub const CHUNKS_SQUARED: usize = CHUNKS * CHUNKS;
pub const CHUNKS_CUBED: usize = CHUNKS * CHUNKS * CHUNKS;

/// A voxel contains three pieces of info, whether it's active, it's type, and some misc values that may be used for different voxel types (such as health remaining on a block).
pub type CbVoxel = (bool, u8, u8);

#[derive(Debug)]
pub struct CbChunkManager {
    dirty: bool,
    randomizer_index: usize,
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

        println!("fin init");
        println!("landscape begin");

        let noise = cb_math::Noise::new(CHUNK_SIZE);

        chunks.par_iter_mut().enumerate().for_each(|(i, chunk)| {
            let (chunk_x, chunk_y, chunk_z) = index_1d_to_3d(i, CHUNKS, CHUNKS);

            chunk.voxel_vec.iter_mut().enumerate().for_each(
                |(i, (voxel_active, voxel_type, _))| {
                    let (voxel_x, voxel_y, voxel_z) = index_1d_to_3d(i, CHUNK_SIZE, CHUNK_SIZE);
                    if voxel_y % 29 == 0 && voxel_x % 3 == 0 {
                        *voxel_active = true;
                    }
                },
            );
        });
        let end = Instant::now() - start;

        println!("Chunks created in: {:?}", end);
        return Self {
            randomizer_index: 0,
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
    pub frame_updated_at: usize,
    pub voxel_vec: Vec<CbVoxel>,
}

impl CbVoxelChunk {
    pub fn new() -> Self {
        let voxel_vec = (0..CHUNK_SIZE_CUBED)
            .collect::<Vec<usize>>()
            .iter()
            .map(|i| {
                return (false, VOXEL_TYPE_GRASS, 0);
            })
            .collect();

        let mut chunk = Self {
            frame_updated_at: 0,
            voxel_vec: voxel_vec,
        };
        return chunk;
    }
}
