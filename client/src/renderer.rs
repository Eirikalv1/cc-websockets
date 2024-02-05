use macroquad::prelude::*;

use crate::{
    objects::{Block, VoxelCamera},
    SCAN_WIDTH, SCAN_WIDTH_CUBED,
};

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
        for block in self.blocks.iter() {
            if block.name == "minecraft:air" {
                continue;
            }

            let mut should_draw = false;
            let coord = block.coord;

            if coord.x == 0.
                || coord.x == SCAN_WIDTH as f32 - 1.
                || coord.y == 0.
                || coord.y == SCAN_WIDTH as f32 - 1.
                || coord.z == 0.
                || coord.z == SCAN_WIDTH as f32 - 1.
            {
                should_draw = true;
            }

            if let Some(adjacent_block) =
                self.blocks
                    .get(Block::linearize(Vec3::new(coord.x + 1., coord.y, coord.z)) as usize)
            {
                if adjacent_block.name == "minecraft:air" {
                    should_draw = true;
                }
            }
            if let Some(adjacent_block) =
                self.blocks
                    .get(Block::linearize(Vec3::new(coord.x - 1., coord.y, coord.z)) as usize)
            {
                if adjacent_block.name == "minecraft:air" {
                    should_draw = true;
                }
            }
            if let Some(adjacent_block) =
                self.blocks
                    .get(Block::linearize(Vec3::new(coord.x, coord.y + 1., coord.z)) as usize)
            {
                if adjacent_block.name == "minecraft:air" {
                    should_draw = true;
                }
            }
            if let Some(adjacent_block) =
                self.blocks
                    .get(Block::linearize(Vec3::new(coord.x, coord.y - 1., coord.z)) as usize)
            {
                if adjacent_block.name == "minecraft:air" {
                    should_draw = true;
                }
            }
            if let Some(adjacent_block) =
                self.blocks
                    .get(Block::linearize(Vec3::new(coord.x, coord.y, coord.z + 1.)) as usize)
            {
                if adjacent_block.name == "minecraft:air" {
                    should_draw = true;
                }
            }
            if let Some(adjacent_block) =
                self.blocks
                    .get(Block::linearize(Vec3::new(coord.x, coord.y, coord.z - 1.)) as usize)
            {
                if adjacent_block.name == "minecraft:air" {
                    should_draw = true;
                }
            }

            if should_draw {
                draw_cube(coord + 0.5, Vec3::ONE, None, block.color);
                draw_cube_wires(coord + 0.5, Vec3::ONE, BLACK);
            }
        }
    }
}
