use crate::cb_math;
use cb_math::pow;

pub const CHUNK_SIZE: usize = 16;
const MAX_CHUNK_INDEX_3d: usize = CHUNK_SIZE - 1;

type COORDINATE = (usize, usize, usize);
/// Voxels are stored in a 1d array, even though they can be referenced within a 3d array
pub const CHUNK_SIZE_1D_ARRAY: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

#[derive(Debug, Copy, Clone)]
pub enum CbVoxelTypes {
    Default,
    Grass,
    Dirt,
    Water,
    Stone,
    Wood,
    Sand,
    Metal,
}

#[derive(Debug, Copy, Clone)]
pub struct CbVoxel {
    pub on: bool,
    pub voxel_type: CbVoxelTypes,
}

impl CbVoxel {
    pub fn new() -> Self {
        return CbVoxel {
            on: true,
            voxel_type: CbVoxelTypes::Default,
        };
    }
}

#[derive(Debug, Clone)]
pub struct CbVoxelChunk {
    pub voxels: Vec<(COORDINATE, CbVoxel)>,
}

impl CbVoxelChunk {
    pub fn new() -> Self {
        let mut voxels = vec![];

        voxels.reserve(CHUNK_SIZE_1D_ARRAY);
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let coordinate = (x, y, z);

                    voxels.push((coordinate, CbVoxel::new()));
                }
            }
        }

        return Self { voxels: voxels };
    }

    pub fn voxel_1d_to_3d(i: usize) -> COORDINATE {
        // i % max_x
        let x = i % MAX_CHUNK_INDEX_3d;
        // (i / max_x) % max_y
        let y = (i / MAX_CHUNK_INDEX_3d) % MAX_CHUNK_INDEX_3d;
        // i / (max_x * max_y)
        let z = i / (MAX_CHUNK_INDEX_3d * MAX_CHUNK_INDEX_3d);

        return (x, y, z);
    }

    pub fn voxel_3d_index(&self, x: usize, y: usize, z: usize) -> &CbVoxel {
        // i = x + y * max_x + z * max_x * max_y
        let i = x + y * MAX_CHUNK_INDEX_3d + z * MAX_CHUNK_INDEX_3d * MAX_CHUNK_INDEX_3d;

        return &self.voxels[i].1;
    }
}
