pub mod greedy_mesher;
pub mod voxel_mesher;

extern crate rayon;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub indices: Vec<i32>,
    /// The number of values each vertex is composed of. Can be 1, 2, 3, or 4. TODO: make this some sort of static, fixed thing.
    pub vertex_size: usize,
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub color_vertex_size: usize,
    pub normals: Vec<f32>,
    pub normal_vertex_size: usize,
    pub wire_frame: bool,
    pub disable_polygon_smooth: bool,
    pub generated_at_frame: usize,
}

impl Mesh {
    pub fn new(
        vertex_size: usize,
        vertices: Vec<f32>,
        indices: Vec<i32>,
        color_vertex_size: usize,
        colors: Vec<f32>,
        normal_vertex_size: usize,
        normals: Vec<f32>,
        generated_at_frame: usize,
    ) -> Self {
        return Self {
            vertex_size: vertex_size,
            vertices: vertices,
            indices: indices,
            color_vertex_size: color_vertex_size,
            colors: colors,
            normal_vertex_size: normal_vertex_size,
            normals: normals,
            wire_frame: false,
            disable_polygon_smooth: false,
            generated_at_frame: generated_at_frame,
        };
    }

    pub fn merge(meshes: &Vec<Mesh>) -> Mesh {
        //TODO: paralellize
        let mut mesh = Mesh::new(3, vec![], vec![], 3, vec![], 3, vec![], 0);

        {
            /*
            let mut empty_mesh = Mesh::new(3, vec![], vec![], 3, vec![], 3, vec![], 0);

            let meshes = Vec::<Mesh>::new();

            let mesh = meshes
                .iter()
                .map(|x| Some(x))
                .collect::<Option<Mesh>>()
                .par_iter_mut()
                .reduce(
                    || None,
                    |a, b| {
                        if a.vertex_size != b.vertex_size {
                            panic!("Unable to merge meshes! Mesh vertex sizes differ.");
                        }

                        if a.normal_vertex_size != b.normal_vertex_size {
                            panic!("Unable to merge meshes! Mesh normal vertex sizes differ.");
                        }

                        if a.color_vertex_size != b.color_vertex_size {
                            panic!("Unable to merge meshes! Mesh color vertex sizes differ.");
                        }

                        a.vertices.append(&mut b.vertices.clone());
                        a.colors.append(&mut b.colors.clone());
                        a.normals.append(&mut b.normals.clone());

                        // do tricky shit with indices
                        let offset = b.vertices.len() as i32 / b.vertex_size as i32;

                        let mut mapped_indices = b.indices.iter().map(|i| i + offset).collect();

                        a.indices.append(&mut mapped_indices);

                        if b.vertex_size != 0 {
                            offset += b.vertices.len() as i32 / b.vertex_size as i32;
                        }

                        if b.generated_at_frame > a.generated_at_frame {
                            a.generated_at_frame = b.generated_at_frame;
                        }

                        return Some(a);
                    },
                );
                */
        }

        if meshes.is_empty() == false {
            let mut is_first = true;
            let mut offset = 0;

            for m in meshes.iter() {
                if is_first {
                    mesh.vertex_size = m.vertex_size;
                    mesh.color_vertex_size = m.color_vertex_size;
                    mesh.normal_vertex_size = m.normal_vertex_size;
                    is_first = false;
                }

                if mesh.vertex_size != m.vertex_size {
                    panic!("Unable to merge meshes! Mesh vertex sizes differ.");
                }

                if mesh.normal_vertex_size != m.normal_vertex_size {
                    panic!("Unable to merge meshes! Mesh normal vertex sizes differ.");
                }

                if mesh.color_vertex_size != m.color_vertex_size {
                    panic!("Unable to merge meshes! Mesh color vertex sizes differ.");
                }

                mesh.vertices.append(&mut m.vertices.clone());
                mesh.colors.append(&mut m.colors.clone());
                mesh.normals.append(&mut m.normals.clone());

                // do tricky shit with indices
                let mut mapped_indices = m.indices.iter().map(|i| i + offset).collect();

                mesh.indices.append(&mut mapped_indices);

                if m.vertex_size != 0 {
                    offset += m.vertices.len() as i32 / m.vertex_size as i32;
                }

                if m.generated_at_frame > mesh.generated_at_frame {
                    mesh.generated_at_frame = m.generated_at_frame;
                }
            }
        }

        return mesh;
    }
}
