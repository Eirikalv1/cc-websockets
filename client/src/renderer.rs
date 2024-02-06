use macroquad::{models::Vertex, prelude::*};

use crate::{
    objects::{Block, VoxelCamera},
    SCAN_WIDTH, SCAN_WIDTH_CUBED,
};

const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

#[derive(Default)]
pub struct Renderer {
    pub blocks: Vec<Block>,
}

impl Renderer {
    pub fn new() -> Self {
        let blocks: Vec<Block> = vec![Default::default(); SCAN_WIDTH_CUBED as usize];

        Renderer { blocks }
    }

    pub async fn draw(&self) {
        clear_background(LIGHTGRAY);

        draw_grid(20, 1., BLACK, GRAY);
        self.mesh();

        VoxelCamera::set_default_camera();
        next_frame().await
    }

    fn mesh(&self) {
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u16> = vec![];
        let mut indicies_index: u32 = 0;

        for block in self.blocks.iter() {
            if block.name == "minecraft:air" {
                continue;
            }
            let mut block_vertices: Vec<Vertex> = vec![];
            let mut block_indices: Vec<u16> = vec![];

            for quad in 0..6 {
                if !self.quad_is_visible(quad, block) {
                    continue;
                }

                block_vertices.append(&mut Self::get_quad_data(quad, block));
                block_indices.append(&mut vec![
                    INDICES[0] + indicies_index as u16,
                    INDICES[1] + indicies_index as u16,
                    INDICES[2] + indicies_index as u16,
                    INDICES[3] + indicies_index as u16,
                    INDICES[4] + indicies_index as u16,
                    INDICES[5] + indicies_index as u16,
                ]);

                indicies_index += 4;
            }

            vertices.append(&mut block_vertices);
            indices.append(&mut block_indices);

            draw_cube_wires(block.coord + 0.5, Vec3::ONE, BLACK);

            // Macroquad's draw call limitation of 10000 vertices or 5000 indices
            if vertices.len() > 9800 || indices.len() > 4800 {
                draw_mesh(&Mesh {
                    vertices: vertices.clone(),
                    indices: indices.clone(),
                    texture: None,
                });

                vertices = vec![];
                indices = vec![];
                indicies_index = 0;
            }
        }

        draw_mesh(&Mesh {
            vertices: vertices.clone(),
            indices: indices.clone(),
            texture: None,
        });
    }

