use super::*;

extern crate nalgebra as na;
use na::Vector3;

use crate::cb_graphics::mesh;
use mesh::Mesh;

fn get_voxel_face(chunk: &CbVoxelChunk, x: usize, y: usize, z: usize, side: usize) -> VoxelFace {
    // NOTE: Add the following here:
    // ** Set per face / per vertex values as well as voxel values here.
    let voxel = chunk.voxels[x][y][z];

    let mut transparent = !voxel.active;

    // Check neighbors to see if obscured and cull if so
    if (x != 0 && x != MAX_CHUNK_INDEX)
        && (y != 0 && y != MAX_CHUNK_INDEX)
        && (z != 0 && z != MAX_CHUNK_INDEX)
    {
        // above layer
        let obscured_above = chunk.voxels[x][y][z - 1].active;
        // same layer
        let obscured_same = chunk.voxels[x][y + 1][z].active
            && chunk.voxels[x][y - 1][z].active
            && chunk.voxels[x + 1][y][z].active
            && chunk.voxels[x - 1][y][z].active;
        // below layer
        let obscured_below = chunk.voxels[x][y][z + 1].active;

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

pub fn calculate_greedy_mesh(chunk: &CbVoxelChunk, frame: usize) -> Mesh {
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
                                voxel_face = Some(get_voxel_face(
                                    chunk,
                                    x[0] as usize,
                                    x[1] as usize,
                                    x[2] as usize,
                                    side,
                                ));
                            } else {
                                voxel_face = None;
                            }

                            if x[d] < CHUNK_WIDTH_I - 1 {
                                voxel_face1 = Some(get_voxel_face(
                                    chunk,
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
                                    Vector3::new(x0 + du0 + dv0, x1 + du1 + dv1, x2 + du2 + dv2),
                                    Vector3::new(x0 + dv0, x1 + dv1, x2 + dv2),
                                    w,
                                    h,
                                    mask[n].unwrap(),
                                    backface,
                                    frame,
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

    return Mesh::merge(meshes);
}

type V3 = nalgebra::Matrix<
    f32,
    nalgebra::U3,
    nalgebra::U1,
    nalgebra::ArrayStorage<f32, nalgebra::U3, nalgebra::U1>,
>;

fn get_quad(
    bottom_left: V3,
    top_left: V3,
    top_right: V3,
    bottom_right: V3,
    width: usize,
    height: usize,
    voxel: VoxelFace,
    backface: bool,
    generated_at_frame: usize,
) -> Mesh {
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

    let vertices = vertices.iter().map(|n| n * VOXEL_SIZE).collect();

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
                CbVoxelTypes::Water => {
                    colors.push(0.0);
                    colors.push(0.0);
                    colors.push(1.0);
                }
                CbVoxelTypes::Stone => {
                    colors.push(0.0);
                    colors.push(0.0);
                    colors.push(1.0);
                }
                CbVoxelTypes::Wood => {
                    colors.push(0.0);
                    colors.push(0.0);
                    colors.push(1.0);
                }
                CbVoxelTypes::Sand => {
                    colors.push(0.0);
                    colors.push(0.0);
                    colors.push(1.0);
                }
                CbVoxelTypes::Metal => {
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
        generated_at_frame,
    );
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
