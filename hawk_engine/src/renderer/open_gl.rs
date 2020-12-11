extern crate sdl2;

use sdl2::video::{gl_attr::GLAttr, GLContext, Window};

use std::ffi::CString;
use crate::math::matrix::Mat4f32;
use crate::renderer::graphics::ImageRGB;

pub fn set_attr(attr: GLAttr) {
    attr.set_context_profile(sdl2::video::GLProfile::Core);
    attr.set_context_version(4, 5);
}

pub fn make_context(window: &Window) -> GLContext {
    window.gl_create_context().unwrap()
}

pub(crate) struct ArrayBuffer {
    vbo: gl::types::GLuint,
}

impl ArrayBuffer {
    pub fn new() -> Self {
        let mut vbo: gl::types::GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut vbo); }

        ArrayBuffer { vbo }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo); }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0); }
    }

    // This does borrow self for simple organization reasons
    pub fn buffer_data<T>(&self, data: &[T]) {
        unsafe { gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as usize as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW);
        }
    }

}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &mut self.vbo); }
    }
}

pub(crate) struct ElementBuffer {
    ebo: gl::types::GLuint,
}

impl ElementBuffer {
    pub fn new() -> Self {
        let mut ebo: gl::types::GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut ebo); }

        ElementBuffer { ebo }
    }

    pub fn bind(&self) { unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo); } }
    pub fn unbind(&self) { unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); } }

    pub fn buffer_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as usize as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}

impl Drop for ElementBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &mut self.ebo )}
    }
}

pub(crate) struct VertexArray {
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut vao: gl::types::GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao); }

        VertexArray { vao }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.vao); }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0); }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao)
        }
    }
}

pub(crate) struct GlTexture {
    texture: gl::types::GLuint,
}

impl GlTexture {
    pub fn new(graphic: ImageRGB) -> Self {
        let mut texture: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGB as i32,
                           graphic.width() as i32,
                           graphic.height() as i32,
                           0,
                           gl::RGB as u32,
                           gl::UNSIGNED_BYTE,
                           graphic.data().as_ptr() as *const gl::types::GLvoid
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        GlTexture { texture }
    }

    pub fn from_path(img_path: &std::path::Path) -> Self {
        let graphic = crate::renderer::graphics::ImageRGB::new(&img_path);
        Self::new(graphic)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }
}

/// Handle that provides transformations for Mat4f32 (mat4)
pub(crate) struct GlMat4f32Handle {
    uniform_location: i32,
}

impl GlMat4f32Handle {
    pub fn new( shader: &crate::renderer::shader::GLShaderProgram, uniform_name: &str) -> Self {
        let attr_name = CString::new(uniform_name).unwrap();

        let uniform_location = unsafe {
            gl::GetUniformLocation(shader.id(), attr_name.as_ptr())
        };

        GlMat4f32Handle { uniform_location }
    }

    pub fn transform(&self, transform_mat: &Mat4f32) {
        unsafe {
            gl::UniformMatrix4fv(self.uniform_location, 1, gl::FALSE, transform_mat.internal.as_ptr())
        }
    }
}