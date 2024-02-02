use macroquad::prelude::*;

struct VoxelCamera {
    grabbed: bool,
    move_speed: f32,
    last_mouse_position: Vec2,
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

        let grabbed = true;
        set_cursor_grab(grabbed);
        show_mouse(false);

        VoxelCamera {
            grabbed,
            move_speed: 0.1,
            last_mouse_position,
            look_speed: 0.1,
            pitch,
            position,
            yaw,
        }
    }

    fn process(&mut self) {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Tab) {
            self.grabbed = !self.grabbed;
            set_cursor_grab(self.grabbed);
            show_mouse(!self.grabbed);
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - self.last_mouse_position;
        self.last_mouse_position = mouse_position;

        self.yaw += mouse_delta.x * delta * self.look_speed;
        self.pitch += mouse_delta.y * delta * -self.look_speed;

        self.pitch = if self.pitch > 1.5 { 1.5 } else { self.pitch };
        self.pitch = if self.pitch < -1.5 { -1.5 } else { self.pitch };

        let front = vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();

        let right = front.cross(Vec3::Y).normalize();

        if is_key_down(KeyCode::W) {
            self.position += front * self.move_speed;
        }
        if is_key_down(KeyCode::S) {
            self.position -= front * self.move_speed;
        }
        if is_key_down(KeyCode::A) {
            self.position -= right * self.move_speed;
        }
        if is_key_down(KeyCode::D) {
            self.position += right * self.move_speed;
        }
        if is_key_down(KeyCode::E) {
            self.position.y += self.move_speed;
        }
        if is_key_down(KeyCode::Q) {
            self.position.y -= self.move_speed;
        }

        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: self.position,
            up: Vec3::Y,
            target: self.position + front,
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), GREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), BLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), RED);

        set_default_camera();
        draw_text("First Person Camera", 10.0, 20.0, 30.0, BLACK);

        draw_text(
            format!("X: {} Y: {}", mouse_position.x, mouse_position.y).as_str(),
            10.0,
            48.0 + 18.0,
            30.0,
            BLACK,
        );
        draw_text(
            format!("Press <TAB> to toggle mouse grab: {}", self.grabbed).as_str(),
            10.0,
            48.0 + 42.0,
            30.0,
            BLACK,
        );
    }
}

pub async fn run() {
    let mut voxel_camera = VoxelCamera::new();
    loop {
        voxel_camera.process();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}
