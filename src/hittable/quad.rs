use core::f32;
use std::sync::Arc;

use glam::Vec3;
use rand::random;

use crate::{
    geometry::{Interval, Point3, Ray},
    hittable::{HitRecord, Hittable, HittableList, bvh::AABB},
    material::Material,
};

#[derive(Debug, Clone)]
pub struct Quad {
    corner: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<Material>,
    bbox: AABB,
    normal: Vec3,
    d: f32,
    area: f32,
}

impl Quad {
    pub fn new(corner: Point3, u: Vec3, v: Vec3, mat: Arc<Material>) -> Self {
        let bbox_diagonal_1 = AABB::from_extrema(corner, corner + u + v);
        let bbox_diagonal_2 = AABB::from_extrema(corner + u, corner + v);
        let bbox = AABB::containing(bbox_diagonal_1, bbox_diagonal_2);
        let n = Vec3::cross(u, v);
        let normal = n.normalize();
        let d = normal.dot(corner);
        let w = n / Vec3::dot(n, n);
        let area = n.length();

        Self {
            corner,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            d,
            area,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(r.direction);
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(r.origin)) / denom;
        if !t_interval.contains(t) {
            return None;
        }

        let intersection = r.at(t);
        let planar_hit_vector = intersection - self.corner;
        let alpha = self.w.dot(planar_hit_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_vector));

        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }

        let mut rec = HitRecord::new(intersection, Arc::clone(&self.mat), t);
        rec.set_uv_coords(alpha, beta);
        rec.set_face_normal(r, self.normal);
        Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f32 {
        if let Some(rec) = self.hit(
            Ray::new(origin, direction),
            Interval::new(0.001, f32::INFINITY),
        ) {
            let distance_squared = rec.t * rec.t * direction.length_squared();
            let cosine = f32::abs(direction.dot(rec.normal) / direction.length());

            distance_squared / (cosine * self.area)
        } else {
            0.0
        }
    }

    fn random(&self, origin: Point3) -> Vec3 {
        self.corner + (random::<f32>() * self.u) + (random::<f32>() * self.v) - origin
    }
}

pub fn make_box(a: Point3, b: Point3, mat: Arc<Material>) -> HittableList {
    let mut sides = HittableList::empty();

    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    sides.add(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        Arc::clone(&mat),
    ));
    sides.add(Quad::new(
        Point3::new(max.x, min.y, max.z),
        -dz,
        dy,
        Arc::clone(&mat),
    ));
    sides.add(Quad::new(
        Point3::new(max.x, min.y, min.z),
        -dx,
        dy,
        Arc::clone(&mat),
    ));
    sides.add(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dz,
        dy,
        Arc::clone(&mat),
    ));
    sides.add(Quad::new(
        Point3::new(min.x, max.y, max.z),
        dx,
        -dz,
        Arc::clone(&mat),
    ));
    sides.add(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dz,
        Arc::clone(&mat),
    ));

    sides
}
