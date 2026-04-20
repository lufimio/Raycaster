use crate::{
    geometry::{random_unit_vector, Color, Ray},
    hittable::HitRecord,
    material::{dielectric::reflect, Scatterable},
};

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0., 1.),
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(r.direction, rec.normal);
        let reflected = reflected.normalized() + self.fuzz * random_unit_vector();
        let scattered = Ray::new(rec.p, reflected);

        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
