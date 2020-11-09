extern crate sdl2;

use sdl2::video::{gl_attr::GLAttr, GLContext, Window};
use crate::renderer::vertex::GLVert;

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

    // This does borrow self for simple organization reasons
    pub fn static_draw(&self, verts: &Vec<GLVert>) {
        unsafe { gl::BufferData(
                gl::ARRAY_BUFFER,
                (verts.len() * std::mem::size_of::<GLVert>()) as usize as gl::types::GLsizeiptr,
                verts.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW);
        }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0); }
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &mut self.vbo); }
    }
}

pub struct VertexArray {
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
