extern crate sdl2;

use sdl2::video::{gl_attr::GLAttr, GLContext, Window};

use crate::{math::matrix::Mat4f32, renderer::graphics::ImageRGB};

use std::ffi::CString;

pub fn set_attr(attr: GLAttr) {
    attr.set_context_profile(sdl2::video::GLProfile::Core);
    attr.set_context_version(4, 5);
}

pub fn make_context(window: &Window) -> GLContext {
    window.gl_create_context().unwrap()
}

pub struct GlTexture {
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

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                graphic.width() as i32,
                graphic.height() as i32,
                0,
                gl::RGB as u32,
                gl::UNSIGNED_BYTE,
                graphic.data().as_ptr() as *const gl::types::GLvoid,
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
pub struct GlMat4f32Handle {
    uniform_location: i32,
}

impl GlMat4f32Handle {
    pub fn new(shader: &crate::renderer::shader::GLShaderProgram, uniform_name: &str) -> Self {
        let attr_name = CString::new(uniform_name).unwrap();

        let uniform_location = unsafe { gl::GetUniformLocation(shader.id(), attr_name.as_ptr()) };

        GlMat4f32Handle { uniform_location }
    }

    pub fn transform(&self, transform_mat: &Mat4f32) {
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_location,
                1,
                gl::FALSE,
                transform_mat.internal.as_ptr(),
            )
        }
    }
}

pub struct GlUniformHandle {
    uniform_location: i32,
}
