mod camera;
mod geometry;
mod hittable;
mod material;

use std::{f64, rc::Rc};

use rand::{random, random_range};

use crate::{
    camera::Camera,
    geometry::{Color, Point3, Vec3},
    hittable::{sphere::Sphere, World},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
};

fn setup_scattered_balls(world: &mut World) {
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)).into());
    world.add(Sphere::new(
        Point3::new(0, -1000, 0),
        1000,
        Rc::clone(&ground_material),
    ));

    for a in -11..=11 {
        for b in -11..=11 {
            let mat = random_range(0..20);
            let center = Point3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if Vec3::length(center - Point3::new(4, 0.2, 0)) > 0.9 {
                let sphere_material = Rc::new(if mat < 16 {
                    let albedo = Color::random() * Color::random();
                    Lambertian::new(albedo).into()
                } else if mat < 19 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range(0.0..0.5);
                    Metal::new(albedo, fuzz).into()
                } else {
                    Dielectric::new(1.5).into()
                });
                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5).into());
    world.add(Sphere::new(
        Point3::new(0, 1, 0),
        1.0,
        Rc::clone(&material1),
    ));

    // let material2 = Rc::new(Dielectric::new(1.0 / 1.5).into());
    // world.add(Sphere::new(
    //     Point3::new(0, 1, 0),
    //     0.9,
    //     Rc::clone(&material2),
    // ));

    let material3 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)).into());
    world.add(Sphere::new(
        Point3::new(-4, 1, 0),
        1.0,
        Rc::clone(&material3),
    ));

    let material4 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.1).into());
    world.add(Sphere::new(
        Point3::new(4, 1, 0),
        1.0,
        Rc::clone(&material4),
    ));
}

fn main() {
    let mut world = World::empty();

    setup_scattered_balls(&mut world);

    let camera = Camera::new(
        16. / 9.,              // aspect ratio
        400,                  // image width
        10,                   // samples per pixel
        50,                    // max depth
        20,                    // fov
        Point3::new(13, 2, 3), // look at
        Point3::new(0, 0, 0),  // look from
        Vec3::new(0, 1, 0),    // camera up
        0.6,                   // defocus angle
        10,                    // focus distance
    );

    camera.render(&world, "output/render.png");
}
