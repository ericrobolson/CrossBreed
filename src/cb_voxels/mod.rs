use crate::cb_math;
use cb_math::pow;

extern crate nalgebra as na;
use na::{Isometry3, Perspective3, Point3, Vector3};

pub const CHUNK_SIZE: usize = 16;
pub const MAX_CHUNK_INDEX: usize = CHUNK_SIZE - 1;

pub const VOXEL_SIZE: i32 = 1;

type COORDINATE = (usize, usize, usize);
/// Voxels are stored in a 1d array, even though they can be referenced within a 3d array
pub const CHUNK_SIZE_1D_ARRAY: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub struct CbChunkManager {}

impl CbChunkManager {
    pub fn new() -> Self {
        return Self {};
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
    mesh: Option<Vec<Mesh>>,
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

                    if x < CHUNK_SIZE / 2 && y < CHUNK_SIZE / 2 && z < CHUNK_SIZE / 2 {
                        voxel.voxel_type = CbVoxelTypes::Grass;
                    }

                    if x % 2 == 0 && y % 2 == 0 && z % 2 == 0 {
                        voxel.voxel_type = CbVoxelTypes::Dirt;
                    }

                    if x % 3 == 0 && y % 3 == 0 && z % 3 == 0 {
                        voxel.active = false;
                    }

                    voxels.push((coordinate, voxel));
                }
            }
        }

        let mut chunk = Self {
            voxels: voxels,
            dirty: true,
            mesh: None,
        };

        chunk.mesh();

        return chunk;
    }

    pub fn mesh(&mut self) -> &Vec<Mesh> {
        if !self.dirty {
            return self.mesh.as_ref().unwrap();
        }

        self.dirty = false;
        let mesh = self.calculate_greedy_mesh();

        self.mesh = Some(mesh);

        return self.mesh.as_ref().unwrap();
    }

    pub fn get_last_mesh(&self) -> &Vec<Mesh> {
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
        //NOTE: if adding culling, it would happen here
        // NOTE: Add the following here:
        // ** Set per face / per vertex values as well as voxel values here.
        let voxel = self.voxel_3d_index(x, y, z);

        let mut transparent = !voxel.active;

        // Check neighbors to see if obscured and cull if so
        if (x != 0 && x != MAX_CHUNK_INDEX)
            && (y != 0 && y != MAX_CHUNK_INDEX)
            && (z != 0 && z != MAX_CHUNK_INDEX)
        {
            // above layer
            let obscured_above = self.voxel_3d_index(x, y, z - 1).active;
            // same layer
            let obscured_same = self.voxel_3d_index(x, y + 1, z).active
                && self.voxel_3d_index(x, y - 1, z).active
                && self.voxel_3d_index(x + 1, y, z).active
                && self.voxel_3d_index(x - 1, y, z).active;
            // below layer
            let obscured_below = self.voxel_3d_index(x, y, z + 1).active;

            if !transparent && obscured_above && obscured_same && obscured_below {
                transparent = true;
            }
        }

        return VoxelFace {
            transparent: transparent,
            vf_type: voxel.voxel_type,
            side: side,
        };
    }

    fn calculate_greedy_mesh(&self) -> Vec<Mesh> {
        const CHUNK_WIDTH: usize = CHUNK_SIZE;
        const CHUNK_WIDTH_I: i32 = CHUNK_WIDTH as i32;
        const CHUNK_HEIGHT: usize = CHUNK_SIZE;
        const CHUNK_HEIGHT_I: i32 = CHUNK_HEIGHT as i32;

        let mut meshes = vec![];

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

                                    let x0 = x[0] as f32;
                                    let x1 = x[1] as f32;
                                    let x2 = x[2] as f32;

                                    let du0 = du[0] as f32;
                                    let du1 = du[1] as f32;
                                    let du2 = du[2] as f32;

                                    let dv0 = dv[0] as f32;
                                    let dv1 = dv[1] as f32;
                                    let dv2 = dv[2] as f32;

                                    // Call the quad() to render the merged quad in the scene. mask[n] will contain the attributes to pass to shaders
                                    let quad = get_quad(
                                        Vector3::new(x0, x1, x2),
                                        Vector3::new(x0 + du0, x1 + du1, x2 + du2),
                                        Vector3::new(
                                            x0 + du0 + dv0,
                                            x1 + du1 + dv1,
                                            x2 + du2 + dv2,
                                        ),
                                        Vector3::new(x0 + dv0, x1 + dv1, x2 + dv2),
                                        w,
                                        h,
                                        mask[n].unwrap(),
                                        backface,
                                    );

                                    meshes.push(quad);
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

        return meshes;
    }
}

