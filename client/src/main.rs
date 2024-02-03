mod event_loop;
pub mod gui_client;

use macroquad::prelude::Conf;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    event_loop::run().await;
}
