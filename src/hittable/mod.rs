pub mod sphere;

use std::rc::Rc;

use enum_dispatch::enum_dispatch;

use crate::{
    geometry::{Interval, Point3, Ray, Vec3},
    hittable::sphere::Sphere,
    material::Material,
};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, mat: Rc<Material>, t: f64) -> Self {
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
}

#[enum_dispatch(Hittable)]
pub enum Object {
    Sphere,
}

pub struct World {
    pub objects: Vec<Object>,
}

impl World {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add<T: Into<Object>>(&mut self, object: T) {
        self.objects.push(object.into());
    }

    pub fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(
                r,
                Interval::new(
                    t_interval.min,
                    if let Some(ref c) = closest {
                        c.t
                    } else {
                        t_interval.max
                    },
                ),
            ) {
                closest = Some(rec);
            }
        }

        closest
    }
}