type V3 = nalgebra::Matrix<
    f32,
    nalgebra::U3,
    nalgebra::U1,
    nalgebra::ArrayStorage<f32, nalgebra::U3, nalgebra::U1>,
>;

const SOUTH: usize = 0;
const NORTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;
const TOP: usize = 4;
const BOTTOM: usize = 5;

fn get_quad(
    bottom_left: V3,
    top_left: V3,
    top_right: V3,
    bottom_right: V3,
    width: usize,
    height: usize,
    voxel: VoxelFace,
    backface: bool,
) -> Mesh {
    let voxel_size = VOXEL_SIZE as f32 / 2.0; //TODO: change this

    const VALUES_IN_VERTEX: usize = 3;
    let vertices;
    let indices;
    {
        vertices = vec![
            // ----
            bottom_left.x,
            bottom_left.y,
            bottom_left.z,
            // ----
            bottom_right.x,
            bottom_right.y,
            bottom_right.z,
            // ----
            top_left.x,
            top_left.y,
            top_left.z,
            // ----
            top_right.x,
            top_right.y,
            top_right.z,
        ];
        if backface {
            indices = vec![
                2, 0, 1, //
                1, 3, 2, //
            ];
        } else {
            indices = vec![
                2, 3, 1, //
                1, 0, 2, //
            ];
        }
    }

    let vertices = vertices.iter().map(|n| n * voxel_size).collect();

    // Colors
    const COLOR_CAPACITY: usize = 9;
    const COLOR_VERTEX_SIZE: usize = 3;

    let mut colors;
    {
        colors = Vec::with_capacity(COLOR_CAPACITY);
        let mut i = 0;
        while i < COLOR_CAPACITY {
            match voxel.vf_type {
                CbVoxelTypes::Default => {
                    colors.push(1.0);
                    colors.push(0.0);
                    colors.push(0.0);
                }
                CbVoxelTypes::Dirt => {
                    colors.push(0.23);
                    colors.push(0.168);
                    colors.push(0.086);
                }
                CbVoxelTypes::Grass => {
                    colors.push(0.0);
                    colors.push(1.0);
                    colors.push(0.0);
                }
                _ => {
                    colors.push(0.0);
                    colors.push(0.0);
                    colors.push(1.0);
                }
            }

            i += COLOR_VERTEX_SIZE;
        }
    }

    let mesh = Mesh::new(
        VALUES_IN_VERTEX,
        vertices,
        indices,
        COLOR_VERTEX_SIZE,
        colors,
    );

    //mesh.wire_frame = true;
    return mesh;
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

/*
    TODO: move into gfx land
*/

#[derive(Debug, Clone)]
pub struct Mesh {
    pub indices: Vec<i32>,
    /// The number of values each vertex is composed of. Can be 1, 2, 3, or 4. TODO: make this some sort of static, fixed thing.
    pub vertex_size: usize,
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub color_vertex_size: usize,
    pub wire_frame: bool,
}

pub type CbMatrix = std::vec::Vec<
    nalgebra::Matrix<
        f32,
        nalgebra::U3,
        nalgebra::U1,
        nalgebra::ArrayStorage<f32, nalgebra::U3, nalgebra::U1>,
    >,
>;

impl Mesh {
    pub fn new(
        vertex_size: usize,
        vertices: Vec<f32>,
        indices: Vec<i32>,
        color_vertex_size: usize,
        colors: Vec<f32>,
    ) -> Self {
        return Self {
            vertex_size: vertex_size,
            vertices: vertices,
            indices: indices,
            color_vertex_size: color_vertex_size,
            colors: colors,
            wire_frame: false,
        };
    }

    pub fn merge(meshes: Vec<Mesh>) -> Mesh {
        let mut mesh = Mesh::new(0, vec![], vec![], 0, vec![]);

        if meshes.is_empty() == false {
            let mut is_first = true;
            for m in meshes.iter() {
                let start_vertex_index = mesh.vertices.len() as i32;

                if is_first {
                    mesh.vertex_size = m.vertex_size;
                    mesh.color_vertex_size = m.color_vertex_size;
                    is_first = false;
                }

                if mesh.vertex_size != m.vertex_size {
                    panic!("Unable to merge meshes! Mesh vertex sizes differ.");
                }

                if mesh.color_vertex_size != m.color_vertex_size {
                    panic!("Unable to merge meshes! Mesh color vertex sizes differ.");
                }

                mesh.vertices.append(&mut m.vertices.clone());
                mesh.colors.append(&mut m.colors.clone());

                // do tricky shit with indices
                for index in m.indices.iter() {
                    mesh.indices.push(index + start_vertex_index);
                }
            }
        }

        return mesh;
    }
}
