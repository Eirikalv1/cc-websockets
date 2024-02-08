use simple_websockets::Message;

use crate::{
    objects::{KeyboardEventHandler, TextInput, VoxelCamera},
    renderer::Renderer,
    sockets::Sockets,
};

pub async fn run() {
    let mut sockets = Sockets::new();
    let mut camera = VoxelCamera::new();
    let mut text_input = TextInput::new();
    let mut keyboard_events = KeyboardEventHandler::new();
    let mut renderer = Renderer::new();

    loop {
        sockets.process(&mut renderer.blocks);

        camera.process();
        if camera.locked {
            text_input.process();
        }

        if KeyboardEventHandler::should_grab() {
            keyboard_events.switch_grab_mode(&mut camera);
        }

        keyboard_events.scroll_mosue();
        renderer.objects_to_render = keyboard_events.scroll_index;

        if KeyboardEventHandler::should_submit_command(&camera) {
            if let Some(responder) = &sockets.client {
                responder.send(Message::Text(text_input.text));
            }
            text_input.text = String::new();
        }

        renderer.draw(&camera).await;
    }
}
