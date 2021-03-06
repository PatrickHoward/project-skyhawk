use crate::math::matrix::Mat4f32;
use gl::types::*;
use std::ffi::{CStr, CString};

fn make_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}

fn make_shader_internal(src: &CStr, kind: GLuint) -> Result<GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &src.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: GLint = 1;
    unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success) };

    if success == 0 {
        let mut len: GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = make_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar)
        };

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

pub struct FragmentShader {
    id: GLuint,
}

impl FragmentShader {
    pub fn new(src: &CStr) -> Result<Self, String> {
        let frag_shader = make_shader_internal(src, gl::FRAGMENT_SHADER);
        match frag_shader {
            Ok(shader) => Ok(FragmentShader { id: shader }),
            Err(str) => Err(str),
        }
    }
}

impl Drop for FragmentShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct VertexShader {
    id: GLuint,
}

impl VertexShader {
    pub fn new(src: &CStr) -> Result<Self, String> {
        let vert_shader = make_shader_internal(src, gl::VERTEX_SHADER);
        match vert_shader {
            Ok(shader) => Ok(VertexShader { id: shader }),
            Err(str) => Err(str),
        }
    }
}

impl Drop for VertexShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct GLShaderProgram {
    id: GLuint,
}

impl GLShaderProgram {
    pub fn new(fragment: FragmentShader, vertex: VertexShader) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(id, vertex.id);
            gl::AttachShader(id, fragment.id);
            gl::LinkProgram(id);
            gl::DetachShader(id, vertex.id);
            gl::DetachShader(id, fragment.id);
        }

        let mut success: GLint = 1;

        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = make_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(GLShaderProgram { id })
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for GLShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct GlShaderUniform {
    shader_id: u32,
    uniform_location: i32,
}

impl GlShaderUniform {
    pub fn new(program: &GLShaderProgram, uniform_name: &str) -> Self {
        let shader_id = program.id();
        let uniform_name: std::ffi::CString = std::ffi::CString::new(uniform_name).unwrap();
        let uniform_location = unsafe { gl::GetUniformLocation(shader_id, uniform_name.as_ptr()) };

        GlShaderUniform {
            shader_id,
            uniform_location,
        }
    }
}

// TODO: Make this Typesafe using generics
impl GlShaderUniform {
    pub fn set_i32(&self, value: i32) {
        unsafe { gl::Uniform1i(self.uniform_location, value) };
    }

    pub fn set_mat(&self, value: &Mat4f32) {
        unsafe {
            gl::UniformMatrix4fv(self.uniform_location, 1, gl::FALSE, value.internal.as_ptr())
        }
    }
}
