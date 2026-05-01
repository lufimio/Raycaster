pub mod dielectric;
pub mod isotropic;
pub mod lambertian;
pub mod light;
pub mod metal;

use crate::{
    geometry::{Color, Point3, Ray},
    hittable::HitRecord,
    material::{
        dielectric::Dielectric, isotropic::Isotropic, lambertian::Lambertian, light::DiffuseLight,
        metal::Metal,
    },
    pdf::PDF,
};
use enum_dispatch::enum_dispatch;

pub enum Scattered<'a> {
    PDF(PDF<'a>),
    Ray(Ray),
}

pub struct ScatterRecord<'a> {
    pub attenuation: Color,
    pub scattered: Scattered<'a>,
}

impl<'a> ScatterRecord<'a> {
    pub fn from_pdf(attenuation: Color, pdf: PDF<'a>) -> Self {
        Self {
            attenuation,
            scattered: Scattered::PDF(pdf),
        }
    }

    pub fn from_ray(attenuation: Color, ray: Ray) -> Self {
        Self {
            attenuation,
            scattered: Scattered::Ray(ray),
        }
    }
}

#[enum_dispatch]
pub trait Scatter {
    fn scatter(&self, _r: Ray, _rec: &HitRecord) -> Option<ScatterRecord<'_>> {
        Option::None
    }

    fn emitted(&self, _r: Ray, _rec: &HitRecord, _u: f32, _v: f32, _p: Point3) -> Color {
        Color::ZERO
    }

    fn scattering_pdf(&self, _r: Ray, _rec: &HitRecord, _scattered: Ray) -> f32 {
        0.0
    }
}

#[derive(Debug)]
pub struct Empty;
impl Scatter for Empty {}

#[enum_dispatch(Scatter)]
#[derive(Debug)]
pub enum Material {
    Empty,
    Lambertian,
    Metal,
    Dielectric,
    Isotropic,
    DiffuseLight,
}
