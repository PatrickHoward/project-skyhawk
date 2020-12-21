extern crate gl;

pub struct ArrayBuffer {
    vbo: gl::types::GLuint,
}

impl ArrayBuffer {
    pub fn new() -> Self {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        ArrayBuffer { vbo }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn buffer_data<T>(&self, data: &[T]) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as usize as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
        self.unbind();
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}

pub struct ElementBuffer {
    ebo: gl::types::GLuint,
}

impl ElementBuffer {
    pub fn new() -> Self {
        let mut ebo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo);
        }

        ElementBuffer { ebo }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

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
        unsafe { gl::DeleteBuffers(1, &mut self.ebo) }
    }
}

pub struct VertexArray {
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        VertexArray { vao }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &mut self.vao) }
    }
}
