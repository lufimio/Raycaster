pub mod bvh;
pub mod sphere;

use std::sync::Arc;

use enum_dispatch::enum_dispatch;

use crate::{
    geometry::{Interval, Point3, Ray, Vec3},
    hittable::{bvh::{AABB, BVHNode}, sphere::Sphere},
    material::Material,
};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, mat: Arc<Material>, t: f64) -> Self {
        Self {
            p,
            normal: Vec3::zero(),
            mat,
            t,
            front_face: false,
        }
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
