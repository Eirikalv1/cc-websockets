use macroquad::prelude::*;
use macroquad::hash;
use macroquad::ui::root_ui;

struct VoxelCamera {
    move_speed: f32,
    last_mouse_position: Vec2,
    locked: bool,
    look_speed: f32,
    pitch: f32,
    position: Vec3,
    yaw: f32,
}

impl VoxelCamera {
    fn new() -> Self {
        let yaw: f32 = 1.18;
        let pitch: f32 = 0.0;

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
        }
    }

    fn process(&mut self) {
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

        let front = vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();

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

pub async fn run() {
    let mut text = String::new();

    let mut camera = VoxelCamera::new();
    
    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    loop {
        clear_background(LIGHTGRAY);

        camera.process();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            camera.locked = !camera.locked;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), GREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), BLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), RED);

        set_default_camera();
        draw_text("First Person Camera", 10.0, 20.0, 30.0, BLACK);

        root_ui().window(hash!(), vec2(700., 0.), vec2(300., 50.), |ui| {
            ui.input_text(hash!(), "Text", &mut text);
            });
        root_ui().pop_skin();

        next_frame().await
    }
}
