use std::{
    fs::File,
    path::Path
};
use crate::math::Vec2f32;

pub(crate) struct ImageRGB {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl ImageRGB {
    pub fn new() -> Self {
        let img = image::open(&Path::new("container.jpg")).unwrap().into_rgb();

        let height = img.height();
        let width = img.width();
        let data = img.into_raw();

        ImageRGB {
            width,
            height,
            data
        }
    }

    pub fn dimensions(&self) -> Vec2f32 {
        Vec2f32::new(self.width as f32, self.height as f32)
    }

    pub fn width(&self)  -> u32 { self.width  }
    pub fn height(&self) -> u32 { self.height }
    pub fn data(&self) -> &Vec<u8> { &self.data }
}

