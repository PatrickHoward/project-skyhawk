use crate::{
    input::*,
    math::{matrix::Mat4f32, rotation, Vec3f32},
};

pub struct Camera {
    pitch: f32,
    yaw: f32,
    speed: f32,
    fov: f32,

    direction: Vec3f32,
    position: Vec3f32,
    front: Vec3f32,
    up: Vec3f32,
}

impl Camera {
    pub fn new() -> Self {
        let position = Vec3f32::new(0.0f32, 0.0f32, 3.0f32);
        let front = Vec3f32::new(0.0f32, 0.0f32, -1.0f32);
        let up = Vec3f32::new(0.0f32, 1.0f32, 0.0f32);

        let pitch = 0.0;
        let yaw = -90.0;
        let speed = 5.0;

        let direction = Vec3f32::new(
            rotation::to_rad(yaw).cos() * rotation::to_rad(pitch).cos(),
            rotation::to_rad(pitch).sin(),
            rotation::to_rad(yaw).sin() * rotation::to_rad(pitch).cos(),
        );

        Camera {
            position,
            direction,
            front,
            up,
            pitch,
            yaw,
            speed,
            fov: 45.0f32,
        }
    }

    pub fn addto_yaw(&mut self, v: f32) {
        self.yaw += v;
    }
    pub fn addto_pitch(&mut self, v: f32) {
        self.pitch += v;
    }
    pub fn addto_fov(&mut self, v: f32) {
        self.fov -= v;
    }

    pub fn tick(&mut self, dt: f32, input: &Input) {
        const UP: InputMapping = InputMapping::Keyboard(sdl2::keyboard::Scancode::W as i32);
        const DOWN: InputMapping = InputMapping::Keyboard(sdl2::keyboard::Scancode::S as i32);
        const LEFT: InputMapping = InputMapping::Keyboard(sdl2::keyboard::Scancode::A as i32);
        const RIGHT: InputMapping = InputMapping::Keyboard(sdl2::keyboard::Scancode::D as i32);

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        if self.fov < 1.0f32 {
            self.fov = 1.0f32;
        } else if self.fov > 45.0f32 {
            self.fov = 45.0f32;
        }

        self.direction = Vec3f32::new(
            rotation::to_rad(self.yaw).cos() * rotation::to_rad(self.pitch).cos(),
            rotation::to_rad(self.pitch).sin(),
            rotation::to_rad(self.yaw).sin() * rotation::to_rad(self.pitch).cos(),
        );

        self.front = self.direction.normalize().unwrap();

        let camera_speed: f32 = dt * self.speed;

        if input.mapping_down(UP) {
            self.position = self.position + self.front.mul_scalar(camera_speed);
        }

        if input.mapping_down(DOWN) {
            self.position = self.position - self.front.mul_scalar(camera_speed);
        }

        if input.mapping_down(LEFT) {
            self.position = self.position
                - self
                    .front
                    .cross(self.up)
                    .normalize()
                    .unwrap()
                    .mul_scalar(camera_speed);
        }

        if input.mapping_down(RIGHT) {
            self.position = self.position
                + self
                    .front
                    .cross(self.up)
                    .normalize()
                    .unwrap()
                    .mul_scalar(camera_speed);
        }
    }

    pub fn get_viewmatrix(&self) -> Mat4f32 {
        Mat4f32::look_at(self.position, self.position + self.front, self.up)
    }

    pub fn get_perspectivematrix(&self) -> Mat4f32 {
        Mat4f32::perspective(self.fov, 1024.0f32 / 768.0f32, 0.1f32, 100.0f32)
    }
}
