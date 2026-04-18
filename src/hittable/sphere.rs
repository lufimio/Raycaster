use std::rc::Rc;

use crate::{
    geometry::{Interval, Point3, Ray},
    hittable::{HitRecord, Hittable}, material::Material,
};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<Material>,
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
        let root = (h - sqrtd) / a;
        if !t_interval.surrounds(root) {
            let root = (h + sqrtd) / a;
            if !t_interval.surrounds(root) {
                return Option::None;
            }
        }

        let mut rec = HitRecord::new(r.at(root), Rc::clone(&self.mat), root);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Option::Some(rec)
    }
}

impl Sphere {
    pub fn new<T: Into<f64>>(center: Point3, radius: T, mat: Rc<Material>) -> Self {
        Self {
            center,
            radius: radius.into().max(0.0),
            mat,
        }
    }
}
