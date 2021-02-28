use crate::renderer::opengl::{texture::GlTexture, vertex::GlVert};

struct GlMesh {
    vertices: Vec<GlVert>,
    indices: Vec<i32>,
    textures: Vec<GlTexture>,
}

impl GlMesh {
    pub fn render() {}
}
