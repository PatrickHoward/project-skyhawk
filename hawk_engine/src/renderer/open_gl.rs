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
