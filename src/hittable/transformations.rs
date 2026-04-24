use std::sync::Arc;

use glam::{Quat, Vec3};

use crate::{
    geometry::{Interval, Point3, Ray},
    hittable::{HitRecord, Hittable, Object, bvh::AABB},
};

#[derive(Debug, Clone)]
pub struct Translation {
    offset: Vec3,
    object: Arc<Object>,
    bbox: AABB,
}

impl Translation {
    pub fn new(object: impl Into<Object>, offset: Vec3) -> Self {
        let object = Arc::new(object.into());
        Self {
            offset,
            bbox: object.bounding_box() + offset,
            object,
        }
    }
}

impl Hittable for Translation {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord> {
        if let Some(mut ray) = self
            .object
            .hit(Ray::new(r.origin - self.offset, r.direction), t_interval)
        {
            ray.p = ray.p + self.offset;
            Some(ray)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

#[derive(Debug, Clone)]
pub struct Rotation {
    rotation: Quat,
    object: Arc<Object>,
    bbox: AABB,
}

impl Rotation {
    pub fn new(object: impl Into<Object>, rotation: Quat) -> Self {
        let object = Arc::new(object.into());
        let bbox = object.bounding_box();
        let mut min = Point3::INFINITY;
        let mut max = Point3::NEG_INFINITY;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.x.max + (1 - i) as f32 * bbox.x.min;
                    let y = j as f32 * bbox.y.max + (1 - j) as f32 * bbox.y.min;
                    let z = k as f32 * bbox.z.max + (1 - k) as f32 * bbox.z.min;

                    let vec = rotation.mul_vec3(Vec3::new(x, y, z));
                    for c in 0..3 {
                        min[c] = min[c].min(vec[c]);
                        max[c] = max[c].max(vec[c]);
                    }
                }
            }
        }

        let bbox = AABB::from_extrema(min, max);

        Self {
            rotation,
            object,
            bbox,
        }
    }
}

impl Hittable for Rotation {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord> {
        let transformed_ray = Ray::new(
            self.rotation.inverse().mul_vec3(r.origin),
            self.rotation.inverse().mul_vec3(r.direction),
        );

        if let Some(mut ray) = self.object.hit(transformed_ray, t_interval) {
            ray.p = self.rotation.mul_vec3(ray.p);
            ray.normal = self.rotation.mul_vec3(ray.normal);
            Some(ray)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
