use sdl2::video::{gl_attr::GLAttr, GLContext, Window};

pub mod buffer;
pub mod mesh;
pub mod shader;
pub mod texture;
pub mod vertex;

pub fn set_attr(attr: GLAttr) {
    attr.set_context_profile(sdl2::video::GLProfile::Core);
    attr.set_context_version(4, 5);
}

pub fn make_context(window: &Window) -> GLContext {
    window.gl_create_context().unwrap()
}
