use std::sync::Arc;

use image::{DynamicImage, GenericImageView, ImageReader, RgbImage};

use crate::{
    geometry::{Color, Interval, Point3, rgb_to_color},
    texture::{Sample, Texture},
};

#[derive(Debug, Clone)]
pub struct Image {
    image: RgbImage,
}

impl Image {
    pub fn new(filename: &str) -> Self {
        Self {
            image: ImageReader::open(filename)
                .expect("Failed to open texture image.")
                .decode()
                .expect("Failed to decode image.")
                .into(),
        }
    }
}

impl Sample for Image {
    fn sample(&self, u: f32, v: f32, _p: Point3) -> Color {
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * (self.image.width() - 1) as f32) as u32;
        let j = (v * (self.image.height() - 1) as f32) as u32;
        rgb_to_color(self.image.get_pixel(i, j).clone())
    }
}
