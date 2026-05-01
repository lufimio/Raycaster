use glam::Vec3;

use crate::{
    geometry::Point3,
    hittable::{Hittable, Object},
    pdf::ProbabilityDensityFunction,
};

#[derive(Debug)]
pub struct HittablePDF<'a> {
    object: &'a Object,
    origin: Point3,
}

impl<'a> HittablePDF<'a> {
    pub fn new(object: &'a Object, origin: Point3) -> Self {
        Self { object, origin }
    }
}

impl<'a> ProbabilityDensityFunction for HittablePDF<'a> {
    fn get_value(&self, direction: Vec3) -> f32 {
        self.object.pdf_value(self.origin, direction)
    }

    fn generate_direction(&self) -> Vec3 {
        self.object.random(self.origin)
    }
}
