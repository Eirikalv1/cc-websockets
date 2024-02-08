use crate::{
    objects::{KeyboardEventHandler, VoxelCamera, VoxelUi},
    renderer::Renderer,
    sockets::Sockets,
};

pub async fn run() {
    let mut sockets = Sockets::new();
    let mut camera = VoxelCamera::new();
    let mut ui_handler = VoxelUi::new();
    let mut keyboard_events = KeyboardEventHandler::new();
    let mut renderer = Renderer::new();

    loop {
        sockets.process(&mut renderer.blocks);

        camera.process();
        if camera.locked {
            ui_handler.process(&sockets);
        }

        if KeyboardEventHandler::should_grab() {
            keyboard_events.switch_grab_mode(&mut camera);
        }

        keyboard_events.scroll_mosue();
        renderer.objects_to_render = keyboard_events.scroll_index;

        if KeyboardEventHandler::should_submit_command(&camera) {
            sockets.send_message(ui_handler.text);
            ui_handler.text = String::new();
        }

        renderer.draw(&camera, &keyboard_events).await;
    }
}
