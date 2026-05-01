use crate::{
    geometry::{Color, Ray, random_unit_vector},
    hittable::HitRecord,
    material::{Scatter, ScatterRecord},
};

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0., 1.),
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r: Ray, rec: &HitRecord) -> Option<ScatterRecord<'_>> {
        let reflected = r.direction.reflect(rec.normal);
        let reflected = reflected.normalize() + self.fuzz * random_unit_vector();
        let scattered = Ray::new(rec.p, reflected);
        Some(ScatterRecord::from_ray(self.albedo, scattered))
    }
}
