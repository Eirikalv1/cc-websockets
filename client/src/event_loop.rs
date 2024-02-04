use macroquad::prelude::*;
use simple_websockets::Message;

use crate::{
    objects::{Block, KeyboardEventHandler, TextInput, VoxelCamera},
    sockets::Sockets,
    SCAN_WIDTH_CUBED,
};

pub async fn run() {
    let mut sockets = Sockets::new();
    let mut camera = VoxelCamera::new();
    let mut text_input = TextInput::new();
    let mut keyboard_events = KeyboardEventHandler::new();

    let mut blocks: Vec<Block> = vec![Default::default(); SCAN_WIDTH_CUBED as usize];

    loop {
        clear_background(LIGHTGRAY);

        sockets.process(&mut blocks);

        camera.process();
        if camera.locked {
            text_input.process();
        }

        if KeyboardEventHandler::should_grab() {
            keyboard_events.switch_grab_mode(&mut camera);
        }
        if KeyboardEventHandler::should_close_app() {
            break;
        }

        if KeyboardEventHandler::should_submit_command(&camera) {
            if let Some(responder) = &sockets.client {
                responder.send(Message::Text(text_input.text));
            }
            text_input.text = String::new();
        }

        draw_grid(20, 1., BLACK, GRAY);

        for block in blocks.iter() {
            if block.name != *"minecraft:air" {
                draw_cube(block.coord - 0.5, vec3(1., 1., 1.), None, GREEN);
                draw_cube_wires(block.coord - 0.5, vec3(1., 1., 1.), BLACK);
            }
        }
        VoxelCamera::set_default_camera();
        next_frame().await
    }
}
