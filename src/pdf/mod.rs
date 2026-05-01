pub mod hittable;
pub mod simple;
pub mod mixture;

use enum_dispatch::enum_dispatch;
use glam::Vec3;

use crate::pdf::{hittable::HittablePDF, mixture::MixturePDF, simple::{CosinePDF, SpherePDF}};

#[enum_dispatch]
pub trait ProbabilityDensityFunction {
    fn get_value(&self, direction: Vec3) -> f32;
    fn generate_direction(&self) -> Vec3;
}

#[enum_dispatch(ProbabilityDensityFunction)]
#[derive(Debug)]
pub enum PDF<'a> {
    SpherePDF,
    CosinePDF,
    HittablePDF(HittablePDF<'a>),
    MixturePDF(MixturePDF<'a>),
}
