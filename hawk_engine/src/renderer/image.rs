use crate::math::Vec2;

use image::imageops::flip_vertical;

use std::path::Path;

pub struct ImageRGB {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl ImageRGB {
    pub fn new(path: &Path) -> Self {
        let img = image::open(path).unwrap().into_rgb();
        let img = flip_vertical(&img);

        let height = img.height();
        let width = img.width();
        let data = img.into_raw();

        ImageRGB {
            width,
            height,
            data,
        }
    }

    pub fn dimensions(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
