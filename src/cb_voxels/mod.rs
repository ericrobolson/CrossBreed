use crate::cb_math;
use cb_math::pow;

extern crate nalgebra as na;
use na::{Isometry3, Perspective3, Point3, Vector3};

pub const CHUNK_SIZE: usize = 8;
pub const MAX_CHUNK_INDEX: usize = CHUNK_SIZE - 1;

type COORDINATE = (usize, usize, usize);
/// Voxels are stored in a 1d array, even though they can be referenced within a 3d array
pub const CHUNK_SIZE_1D_ARRAY: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

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
        let mesh = self.calculate_greedy_mesh();

        self.mesh = Some(mesh);

        return self.mesh.as_ref().unwrap();
    }

    pub fn voxel_1d_to_3d(i: usize) -> COORDINATE {
        // i % max_x
        let x = i % CHUNK_SIZE;
        // (i / max_x) % max_y
        let y = (i / CHUNK_SIZE) % CHUNK_SIZE;
        // i / (max_x * max_y)
        let z = i / (CHUNK_SIZE * CHUNK_SIZE);

        return (x, y, z);
    }

    pub fn voxel_3d_index(&self, x: usize, y: usize, z: usize) -> &CbVoxel {
        // i = x + y * max_x + z * max_x * max_y
        let i = x + y * MAX_CHUNK_INDEX + z * MAX_CHUNK_INDEX * MAX_CHUNK_INDEX;

        return &self.voxels[i].1;
    }

    fn get_voxel_face(&self, x: usize, y: usize, z: usize, side: usize) -> VoxelFace {
        let voxel = self.voxel_3d_index(x, y, z);

        let transparent = !voxel.active;

        return VoxelFace {
            transparent: transparent,
            vf_type: voxel.voxel_type,
            side: side,
        };
    }

    fn calculate_greedy_mesh(&self) -> Mesh {
        const SOUTH: usize = 0;
        const NORTH: usize = 1;
        const EAST: usize = 2;
        const WEST: usize = 3;
        const TOP: usize = 4;
        const BOTTOM: usize = 5;

        const CHUNK_WIDTH: usize = CHUNK_SIZE;
        const CHUNK_WIDTH_I: i32 = CHUNK_WIDTH as i32;
        const CHUNK_HEIGHT: usize = CHUNK_SIZE;
        const CHUNK_HEIGHT_I: i32 = CHUNK_HEIGHT as i32;

        // Referenced https://github.com/roboleary/GreedyMesh/blob/master/src/mygame/Main.java

        // Create the working variables
        let_mut_for![(i, j, k, l, w, h, u, v, n, side), usize, 0];
        let_mut_for![(x, q, du, dv), Vector3<i32>, Vector3::new(0, 0, 0)];

        // Create a mask of matching voxel faces as we go through the chunk in 6 directions, once for each face
        let mut mask = Vec::with_capacity(CHUNK_WIDTH * CHUNK_HEIGHT);
        for _ in 0..CHUNK_WIDTH * CHUNK_HEIGHT {
            mask.push(None);
        }

        // Working variables to hold two faces during comparison
        let_mut_for![(voxel_face, voxel_face1), Option<VoxelFace>, None];

        let mut backface = false;

        // First loop run it with the backface, second loop run it without. This allows us to track the directions the indices should run during the creation of the quad.
        // Outer loop will run twice, inner loop 3 times. Once for each voxel face.
        for _ in 0..2 {
            backface = !backface;

            // Sweep over the 3 dimensions to mesh it.
            for d in 0..3 {
                // Set variables
                {
                    u = (d + 1) % 3;
                    v = (d + 2) % 3;

                    x[0] = 0;
                    x[1] = 0;
                    x[2] = 0;

                    q[0] = 0;
                    q[1] = 0;
                    q[2] = 0;
                    q[d] = 1;
                }

                // Keep track of the side that is being meshed.
                {
                    if d == 0 {
                        if backface {
                            side = WEST;
                        } else {
                            side = EAST;
                        }
                    } else if d == 1 {
                        if backface {
                            side = BOTTOM;
                        } else {
                            side = TOP;
                        }
                    } else if d == 2 {
                        if backface {
                            side = SOUTH;
                        } else {
                            side = NORTH;
                        }
                    }
                }

                // Move through the dimension from front to back
                x[d] = -1;
                while x[d] < CHUNK_WIDTH_I {
                    // Compute the mask
                    {
                        n = 0;

                        x[v] = 0;
                        while x[v] < CHUNK_HEIGHT_I {
                            x[u] = 0;
                            while x[u] < CHUNK_WIDTH_I {
                                // Retrieve the two voxel faces to compare.
                                if x[d] >= 0 {
                                    voxel_face = Some(self.get_voxel_face(
                                        x[0] as usize,
                                        x[1] as usize,
                                        x[2] as usize,
                                        side,
                                    ));
                                } else {
                                    voxel_face = None;
                                }

                                if x[d] < CHUNK_WIDTH_I - 1 {
                                    voxel_face1 = Some(self.get_voxel_face(
                                        (x[0] + q[0]) as usize,
                                        (x[1] + q[1]) as usize,
                                        (x[2] + q[2]) as usize,
                                        side,
                                    ));
                                } else {
                                    voxel_face1 = None;
                                }
                                // Compare the faces based on number of attributes. Choose the face to add to the mask depending on backface or not.
                                if voxel_face.is_some()
                                    && voxel_face1.is_some()
                                    && voxel_face.unwrap().equals(&voxel_face1.unwrap())
                                {
                                    mask[n] = None;
                                } else if backface {
                                    mask[n] = voxel_face1;
                                } else if !backface {
                                    mask[n] = voxel_face;
                                }

                                n += 1;
                                x[u] += 1;
                            }

                            x[v] += 1;
                        }
                    }

                    x[d] += 1;

                    // Now generate the mesh for the mask
                    n = 0;
                    j = 0;
                    while j < CHUNK_HEIGHT {
                        i = 0;
                        while i < CHUNK_WIDTH {
                            if mask[n].is_some() {
                                // Compute the width
                                w = 1;
                                while i + w < CHUNK_WIDTH
                                    && mask[n + w].is_some()
                                    && mask[n + w].unwrap().equals(&mask[n].unwrap())
                                {
                                    w += 1;
                                }

                                // Compute the height
                                let mut done = false;

                                h = 1;
                                while j + h < CHUNK_HEIGHT {
                                    k = 0;
                                    while k < w {
                                        if mask[n + k + h * CHUNK_WIDTH].is_none()
                                            || !mask[n + k + h * CHUNK_WIDTH]
                                                .unwrap()
                                                .equals(&mask[n].unwrap())
                                        {
                                            done = true;
                                            break;
                                        }
                                        k += 1;
                                    }

                                    if done {
                                        break;
                                    }
                                    h += 1;
                                }

                                // Do not mesh transparent/culled faces
                                if !mask[n].unwrap().transparent {
                                    // Add quad
                                    x[u] = i as i32;
                                    x[v] = j as i32;

                                    du[0] = 0;
                                    du[1] = 0;
                                    du[2] = 0;
                                    du[u] = w as i32;

                                    dv[0] = 0;
                                    dv[1] = 0;
                                    dv[2] = 0;
                                    dv[v] = h as i32;

                                    //TODO: replace with Quad function
                                    // Call the quad() to render the merged quad in the scene. mask[n] will contain the attributes to pass to shaders
                                    let quad = (
                                        Vector3::new(x[0], x[1], x[2]),
                                        Vector3::new(x[0] + du[0], x[1] + du[1], x[2] + du[2]),
                                        Vector3::new(
                                            x[0] + du[0] + dv[0],
                                            x[1] + du[1] + dv[1],
                                            x[2] + du[2] + dv[2],
                                        ),
                                        Vector3::new(x[0] + dv[0], x[1] + dv[1], x[2] + dv[2]),
                                        w,
                                        h,
                                        mask[n],
                                        backface,
                                    );
                                }

                                // Zero out the mask
                                l = 0;
                                while l < h {
                                    k = 0;
                                    while k < w {
                                        mask[n + k + l * CHUNK_WIDTH] = None;

                                        k += 1;
                                    }

                                    l += 1;
                                }

                                // Increment the counters + continue
                                i += w;
                                n += w;
                            } else {
                                i += 1;
                                n += 1;
                            }
                        }
                        j += 1;
                    }
                }
            }
        }

        // NOTE: THIS IS JUST DUMB CODE TO GET IT COMPILING - REMOVE
        mask.push(Some(VoxelFace {
            transparent: false,
            side: 0,
            vf_type: CbVoxelTypes::Default,
        }));

        return Mesh {};
        // END NOTE
    }
}

/// Struct used for meshing purposes
#[derive(Debug, Copy, Clone)]
struct VoxelFace {
    pub transparent: bool,
    pub vf_type: CbVoxelTypes,
    pub side: usize,
}

impl VoxelFace {
    fn equals(&self, other: &VoxelFace) -> bool {
        return self.transparent == other.transparent && self.vf_type == other.vf_type;
    }
}
