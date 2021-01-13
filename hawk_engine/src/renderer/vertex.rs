use crate::{
    math::{Vec2, Vec3},
    renderer::color::Color,
};

use gl;

use std::mem;

/// # Vertex
/// Higher level concept of a vertex, should be converted into the appropriate data type
/// upon rendering.
pub struct Vertex {
    point: Vec3,
    color: Color,
    texcord: Vec2,
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

type X3f32Tuple = (f32, f32, f32);
type X2f32Tuple = (f32, f32);

/// # GLVert
/// Struct for providing packed vertex data to OpenGL's shaders, should only be used with
/// renderer::vertex::Vertex converted to a GlVert when piping data to OpenGL.
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

        unsafe { Self::vertex_attrib_ptr(location, 3, stride, offset) }

        location = 1;
        offset = offset + mem::size_of::<X3f32Tuple>();

        unsafe { Self::vertex_attrib_ptr(location, 3, stride, offset) }

        location = 2;
        offset = offset + mem::size_of::<X3f32Tuple>();

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
            offset as *const gl::types::GLvoid,
        );
    }
}

pub trait AsGLVert {
    fn as_glvert(&self) -> GLVert;
}

impl AsGLVert for Vertex {
    fn as_glvert(&self) -> GLVert {
        GLVert {
            point: self.point.into_tuple(),
            color: self.color.color().into_tuple(),
            texcord: self.texcord.into_tuple(),
        }
    }
}
