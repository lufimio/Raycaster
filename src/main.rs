mod camera;
mod geometry;
mod hittable;
mod material;

use std::rc::Rc;

use crate::{
    camera::Camera,
    geometry::{Color, Point3},
    hittable::{World, sphere::Sphere},
    material::{lambertian::Lambertian, metal::Metal},
};

fn main() {
    let mut world = World::empty();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0)).into());
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)).into());
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3).into());
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0).into());

    world.add(Sphere::new(
        Point3::new(0, -100.5, -1),
        100,
        Rc::clone(&material_ground),
    ));
    world.add(Sphere::new(
        Point3::new(0, 0, -1.2),
        0.5,
        Rc::clone(&material_center),
    ));
    world.add(Sphere::new(
        Point3::new(-1, 0, -1),
        0.5,
        Rc::clone(&material_left),
    ));
    world.add(Sphere::new(
        Point3::new(1, 0, -1),
        0.5,
        Rc::clone(&material_right),
    ));

    let camera = Camera::new(16. / 9., 400, 100, 50);
    camera.render(&world);
}
