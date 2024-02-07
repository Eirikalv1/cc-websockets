use macroquad::{models::Vertex, prelude::*};

use crate::{
    objects::{Block, KeyboardEventHandler, VoxelCamera},
    SCAN_WIDTH, SCAN_WIDTH_CUBED, SCREEN_HEIGHT, SCREEN_WIDTH,
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

    pub async fn draw(&self, camera: &VoxelCamera) {
        clear_background(LIGHTGRAY);

        draw_grid(20, 1., BLACK, GRAY);
        self.mesh();

        self.draw_ui(camera);

        next_frame().await
    }

    pub fn draw_ui(&self, camera: &VoxelCamera) {
        // set_default_camera should be called after drawing in 3d
        set_default_camera();

        draw_text(
            "+",
            SCREEN_WIDTH as f32 / 2.,
            SCREEN_HEIGHT as f32 / 2.,
            30.,
            DARKGRAY,
        );
        if KeyboardEventHandler::left_clicked() {
            let block_name = self.ray_march(camera);
            if block_name.len() > 0 {
                draw_rectangle(8., 8., 10. + block_name.len() as f32 * 16., 39., DARKGRAY);
            }
            draw_text(block_name, 20.0, 35.0, 35., WHITE);
        }
    }

    fn mesh(&self) {
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u16> = vec![];
        let mut indicies_index: u32 = 0;

        let texture = Some(Texture2D::from_file_with_format(
            include_bytes!("smooth_stone.png"),
            None,
        ));
        texture.as_ref().unwrap().set_filter(FilterMode::Nearest);

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

            // Macroquad's draw call limitation of 10000 vertices or 5000 indices
            if vertices.len() > 9800 || indices.len() > 4800 {
                draw_mesh(&Mesh {
                    vertices: vertices.clone(),
                    indices: indices.clone(),
                    texture: texture.clone(),
                });

                vertices = vec![];
                indices = vec![];
                indicies_index = 0;
            }
        }

        draw_mesh(&Mesh {
            vertices: vertices.clone(),
            indices: indices.clone(),
            texture: texture.clone(),
        });
    }

    fn ray_march(&self, camera: &VoxelCamera) -> &str {
        let max_steps = 10000;
        let max_distance = 100.;
        let surface_distance = 0.01;

        let ray_origin = camera.position;
        let ray_direction = camera.direction;

        let mut distance = 0.;
        let mut block_name = "";

        for _ in 0..max_steps {
            let position = ray_origin + ray_direction * distance;
            let closest_block = self.get_closest_block(position);

            distance += closest_block.0;
            block_name = closest_block.1;

            if closest_block.0 < surface_distance {
                return block_name;
            }
            if distance > max_distance {
                return "";
            }
        }

        block_name
    }

    fn get_closest_block(&self, point: Vec3) -> (f32, &str) {
        let mut shortest_distance = f32::INFINITY;
        let mut block_name = "";

        for block in self.blocks.iter() {
            if block.name == "minecraft:air" {
                continue;
            }

            let center = block.coord + 0.5;

            let delta = vec3(
                (point.x - center.x).abs() - 0.5,
                (point.y - center.y).abs() - 0.5,
                (point.z - center.z).abs() - 0.5,
            );
            let sdf_value = delta.x.max(delta.y).max(delta.z).max(0.);

            if sdf_value < shortest_distance {
                shortest_distance = sdf_value;
                block_name = &block.name;
            }
        }

        (shortest_distance, block_name)
    }

    fn get_quad_data(quad: usize, block: &Block) -> Vec<Vertex> {
        let Block {
            name: _,
            coord:
                Vec3 {
                    x: min_x,
                    y: min_y,
                    z: min_z,
                },
            color,
        } = block.to_owned();

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
