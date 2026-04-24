pub mod bvh;
pub mod quad;
pub mod sphere;
pub mod constant_density_medium;
pub mod transformations;

use crate::{
    geometry::{Interval, Point3, Ray, Vec3},
    hittable::{
        bvh::{AABB, BVHNode}, constant_density_medium::ConstantDensityMedium, quad::Quad, sphere::Sphere, transformations::{Rotation, Translation}
    },
    material::Material,
};
use enum_dispatch::enum_dispatch;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<Material>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, mat: Arc<Material>, t: f32) -> Self {
        Self {
            p,
            normal: Vec3::ZERO,
            mat,
            t,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn set_uv_coords(&mut self, u: f32, v: f32) {
        self.u = u;
        self.v = v;
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
}

#[enum_dispatch(Hittable)]
#[derive(Debug)]
pub enum Object {
    Sphere,
    Quad,
    ConstantDensityMedium,
    Translation,
    Rotation,
    HittableList,
    BVHNode,
}

#[derive(Debug)]
pub struct HittableList {
    pub objects: Vec<Arc<Object>>,
    bbox: AABB,
}

impl HittableList {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::new(Interval::empty(), Interval::empty(), Interval::empty()),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.bbox = AABB::new(Interval::empty(), Interval::empty(), Interval::empty());
    }

    pub fn add<T: Into<Object>>(&mut self, object: T) {
        let object: Arc<Object> = Arc::new(object.into());
        self.bbox = AABB::containing(self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;
        let mut closest_dist = t_interval.max;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(r, Interval::new(t_interval.min, closest_dist)) {
                closest_dist = rec.t;
                closest = Some(rec);
            }
        }

        closest
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
