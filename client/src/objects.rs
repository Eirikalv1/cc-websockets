use macroquad::prelude::*;

use crate::{SCAN_WIDTH, SCAN_WIDTH_SQUARED};

#[derive(Default)]
pub struct VoxelCamera {
    move_speed: f32,
    last_mouse_position: Vec2,
    pub locked: bool,
    look_speed: f32,
    pitch: f32,
    pub position: Vec3,
    yaw: f32,
    pub direction: Vec3,
}

impl VoxelCamera {
    pub fn new() -> Self {
        let yaw: f32 = 1.18;
        let pitch: f32 = 0.0;
        let direction = Vec3::ZERO;

        let position = vec3(0.0, 1.0, 0.0);
        let last_mouse_position: Vec2 = mouse_position().into();

        VoxelCamera {
            move_speed: 0.1,
            last_mouse_position,
            locked: false,
            look_speed: 0.1,
            pitch,
            position,
            yaw,
            direction,
        }
    }

    pub fn process(&mut self) {
        let delta = get_frame_time();

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - self.last_mouse_position;
        self.last_mouse_position = mouse_position;

        if !self.locked {
            self.yaw += mouse_delta.x * delta * self.look_speed;
            self.pitch += mouse_delta.y * delta * -self.look_speed;
        }

        self.pitch = if self.pitch > 1.5 { 1.5 } else { self.pitch };
        self.pitch = if self.pitch < -1.5 { -1.5 } else { self.pitch };

        self.direction = vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();
        let front = self.direction;

        let right = front.cross(Vec3::Y).normalize();

        if is_key_down(KeyCode::W) && !self.locked {
            self.position += front * self.move_speed;
        }
        if is_key_down(KeyCode::S) && !self.locked {
            self.position -= front * self.move_speed;
        }
        if is_key_down(KeyCode::A) && !self.locked {
            self.position -= right * self.move_speed;
        }
        if is_key_down(KeyCode::D) && !self.locked {
            self.position += right * self.move_speed;
        }
        if is_key_down(KeyCode::E) && !self.locked {
            self.position.y += self.move_speed;
        }
        if is_key_down(KeyCode::Q) && !self.locked {
            self.position.y -= self.move_speed;
        }

        set_camera(&Camera3D {
            position: self.position,
            up: Vec3::Y,
            target: self.position + front,
            ..Default::default()
        });
    }
}

#[derive(Default)]
pub struct TextInput {
    pub text: String,
}

impl TextInput {
    pub fn new() -> Self {
        TextInput {
            text: String::new(),
        }
    }

    pub fn process(&mut self) {
        use macroquad::hash;
        use macroquad::ui::root_ui;

        root_ui().window(hash!(), vec2(700., 0.), vec2(300., 50.), |ui| {
            ui.input_text(macroquad::hash!(), "Text", &mut self.text);
        });
        root_ui().pop_skin();
    }
}

#[derive(Clone)]
pub struct Block {
    pub name: String,
    pub coord: Vec3,
    pub color: Color,
}

impl Block {
    pub fn delinearize(block_index: u16) -> Vec3 {
        let w = SCAN_WIDTH;
        let r = block_index % (w * w);

        let x = r % w;
        let y = r / w;
        let z = block_index / (w * w);

        vec3(x as f32, y as f32, z as f32)
    }

    pub fn linearize(coord: Vec3) -> u16 {
        (coord.x + coord.y * SCAN_WIDTH as f32 + coord.z * SCAN_WIDTH_SQUARED as f32) as u16
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            name: "minecraft:air".to_string(),
            coord: Vec3::ZERO,
            color: GREEN,
        }
    }
}

#[derive(Default)]
pub struct KeyboardEventHandler {
    pub mouse_grabbed: bool,
}

impl KeyboardEventHandler {
    pub fn left_clicked() -> bool {
        is_mouse_button_down(MouseButton::Left)
    }

    pub fn new() -> Self {
        let mouse_grabbed = true;

        set_cursor_grab(mouse_grabbed);
        show_mouse(false);

        KeyboardEventHandler { mouse_grabbed }
    }

    pub fn should_close_app() -> bool {
        is_key_pressed(KeyCode::Escape)
    }

    pub fn should_grab() -> bool {
        is_key_pressed(KeyCode::Tab)
    }

    pub fn should_submit_command(camera: &VoxelCamera) -> bool {
        is_key_pressed(KeyCode::Enter) && camera.locked
    }

    pub fn switch_grab_mode(&mut self, camera: &mut VoxelCamera) {
        self.mouse_grabbed = !self.mouse_grabbed;
        camera.locked = !camera.locked;
        set_cursor_grab(self.mouse_grabbed);
        show_mouse(!self.mouse_grabbed);
    }
}
