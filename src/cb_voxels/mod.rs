use crate::cb_graphics::mesh;
use mesh::Mesh;

mod greedy_mesher;

extern crate rayon;
use rayon::prelude::*;

pub const CHUNK_SIZE: usize = 32;
pub const MAX_CHUNK_INDEX: usize = CHUNK_SIZE - 1;

pub const VOXEL_SIZE: f32 = 0.5;

pub const CHUNKS: usize = 8;
pub const NUM_CHUNKS: usize = CHUNKS * CHUNKS * CHUNKS;

#[derive(Debug)]
pub struct CbChunkManager {
    pub chunks: Vec<Vec<Vec<CbVoxelChunk>>>,
}

impl CbChunkManager {
    pub fn new() -> Self {
        println!("Chunks init");

        let mut range = vec![];
        for _ in 0..CHUNKS {
            range.push(false);
        }

        let chunks = range
            .par_iter()
            .enumerate()
            .map(|(i, _)| {
                let mut range2 = vec![];
                for _ in 0..CHUNKS {
                    range2.push(false);
                }

                let foo = range2
                    .par_iter()
                    .enumerate()
                    .map(|(i, _)| CbChunkManager::init_chunk_slice(i))
                    .collect();

                println!("** chunk set finished: {}", i);

                return foo;
            })
            .collect();

        println!("Chunks created");
        let mut manager = Self { chunks: chunks };
        manager.mesh(0);
        return manager;
    }

    fn init_chunk_slice(i: usize) -> Vec<CbVoxelChunk> {
        let mut range = vec![];
        for _ in 0..CHUNKS {
            range.push(false);
        }

        let chunks = range.par_iter().map(|_| CbVoxelChunk::new()).collect();

        println!("* {}: chunk slice finished", i);

        return chunks;
    }

    pub fn mesh(&mut self, frame: usize) {
        self.chunks
            .par_iter_mut()
            .flatten()
            .flatten()
            .for_each(|chunk| {
                chunk.mesh(frame);

                println!("finished chunk");
            });
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
    pub voxels: std::boxed::Box<[[[CbVoxel; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}

impl CbVoxelChunk {
    pub fn new() -> Self {
        let mut voxel = CbVoxel::new();
        voxel.active = true;
        voxel.voxel_type = CbVoxelTypes::Grass;

        let voxels = Box::new([[[voxel; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);

        let mut chunk = Self {
            voxels: voxels,
            dirty: true,
            mesh: None,
        };

        chunk.init_landscape();

        return chunk;
    }

    pub fn init_landscape(&mut self) {
        for y in 0..CHUNK_SIZE {
            for x in 0..y {
                for z in 0..y {
                    if y % 2 == 0 {
                        self.voxels[x][y][z].voxel_type = CbVoxelTypes::Dirt;
                    } else {
                        self.voxels[x][y][z].active = false;
                    }
                }
            }
        }
    }

    pub fn mesh(&mut self, frame: usize) -> &Mesh {
        if !self.dirty {
            return self.mesh.as_ref().unwrap();
        }

        // Remesh and store it
        self.dirty = false;
        let mesh = greedy_mesher::calculate_greedy_mesh(&self, frame);
        self.mesh = Some(mesh);

        return self.mesh.as_ref().unwrap();
    }

    pub fn get_last_mesh(&self) -> &Mesh {
        return self.mesh.as_ref().unwrap();
    }
}
