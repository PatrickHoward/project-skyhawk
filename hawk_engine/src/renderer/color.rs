use crate::math::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    internal: Vec3,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        let internal = Vec3::new((r / 255) as f32, (g / 255) as f32, (b / 255) as f32);

        Color { internal, a }
    }

    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (
            self.internal.x,
            self.internal.y,
            self.internal.z,
            self.a as f32,
        )
    }

    pub fn new_opaque(r: u8, g: u8, b: u8) -> Self {
        let internal = Vec3::new((r / 255) as f32, (g / 255) as f32, (b / 255) as f32);

        Color { internal, a: 255 }
    }

    pub fn normalized_opaque(&self) -> (f32, f32, f32) {
        (self.internal.x, self.internal.y, self.internal.z)
    }

    pub fn color(&self) -> Vec3 {
        self.internal
    }
}

impl Color {
    pub fn red() -> Self {
        Color::new(255, 0, 0, 255)
    }

    pub fn green() -> Self {
        Color::new(0, 255, 0, 255)
    }

    pub fn blue() -> Self {
        Color::new(0, 0, 255, 255)
    }

    pub fn white() -> Self {
        Color::new(255, 255, 255, 255)
    }

    pub fn black() -> Self {
        Color::new(0, 0, 0, 255)
    }
}
