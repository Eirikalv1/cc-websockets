use macroquad::prelude::*;
use simple_websockets::{Event, Message};

use crate::gui_client::{TextInput, VoxelCamera};

fn to3d(i: f32) -> Vec3 {
    let w = 3.;
    let z = f32::floor(i / (w * w));
    let r = i % (w * w);
    let y = f32::floor(r / w);
    let x = r % w;
    vec3(x, y, z)
}

pub async fn run() {
    let event_hub = simple_websockets::launch(1234).expect("failed to listen on port 1234");
    let mut client = None;

    let mut camera = VoxelCamera::new();
    let mut text_input = TextInput::new();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    let mut block_positions = vec![];

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

                        let _blocks = msg
                            .split("][")
                            .next()
                            .unwrap()
                            .trim_start_matches('[')
                            .trim_end_matches(']')
                            .split(',')
                            .map(|s| s.trim_matches('"').to_string())
                            .collect::<Vec<String>>();
                        let block_positions_linearized = msg
                            .split("][")
                            .nth(1)
                            .unwrap()
                            .trim_start_matches('[')
                            .trim_end_matches(']')
                            .split(',')
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();

                        block_positions = block_positions_linearized
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

        for (i, block) in block_positions.iter().enumerate() {
            if *block != 0 {
                draw_cube(to3d(i as f32) - 0.5, vec3(1., 1., 1.), None, GREEN);
                draw_cube_wires(to3d(i as f32) - 0.5, vec3(1., 1., 1.), BLACK);
            }
        }

        set_default_camera();
        draw_text("First Person Camera", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
