use crate::{
    geometry::{random_cosine_direction, random_unit_vector},
    pdf::ProbabilityDensityFunction,
};
use glam::{Mat3, Vec3};
use std::f32;

#[derive(Debug)]
pub struct SpherePDF;

impl ProbabilityDensityFunction for SpherePDF {
    fn get_value(&self, _direction: Vec3) -> f32 {
        1.0 / (4.0 * f32::consts::PI)
    }

    fn generate_direction(&self) -> Vec3 {
        random_unit_vector()
    }
}

#[derive(Debug)]
pub struct CosinePDF {
    onb: Mat3,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        let (u, v) = w.any_orthonormal_pair();
        let onb = Mat3::from_cols(u, v, w);
        Self { onb }
    }
}

impl ProbabilityDensityFunction for CosinePDF {
    fn get_value(&self, direction: Vec3) -> f32 {
        let cos_theta = direction.normalize().dot(self.onb.col(2));
        f32::max(0.0, cos_theta / f32::consts::PI)
    }

    fn generate_direction(&self) -> Vec3 {
        self.onb * random_cosine_direction()
    }
}
