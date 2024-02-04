use macroquad::prelude::*;
use simple_websockets::{Event, Message};

use crate::objects::{Block, TextInput, VoxelCamera};

pub async fn run() {
    let event_hub = simple_websockets::launch(1234).expect("failed to listen on port 1234");
    let mut client = None;

    let mut camera = VoxelCamera::new();
    let mut text_input = TextInput::new();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    let radius: u16 = 8;
    let width = 2 * radius + 1;
    let mut blocks: Vec<Block> = vec![Default::default(); width.pow(3) as usize];

    loop {
        if let Some(event) = event_hub.next_event() {
            match event {
                Event::Connect(_, responder) => {
                    log::info!("Turtle connected.");

                    client = Some(responder);
                }
                Event::Disconnect(_) => {
                    log::info!("Turtle disconnected.");

                    client = None;
                }
                Event::Message(_, msg_frame) => {
                    if let Message::Text(mut msg) = msg_frame.clone() {
                        if msg == "nil" {
                            msg = "Command executed successfully".to_string();
                        }

                        let names = msg
                            .split("][")
                            .next()
                            .unwrap()
                            .trim_start_matches('[')
                            .trim_end_matches(']')
                            .split(',')
                            .map(|s| s.trim_matches('"').to_string())
                            .collect::<Vec<String>>();
                        let coords_linearized = msg
                            .split("][")
                            .nth(1)
                            .unwrap()
                            .trim_start_matches('[')
                            .trim_end_matches(']')
                            .split(',')
                            .map(|s| s.parse::<u16>().unwrap())
                            .collect::<Vec<u16>>();

                        for (block_index, name_index) in coords_linearized.iter().enumerate() {
                            let block = &mut blocks[block_index];

                            block.coord = Block::delinearize(width, block_index as u16);
                            if *name_index > 0 {
                                block.name = names[*name_index as usize - 1].to_owned();
                            } else {
                                block.name = "minecraft:air".to_string();
                            }
                        }
                    }
                }
            }
        };

        clear_background(LIGHTGRAY);

        camera.process();

        if camera.locked {
            text_input.process();
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            camera.locked = !camera.locked;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if is_key_pressed(KeyCode::Enter) && camera.locked {
            if let Some(responder) = &client {
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

        set_default_camera();
        draw_text("First Person Camera", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
