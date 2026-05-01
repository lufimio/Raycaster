use glam::Vec3;
use rand::random_range;

use crate::pdf::{PDF, ProbabilityDensityFunction};
use std::sync::Arc;

#[derive(Debug)]
pub struct MixturePDF<'a> {
    pdfs: Vec<Arc<PDF<'a>>>,
}

impl<'a> MixturePDF<'a> {
    pub fn new(pdfs: Vec<Arc<PDF<'a>>>) -> Self {
        Self { pdfs }
    }

    pub fn from_pair(p0: impl Into<PDF<'a>>, p1: impl Into<PDF<'a>>) -> Self {
        Self {
            pdfs: vec![Arc::new(p0.into()), Arc::new(p1.into())],
        }
    }
}

impl<'a> ProbabilityDensityFunction for MixturePDF<'a> {
    fn get_value(&self, direction: Vec3) -> f32 {
        self.pdfs
            .iter()
            .map(|pdf| (1.0 / self.pdfs.len() as f32) * pdf.get_value(direction))
            .sum()
    }

    fn generate_direction(&self) -> Vec3 {
        self.pdfs[random_range(0..self.pdfs.len())].generate_direction()
    }
}
