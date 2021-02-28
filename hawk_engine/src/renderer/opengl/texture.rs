use crate::mem::sid::AsStrId;
use crate::renderer::graphics::ImageRGB;

pub struct GlTexture {
    name: crate::mem::StrId,
    texture: gl::types::GLuint,
}

impl GlTexture {
    pub fn new(graphic: ImageRGB, texture_name: Option<&str>) -> Self {
        let name = match texture_name {
            Some(s) => s,
            None => "<default>",
        }
        .as_str_id();

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

        GlTexture { texture, name }
    }

    pub fn from_path(img_path: &std::path::Path) -> Self {
        let graphic = crate::renderer::graphics::ImageRGB::new(&img_path);
        Self::new(graphic, None) // TODO: Replace "None" with actual texture name
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }
}
