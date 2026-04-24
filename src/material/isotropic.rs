use crate::{
    geometry::{Color, Ray, random_unit_vector},
    hittable::HitRecord,
    material::Scatter,
    texture::{Sample, Texture},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Isotropic {
    tex: Arc<Texture>,
}

impl Isotropic {
    pub fn new(tex: Arc<Texture>) -> Self {
        Self { tex }
    }
}

impl Scatter for Isotropic {
    fn scatter(&self, _r: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        Some((
            Ray::new(rec.p, random_unit_vector()),
            self.tex.sample(rec.u, rec.v, rec.p)
        ))
    }
}
