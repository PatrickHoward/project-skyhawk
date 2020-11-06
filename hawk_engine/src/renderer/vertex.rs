use crate::math::Vec3f32;
use crate::renderer::color::Color;

pub struct Vertex {
    point: Vec3f32,
    color: Color,
}

impl Vertex {
    pub fn new(point: Vec3f32, color: Color) -> Self {
        Vertex { point, color }
    }

    pub fn pos(&self) -> Vec3f32 {
        self.point
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct GLVert {
    point: (f32, f32, f32),
    color: (f32, f32, f32),
}

pub trait AsGLVert {
    fn as_glvert(&self) -> GLVert;
}

impl AsGLVert for Vertex {
    fn as_glvert(&self) -> GLVert {
        GLVert {
            point: self.point.as_tuple(),
            color: self.color.color().as_tuple(),
        }
    }
}
