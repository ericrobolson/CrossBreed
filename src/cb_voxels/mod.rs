use crate::cb_math;
use cb_math::pow;

pub const CHUNK_SIZE: usize = 4;
pub const MAX_CHUNK_INDEX: usize = CHUNK_SIZE - 1;
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

///TODO: move into gfx land
#[derive(Debug, Clone)]
pub struct Mesh {}

impl Mesh {
    pub fn new() -> Self {
        return Self {};
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CbVoxel {
    pub active: bool,
    pub voxel_type: CbVoxelTypes,
}

impl CbVoxel {
    pub fn new() -> Self {
        return CbVoxel {
            active: true,
            voxel_type: CbVoxelTypes::Default,
        };
    }
}

#[derive(Debug, Clone)]
pub struct CbVoxelChunk {
    dirty: bool,
    mesh: Option<Mesh>,
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
                    let mut voxel = CbVoxel::new();

                    if (x + y + z) % 2 == 0 {
                        //    voxel.active = false;
                    }
                    voxels.push((coordinate, voxel));
                }
            }
        }

        return Self {
            voxels: voxels,
            dirty: true,
            mesh: None,
        };
    }

    pub fn mesh(&mut self) -> &Mesh {
        if !self.dirty {
            // return previously calculated mesh
        }

        self.dirty = false;
        let mesh = Mesh::new();

        self.mesh = Some(mesh);

        return self.mesh.as_ref().unwrap();
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
