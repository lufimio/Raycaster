use glam::Vec3;
use rand::random;

use crate::{
    geometry::{Interval, Ray},
    hittable::{HitRecord, Hittable, Object, bvh::AABB},
    material::{Material, isotropic::Isotropic},
    texture::Texture,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ConstantDensityMedium {
    boundary: Arc<Object>,
    neg_inv_density: f32,
    phase_function: Arc<Material>,
}

impl ConstantDensityMedium {
    pub fn new(boundary: impl Into<Object>, density: f32, tex: Arc<Texture>) -> Self {
        Self {
            boundary: Arc::new(boundary.into()),
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(tex).into()),
        }
    }
}

impl Hittable for ConstantDensityMedium {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(r, Interval::all()) {
            if let Some(mut rec2) = self
                .boundary
                .hit(r, Interval::new(rec1.t + 0.0001, f32::INFINITY))
            {
                rec1.t = rec1.t.max(t_interval.min);
                rec2.t = rec2.t.min(t_interval.max);

                if rec1.t >= rec2.t {
                    return None;
                }

                rec1.t = rec1.t.max(0.0);

                let ray_length = r.direction.length();
                let distance_in_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random::<f32>().ln();

                if hit_distance > distance_in_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                let mut rec = HitRecord::new(r.at(t), Arc::clone(&self.phase_function), t);
                rec.normal = Vec3::X;
                rec.front_face = true;
                return Some(rec);
            }
        }
        None
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
