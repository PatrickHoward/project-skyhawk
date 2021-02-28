use crate::math::Vec3;
use crate::renderer::vertex::Vertex;
use crate::renderer::{X2f32Tuple, X3f32Tuple};
use std::mem;

/// # GLVert
/// Struct for providing packed vertex data to OpenGL's shaders, should only be used with
/// renderer::vertex::Vertex converted to a GlVert when piping data to OpenGL.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct GLVert {
    point: X3f32Tuple,
    color: X3f32Tuple,
    texcord: X2f32Tuple,
    normal: X3f32Tuple,
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

        location = 3;

        offset = offset + mem::size_of::<X3f32Tuple>();

        unsafe {
            Self::vertex_attrib_ptr(location, 3, stride, offset);
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
            normal: Vec3::zero().into_tuple(),
        }
    }
}
