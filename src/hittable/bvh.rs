use std::{cmp::Ordering, sync::Arc, usize};

use rand::random_range;

use crate::{
    geometry::{Interval, Point3, Ray},
    hittable::{HitRecord, Hittable, HittableList, Object},
};

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_extrema(a: Point3, b: Point3) -> Self {
        Self {
            x: Interval::new(f64::min(a.x, b.x), f64::max(a.x, b.x)),
            y: Interval::new(f64::min(a.y, b.y), f64::max(a.y, b.y)),
            z: Interval::new(f64::min(a.z, b.z), f64::max(a.z, b.z)),
        }
    }

    pub fn empty() -> Self {
        Self::new(Interval::empty(), Interval::empty(), Interval::empty())
    }

    pub fn all() -> Self {
        Self::new(Interval::all(), Interval::all(), Interval::all())
    }

    pub fn containing(a: AABB, b: AABB) -> AABB {
        Self::new(
            Interval::containing(a.x, b.x),
            Interval::containing(a.y, b.y),
            Interval::containing(a.z, b.z),
        )
    }

    pub fn axis_interval(self, n: usize) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid Axis {}, expected 0-2", n),
        }
    }

    pub fn hit(self, r: Ray, t_interval: Interval) -> bool {
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / r.direction[axis];

            let t0 = (ax.min - r.origin[axis]) * adinv;
            let t1 = (ax.max - r.origin[axis]) * adinv;

            let mut ray_t = t_interval;
            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug, Clone)]
pub struct BVHNode {
    left: Option<Arc<Object>>,
    right: Option<Arc<Object>>,
    bbox: AABB,
}

impl BVHNode {
    pub fn from_hittable_list(list: HittableList) -> Self {
        Self::new(list.objects)
    }

    fn new(mut objects: Vec<Arc<Object>>) -> Self {
        let axis = random_range(0..=2);

        let left;
        let right;
        let bbox;
        if objects.len() == 0 {
            left = None;
            right = None;
            bbox = AABB::empty();
        } else if objects.len() == 1 {
            left = Some(Arc::clone(&objects[0]));
            right = None;
            bbox = left.as_ref().unwrap().bounding_box();
        } else if objects.len() == 2 {
            left = Some(Arc::clone(&objects[0]));
            right = Some(Arc::clone(&objects[1]));
            bbox = AABB::containing(
                left.as_ref().unwrap().bounding_box(),
                right.as_ref().unwrap().bounding_box(),
            );
        } else {
            objects.sort_by(|a, b| {
                a.bounding_box()
                    .axis_interval(axis)
                    .min
                    .total_cmp(&b.bounding_box().axis_interval(axis).min)
            });
            let mid = objects.len() / 2;
            right = Some(Arc::new(BVHNode::new(objects.split_off(mid)).into()));
            left = Some(Arc::new(BVHNode::new(objects).into()));
            bbox = AABB::containing(
                left.as_ref().unwrap().bounding_box(),
                right.as_ref().unwrap().bounding_box(),
            );
        }

        Self { left, right, bbox }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: Ray, t_interval: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_interval) {
            None
        } else {
            if let Some(left) = self.left.as_ref() {
                let left_rec = left.hit(r, t_interval);
                if let Some(right) = self.right.as_ref()
                    && let Some(right_rec) = right.hit(
                        r,
                        Interval::new(
                            t_interval.min,
                            match left_rec {
                                Some(ref rec) => rec.t,
                                None => t_interval.max,
                            },
                        ),
                    )
                {
                    Some(right_rec)
                } else {
                    left_rec
                }
            } else {
                None
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
