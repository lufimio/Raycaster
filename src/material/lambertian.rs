use crate::{
    geometry::Ray,
    hittable::HitRecord,
    material::{Scatter, ScatterRecord},
    pdf::simple::CosinePDF,
    texture::{Sample, Texture},
};
use std::{f32, sync::Arc};

#[derive(Debug, Clone)]
pub struct Lambertian {
    tex: Arc<Texture>,
}

impl Lambertian {
    pub fn new(tex: Arc<Texture>) -> Self {
        Self { tex }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _r: Ray, rec: &HitRecord) -> Option<ScatterRecord<'_>> {
        Some(ScatterRecord::from_pdf(
            self.tex.sample(rec.u, rec.v, rec.p),
            CosinePDF::new(rec.normal).into(),
        ))
    }

    fn scattering_pdf(&self, _r: Ray, rec: &HitRecord, scattered: Ray) -> f32 {
        let cos_theta = rec.normal.dot(scattered.direction.normalize());
        if cos_theta < 0. {
            0.
        } else {
            cos_theta / f32::consts::PI
        }
    }
}
