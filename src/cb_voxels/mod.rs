use crate::cb_graphics::mesh;
use mesh::Mesh;

mod greedy_mesher;

extern crate rayon;
use rayon::prelude::*;

// NOTE: Voxel size is about 6 inches
// Human is about 6ft, or 12 voxels

pub const CHUNK_SIZE: usize = 32;
pub const MAX_CHUNK_INDEX: usize = CHUNK_SIZE - 1;

pub const VOXEL_SIZE: f32 = 0.05;

pub const CHUNKS: usize = 8;
pub const NUM_CHUNKS: usize = CHUNKS * CHUNKS * CHUNKS;

#[derive(Debug)]
pub struct CbChunkManager {
    gen_mesh: Option<Mesh>,
    dirty: bool,
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
                return foo;
            })
            .collect();

        println!("Chunks created");
        let mut manager = Self {
            gen_mesh: None,
            chunks: chunks,
            dirty: true,
        };
        println!("meshing begin");
        manager.mesh(0);
        println!("meshing end");
        return manager;
    }

    fn init_chunk_slice(i: usize) -> Vec<CbVoxelChunk> {
        let mut range = vec![];
        for _ in 0..CHUNKS {
            range.push(false);
        }

        let chunks = range.par_iter().map(|_| CbVoxelChunk::new()).collect();

        return chunks;
    }

    pub fn mesh(&mut self, frame: usize) {
        if self.dirty == false {
            return;
        }
        self.dirty = false;
        self.chunks
            .par_iter_mut()
            .flatten()
            .flatten()
            .for_each(|chunk| {
                chunk.mesh(frame);
            });
        let mut meshes = vec![];

        for (ix, x) in self.chunks.iter().enumerate() {
            for (iy, y) in x.iter().enumerate() {
                for (iz, chunk) in y.iter().enumerate() {
                    let mut mesh = chunk.get_last_mesh().clone();

                    // Modify the offsets so that the mesh is done properly
                    const X_VALUE: usize = 0;
                    const Y_VALUE: usize = 1;
                    const Z_VALUE: usize = 2;

                    const CHUNK_VERTEX_OFFSET: f32 = VOXEL_SIZE * CHUNK_SIZE as f32;

                    const VALUES_IN_VERTEX: usize = 3;
                    mesh.vertices
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, mut vert)| {
                            let value_type = i % VALUES_IN_VERTEX;

                            if value_type == X_VALUE {
                                // add modified x offset
                                *vert += CHUNK_VERTEX_OFFSET * ix as f32;
                            } else if value_type == Y_VALUE {
                                // add modified y offset
                                *vert += CHUNK_VERTEX_OFFSET * iy as f32;
                            } else if value_type == Z_VALUE {
                                // add modified z offset
                                *vert += CHUNK_VERTEX_OFFSET * iz as f32;
                            }
                        });

                    meshes.push(mesh);
                }
            }
        }

        let mut mesh = Mesh::merge(meshes);
        self.gen_mesh = Some(mesh);
    }

    pub fn last_mesh(&self) -> Mesh {
        return self.gen_mesh.as_ref().unwrap().clone();
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
    pub top_empty: bool,
    pub bottom_empty: bool,
    pub north_empty: bool,
    pub south_empty: bool,
    pub east_empty: bool,
    pub west_empty: bool,
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
            top_empty: false,
            bottom_empty: false,
            north_empty: false,
            south_empty: false,
            east_empty: false,
            west_empty: false,
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

        //TODO: calculate if top, bot, N, S, E, W are empty; used for determining whether to mesh chunks or not

        return self.mesh.as_ref().unwrap();
    }

    pub fn get_last_mesh(&self) -> &Mesh {
        return self.mesh.as_ref().unwrap();
    }
}
