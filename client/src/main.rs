mod event_loop;
pub mod objects;
pub mod renderer;
pub mod sockets;

use macroquad::prelude::Conf;

pub const SCAN_RADIUS: u16 = 16; // Between 1 and 16
pub const SCAN_WIDTH: u16 = 2 * SCAN_RADIUS + 1;
pub const SCAN_WIDTH_SQUARED: u16 = SCAN_WIDTH.pow(2);
pub const SCAN_WIDTH_CUBED: u16 = SCAN_WIDTH.pow(3);

pub const SCREEN_WIDTH: i32 = 1260;
pub const SCREEN_HEIGHT: i32 = 768;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
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
