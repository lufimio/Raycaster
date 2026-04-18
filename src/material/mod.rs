pub mod lambertian;
pub mod metal;

use crate::{
    geometry::{Color, Ray},
    hittable::HitRecord,
    material::{lambertian::Lambertian, metal::Metal},
};

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Scatterable {
    fn scatter(&self, r: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        Option::None
    }
}

#[enum_dispatch(Scatterable)]
#[derive(Debug)]
pub enum Material {
    Lambertian,
    Metal
}
