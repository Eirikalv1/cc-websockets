use macroquad::prelude::*;

use crate::{
    objects::{Block, VoxelCamera},
    SCAN_WIDTH_CUBED,
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

        for block in self.blocks.iter() {
            if block.name != *"minecraft:air" {
                draw_cube(block.coord - 0.5, vec3(1., 1., 1.), None, GREEN);
                draw_cube_wires(block.coord - 0.5, vec3(1., 1., 1.), BLACK);
            }
        }
        VoxelCamera::set_default_camera();
        next_frame().await
    }
}
