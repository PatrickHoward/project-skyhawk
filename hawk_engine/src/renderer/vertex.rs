use crate::{
    math::{Vec2, Vec3},
    renderer::color::Color,
};

/// # Vertex
/// Higher level concept of a vertex, should be converted into the appropriate data type
/// upon rendering.
pub struct Vertex {
    pub point: Vec3,
    pub color: Color,
    pub texcord: Vec2,
}

impl Vertex {
    pub fn new(point: Vec3, color: Color, texcord: Vec2) -> Self {
        Vertex {
            point,
            color,
            texcord,
        }
    }

    pub fn pos(&self) -> Vec3 {
        self.point
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn texcord(&self) -> Vec2 {
        self.texcord
    }
}
