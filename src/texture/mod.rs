pub mod checker;
pub mod image;
pub mod solid_color;

use crate::{
    geometry::{Color, Point3},
    texture::{checker::Checker, image::Image, solid_color::SolidColor},
};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Sample {
    fn sample(&self, u: f32, v: f32, p: Point3) -> Color;
}

#[enum_dispatch(Sample)]
#[derive(Debug, Clone)]
pub enum Texture {
    SolidColor,
    Checker,
    Image,
}

impl From<Color> for Texture {
    fn from(value: Color) -> Self {
        Texture::SolidColor(SolidColor::new(value))
    }
}
