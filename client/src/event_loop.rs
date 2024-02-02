use macroquad::prelude::*;
use simple_websockets::{Event, Message};

use crate::gui_client::{TextInput, VoxelCamera};

pub async fn run() {
    let event_hub = simple_websockets::launch(1234).expect("failed to listen on port 1234");
    let mut client = None;

    let mut camera = VoxelCamera::new();
    let mut text_input = TextInput::new();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    loop {
        if let Some(event) = event_hub.next_event() {
            match event {
                Event::Connect(_, responder) => {
                    println!("Client connected.");

                    client = Some(responder);
                }
                Event::Disconnect(_) => {
                    println!("Client disconnected.");

                    client = None;
                }
                Event::Message(_, mut message) => {
                    if let Message::Text(msg) = &message {
                        if msg == "nil" {
                            message = Message::Text("Command executed successfully".to_string());
                        }
                    }
                    println!("{:?}", message);
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

        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), GREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), BLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), RED);

        set_default_camera();
        draw_text("First Person Camera", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
