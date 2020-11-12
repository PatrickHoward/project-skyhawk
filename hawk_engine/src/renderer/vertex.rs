use crate::{
    math::{Vec3f32, Vec2f32},
    renderer::color::Color
};

use gl;
use std::mem;

pub struct Vertex {
    point: Vec3f32,
    color: Color,
    texcord: Vec2f32,
}

impl Vertex {
    pub fn new(point: Vec3f32, color: Color, texcord: Vec2f32) -> Self {
        Vertex { point, color, texcord }
    }

    pub fn pos(&self) -> Vec3f32 {
        self.point
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn texcord(&self) -> Vec2f32 { self.texcord }
}

type X3f32Tuple = (f32, f32, f32);
type X2f32Tuple = (f32, f32);

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct GLVert {
    point: X3f32Tuple,
    color: X3f32Tuple,
    texcord: X2f32Tuple,
}

impl GLVert {
    pub fn vertex_attr_pointer() {
        let stride = mem::size_of::<Self>();
        let mut location = 0;
        let mut offset = 0;

        unsafe {
            Self::vertex_attrib_ptr(location, 3, stride, offset)
        }

        location = 1;
        offset = offset + mem::size_of::<X3f32Tuple>();

        unsafe {
            Self::vertex_attrib_ptr(location, 3, stride, offset)
        }

        location = 2;
        offset = offset + mem::size_of::<X2f32Tuple>();

        unsafe {
            Self::vertex_attrib_ptr(location, 2, stride, offset);
        }
    }

    unsafe fn vertex_attrib_ptr(location: i32, count: i32, stride: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            count,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

pub trait AsGLVert {
    fn as_glvert(&self) -> GLVert;
}

impl AsGLVert for Vertex {
    fn as_glvert(&self) -> GLVert {
        GLVert {
            point: self.point.as_tuple(),
            color: self.color.color().as_tuple(),
            texcord: self.texcord.as_tuple(),
        }
    }
}
