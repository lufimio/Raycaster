use crate::{
    geometry::{Color, Ray, near_zero, random_unit_vector},
    hittable::HitRecord,
    material::Scatterable,
};

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _r: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if near_zero(scatter_direction) {
            scatter_direction = rec.normal
        }

        Some((Ray::new(rec.p, scatter_direction), self.albedo))
    }
}
