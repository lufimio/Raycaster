use crate::{
    geometry::{Color, Ray},
    hittable::HitRecord,
    material::{Scatter, ScatterRecord},
};
use glam::Vec3;
use rand::random;

fn reflectance(cosine: f32, ri: f32) -> f32 {
    let r0 = (1.0 - ri) / (1.0 + ri);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r: Ray, rec: &HitRecord) -> Option<ScatterRecord<'_>> {
        let ri = if rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = r.direction.normalize();
        let cos_theta = f32::min(Vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
        let direction = if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > random() {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, ri)
        };

        Some(ScatterRecord::from_ray(
            Color::ONE,
            Ray::new(rec.p, direction),
        ))
    }
}
