pub mod singlebox {
    use crate::math::{Vec2f32, Vec3f32};

    pub fn get_verticies() -> [Vec3f32; 4] {
        [
            Vec3f32::new(0.5, 0.5, 0.0),
            Vec3f32::new(0.5, -0.5, 0.0),
            Vec3f32::new(-0.5, -0.5, 0.0),
            Vec3f32::new(-0.5, 0.5, 0.0),
        ]
    }
    pub fn get_texture_coordinates() -> [Vec2f32; 4] {
        [
            Vec2f32::new(1.0f32, 1.0f32),
            Vec2f32::new(1.0f32, 0.0f32),
            Vec2f32::new(0.0f32, 0.0f32),
            Vec2f32::new(0.0f32, 1.0f32),
        ]
    }
    pub fn get_indicies() -> [u32; 6] {
        [0, 1, 3, 1, 2, 3]
    }
}

pub mod multibox {
    use crate::math::{Vec2f32, Vec3f32};

    pub fn get_verticies() -> [Vec3f32; 36] {
        [
            Vec3f32::new(-0.5f32, -0.5f32, -0.5f32),
            Vec3f32::new(0.5f32, -0.5f32, -0.5f32),
            Vec3f32::new(0.5f32, 0.5f32, -0.5f32),
            Vec3f32::new(0.5f32, 0.5f32, -0.5f32),
            Vec3f32::new(-0.5f32, 0.5f32, -0.5f32),
            Vec3f32::new(-0.5f32, -0.5f32, -0.5f32),
            Vec3f32::new(-0.5f32, -0.5f32, 0.5f32),
            Vec3f32::new(0.5f32, -0.5f32, 0.5f32),
            Vec3f32::new(0.5f32, 0.5f32, 0.5f32),
            Vec3f32::new(0.5f32, 0.5f32, 0.5f32),
            Vec3f32::new(-0.5f32, 0.5f32, 0.5f32),
            Vec3f32::new(-0.5f32, -0.5f32, 0.5f32),
            Vec3f32::new(-0.5f32, 0.5f32, 0.5f32),
            Vec3f32::new(-0.5f32, 0.5f32, -0.5f32),
            Vec3f32::new(-0.5f32, -0.5f32, -0.5f32),
            Vec3f32::new(-0.5f32, -0.5f32, -0.5f32),
            Vec3f32::new(-0.5f32, -0.5f32, 0.5f32),
            Vec3f32::new(-0.5f32, 0.5f32, 0.5f32),
            Vec3f32::new(0.5f32, 0.5f32, 0.5f32),
            Vec3f32::new(0.5f32, 0.5f32, -0.5f32),
            Vec3f32::new(0.5f32, -0.5f32, -0.5f32),
            Vec3f32::new(0.5f32, -0.5f32, -0.5f32),
            Vec3f32::new(0.5f32, -0.5f32, 0.5f32),
            Vec3f32::new(0.5f32, 0.5f32, 0.5f32),
            Vec3f32::new(-0.5f32, -0.5f32, -0.5f32),
            Vec3f32::new(0.5f32, -0.5f32, -0.5f32),
            Vec3f32::new(0.5f32, -0.5f32, 0.5f32),
            Vec3f32::new(0.5f32, -0.5f32, 0.5f32),
            Vec3f32::new(-0.5f32, -0.5f32, 0.5f32),
            Vec3f32::new(-0.5f32, -0.5f32, -0.5f32),
            Vec3f32::new(-0.5f32, 0.5f32, -0.5f32),
            Vec3f32::new(0.5f32, 0.5f32, -0.5f32),
            Vec3f32::new(0.5f32, 0.5f32, 0.5f32),
            Vec3f32::new(0.5f32, 0.5f32, 0.5f32),
            Vec3f32::new(-0.5f32, 0.5f32, 0.5f32),
            Vec3f32::new(-0.5f32, 0.5f32, -0.5f32),
        ]
    }
    pub fn get_texture_coordinates() -> [Vec2f32; 36] {
        [
            Vec2f32::new(0.0f32, 0.0f32),
            Vec2f32::new(1.0f32, 0.0f32),
            Vec2f32::new(1.0f32, 1.0f32),
            Vec2f32::new(1.0f32, 1.0f32),
            Vec2f32::new(0.0f32, 1.0f32),
            Vec2f32::new(0.0f32, 0.0f32),
            Vec2f32::new(0.0f32, 0.0f32),
            Vec2f32::new(1.0f32, 0.0f32),
            Vec2f32::new(1.0f32, 1.0f32),
            Vec2f32::new(1.0f32, 1.0f32),
            Vec2f32::new(0.0f32, 1.0f32),
            Vec2f32::new(0.0f32, 0.0f32),
            Vec2f32::new(1.0f32, 0.0f32),
            Vec2f32::new(1.0f32, 1.0f32),
            Vec2f32::new(0.0f32, 1.0f32),
            Vec2f32::new(0.0f32, 1.0f32),
            Vec2f32::new(0.0f32, 0.0f32),
            Vec2f32::new(1.0f32, 0.0f32),
            Vec2f32::new(1.0f32, 0.0f32),
            Vec2f32::new(1.0f32, 1.0f32),
            Vec2f32::new(0.0f32, 1.0f32),
            Vec2f32::new(0.0f32, 1.0f32),
            Vec2f32::new(0.0f32, 0.0f32),
            Vec2f32::new(1.0f32, 0.0f32),
            Vec2f32::new(0.0f32, 1.0f32),
            Vec2f32::new(1.0f32, 1.0f32),
            Vec2f32::new(1.0f32, 0.0f32),
            Vec2f32::new(1.0f32, 0.0f32),
            Vec2f32::new(0.0f32, 0.0f32),
            Vec2f32::new(0.0f32, 1.0f32),
            Vec2f32::new(0.0f32, 1.032),
            Vec2f32::new(1.0f32, 1.0f32),
            Vec2f32::new(1.0f32, 0.0f32),
            Vec2f32::new(1.0f32, 0.0f32),
            Vec2f32::new(0.0f32, 0.0f32),
            Vec2f32::new(0.0f32, 1.0f32),
        ]
    }
    pub fn get_cube_positions() -> [Vec3f32; 10] {
        [
            Vec3f32::zero(),
            Vec3f32::new(2.0f32, 5.0f32, -15.0f32),
            Vec3f32::new(-1.5f32, -2.2f32, -2.5f32),
            Vec3f32::new(-3.8f32, -2.0f32, -12.3f32),
            Vec3f32::new(2.4f32, -0.4f32, -3.5f32),
            Vec3f32::new(-1.7f32, 3.0f32, -7.5f32),
            Vec3f32::new(1.3f32, -2.0f32, -2.5f32),
            Vec3f32::new(1.5f32, 2.0f32, -2.5f32),
            Vec3f32::new(1.5f32, 0.2f32, -1.5f32),
            Vec3f32::new(-1.3f32, 1.0f32, -1.5f32),
        ]
    }
}
