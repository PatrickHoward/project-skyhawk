use crate::{
    math::Vec3f32,
    renderer::color::Color
};

use gl;

use std::mem;

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

type X3f32Tuple = (f32, f32, f32);

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct GLVert {
    point: X3f32Tuple,
    color: X3f32Tuple,
}

impl GLVert {
    pub fn vertex_attr_pointer() {
        let stride = mem::size_of::<Self>();
        let mut location = 0;
        let mut offset = 0;

        unsafe {
            Self::vertex_attrib_ptr(location, stride, offset)
        }

        location = 1;
        offset = offset + mem::size_of::<X3f32Tuple>();

        unsafe {
            Self::vertex_attrib_ptr(location, stride, offset)
        }
    }

    unsafe fn vertex_attrib_ptr(location: i32, stride: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            3,
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
        }
    }
}