    fn get_quad_data(quad: usize, block: &Block) -> Vec<Vertex> {
        let min_x = block.coord.x;
        let min_y = block.coord.y;
        let min_z = block.coord.z;
        let color = block.color;

        let max_x = min_x + 1.;
        let max_y = min_y + 1.;
        let max_z = min_z + 1.;

        match quad {
            // Front face
            0 => vec![
                Vertex {
                    position: vec3(min_x, min_y, max_z),
                    uv: vec2(0., 0.),
                    color,
                },
                Vertex {
                    position: vec3(max_x, min_y, max_z),
                    uv: vec2(1., 0.),
                    color,
                },
                Vertex {
                    position: vec3(max_x, max_y, max_z),
                    uv: vec2(1., 1.),
                    color,
                },
                Vertex {
                    position: vec3(min_x, max_y, max_z),
                    uv: vec2(0., 1.),
                    color,
                },
            ],
            // Back face
            1 => vec![
                Vertex {
                    position: vec3(min_x, min_y, min_z),
                    uv: vec2(1., 0.),
                    color,
                },
                Vertex {
                    position: vec3(max_x, min_y, min_z),
                    uv: vec2(0., 0.),
                    color,
                },
                Vertex {
                    position: vec3(max_x, max_y, min_z),
                    uv: vec2(0., 1.),
                    color,
                },
                Vertex {
                    position: vec3(min_x, max_y, min_z),
                    uv: vec2(1., 1.),
                    color,
                },
            ],
            // Top face
            2 => vec![
                Vertex {
                    position: vec3(min_x, max_y, min_z),
                    uv: vec2(1., 0.),
                    color,
                },
                Vertex {
                    position: vec3(min_x, max_y, max_z),
                    uv: vec2(0., 0.),
                    color,
                },
                Vertex {
                    position: vec3(max_x, max_y, max_z),
                    uv: vec2(0., 1.),
                    color,
                },
                Vertex {
                    position: vec3(max_x, max_y, min_z),
                    uv: vec2(1., 1.),
                    color,
                },
            ],
            // Bottom face
            3 => vec![
                Vertex {
                    position: vec3(min_x, min_y, min_z),
                    uv: vec2(0., 0.),
                    color,
                },
                Vertex {
                    position: vec3(min_x, min_y, max_z),
                    uv: vec2(1., 0.),
                    color,
                },
                Vertex {
                    position: vec3(max_x, min_y, max_z),
                    uv: vec2(1., 1.),
                    color,
                },
                Vertex {
                    position: vec3(max_x, min_y, min_z),
                    uv: vec2(0., 1.),
                    color,
                },
            ],
            // Right face
            4 => vec![
                Vertex {
                    position: vec3(max_x, min_y, min_z),
                    uv: vec2(0., 0.),
                    color,
                },
                Vertex {
                    position: vec3(max_x, max_y, min_z),
                    uv: vec2(1., 0.),
                    color,
                },
                Vertex {
                    position: vec3(max_x, max_y, max_z),
                    uv: vec2(1., 1.),
                    color,
                },
                Vertex {
                    position: vec3(max_x, min_y, max_z),
                    uv: vec2(0., 1.),
                    color,
                },
            ],
            // Left face
            5 => vec![
                Vertex {
                    position: vec3(min_x, min_y, min_z),
                    uv: vec2(1., 0.),
                    color,
                },
                Vertex {
                    position: vec3(min_x, max_y, min_z),
                    uv: vec2(0., 0.),
                    color,
                },
                Vertex {
                    position: vec3(min_x, max_y, max_z),
                    uv: vec2(0., 1.),
                    color,
                },
                Vertex {
                    position: vec3(min_x, min_y, max_z),
                    uv: vec2(1., 1.),
                    color,
                },
            ],
            _ => panic!("Quad indexing out of range"),
        }
    }

    fn quad_is_visible(&self, quad: usize, block: &Block) -> bool {
        let coord = block.coord;

        match quad {
            0 => {
                if let Some(adjacent_block) =
                    self.blocks
                        .get(Block::linearize(Vec3::new(coord.x, coord.y, coord.z + 1.)) as usize)
                {
                    if adjacent_block.name == "minecraft:air" {
                        return true;
                    }
                }
                if coord.z == SCAN_WIDTH as f32 - 1. {
                    return true;
                }
            }
            1 => {
                if let Some(adjacent_block) =
                    self.blocks
                        .get(Block::linearize(Vec3::new(coord.x, coord.y, coord.z - 1.0)) as usize)
                {
                    if adjacent_block.name == "minecraft:air" {
                        return true;
                    }
                }
                if coord.z == 0. {
                    return true;
                }
            }
            2 => {
                if let Some(adjacent_block) =
                    self.blocks
                        .get(Block::linearize(Vec3::new(coord.x, coord.y + 1., coord.z)) as usize)
                {
                    if adjacent_block.name == "minecraft:air" {
                        return true;
                    }
                }
                if coord.y == SCAN_WIDTH as f32 - 1. {
                    return true;
                }
            }
            3 => {
                if let Some(adjacent_block) =
                    self.blocks
                        .get(Block::linearize(Vec3::new(coord.x, coord.y - 1., coord.z)) as usize)
                {
                    if adjacent_block.name == "minecraft:air" {
                        return true;
                    }
                }
                if coord.y == 0. {
                    return true;
                }
            }
            4 => {
                if let Some(adjacent_block) =
                    self.blocks
                        .get(Block::linearize(Vec3::new(coord.x + 1., coord.y, coord.z)) as usize)
                {
                    if adjacent_block.name == "minecraft:air" {
                        return true;
                    }
                }
                if coord.x == SCAN_WIDTH as f32 - 1. {
                    return true;
                }
            }
            5 => {
                if let Some(adjacent_block) =
                    self.blocks
                        .get(Block::linearize(Vec3::new(coord.x - 1., coord.y, coord.z)) as usize)
                {
                    if adjacent_block.name == "minecraft:air" {
                        return true;
                    }
                }
                if coord.x == 0. {
                    return true;
                }
            }
            _ => unreachable!("Quad indexing out of range"),
        }
        false
    }
}
