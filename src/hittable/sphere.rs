use glam::{Mat3, Vec3};
use rand::random;

use crate::{
    geometry::{Interval, Point3, Ray},
    hittable::{HitRecord, Hittable, bvh::AABB},
    material::Material,
};
use core::f32;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Arc<Material>,
    bbox: AABB,
}

fn random_to_sphere(radius: f32, distance_squared: f32) -> Vec3 {
    let z = 1.0 + random::<f32>() * (f32::sqrt(1.0 - radius * radius / distance_squared) - 1.0);
    let phi = 2.0 * random::<f32>() * f32::consts::PI;
    let x = phi.cos() * f32::sqrt(1.0 - z * z);
    let y = phi.sin() * f32::sqrt(1.0 - z * z);
    Vec3::new(x, y, z)
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return Option::None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !t_interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !t_interval.surrounds(root) {
                return Option::None;
            }
        }

        let mut rec = HitRecord::new(r.at(root), Arc::clone(&self.mat), root);
        let outward_normal = (rec.p - self.center) / self.radius;
        let theta = f32::acos(-outward_normal.y);
        let phi = f32::atan2(-outward_normal.z, outward_normal.x) + f32::consts::PI;
        rec.set_uv_coords(phi / (2.0 * f32::consts::PI), theta / f32::consts::PI);
        rec.set_face_normal(r, outward_normal);

        Option::Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f32 {
        if let Some(_) = self.hit(Ray::new(origin, direction), Interval::new(0.001, f32::INFINITY)) {
            let dist_squared = Vec3::length_squared(self.center - origin);

            if dist_squared <= self.radius * self.radius {
                return 0.0;
            }

            let cos_theta_max = f32::sqrt(1.0 - self.radius * self.radius / dist_squared);
            let solid_angle = 2.0 * f32::consts::PI * (1.0 - cos_theta_max);

            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let direction = self.center - origin;
        let (u, v) = direction.any_orthonormal_pair();
        let onb = Mat3::from_cols(u, v, direction);
        onb * random_to_sphere(self.radius, direction.length_squared())
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, mat: Arc<Material>) -> Self {
        let radius = radius.max(0.0);
        Self {
            center,
            radius,
            mat,
            bbox: AABB::from_extrema(center - Vec3::ONE * radius, center + Vec3::ONE * radius),
        }
    }
}
