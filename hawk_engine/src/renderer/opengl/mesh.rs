use crate::renderer::opengl::{texture::GlTexture, vertex::GLVert};

struct GlMesh {
    vertices: Vec<GLVert>,
    indices: Vec<i32>,
    textures: Vec<GlTexture>,
}

impl GlMesh {
    pub fn render() {}
}
