use crate::{
    geometry::Ray,
    hittable::HitRecord,
    material::{Scatter, ScatterRecord},
    pdf::simple::SpherePDF,
    texture::{Sample, Texture},
};
use std::{f32, sync::Arc};

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
    fn scatter(&self, _r: Ray, rec: &HitRecord) -> Option<ScatterRecord<'_>> {
        Some(ScatterRecord::from_pdf(
            self.tex.sample(rec.u, rec.v, rec.p),
            SpherePDF.into(),
        ))
    }

    fn scattering_pdf(&self, _r: Ray, _rec: &HitRecord, _scattered: Ray) -> f32 {
        1. / (4. * f32::consts::PI)
    }
}
