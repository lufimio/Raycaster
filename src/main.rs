mod camera;
mod geometry;
mod hittable;
mod material;

use std::{f32, sync::Arc};
use rand::{random, random_range};
use crate::{
    camera::Camera,
    geometry::{Color, Point3, Vec3},
    hittable::{HittableList, bvh::BVHNode, sphere::Sphere},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
};

fn setup_scattered_balls() {
    let mut world = HittableList::empty();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)).into());
    world.add(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::clone(&ground_material),
    ));

    for a in -11..=11 {
        for b in -11..=11 {
            let mat = random_range(0..20);
            let center = Point3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if Vec3::length(center - Point3::new(4., 0.2, 0.)) > 0.9 {
                let sphere_material = Arc::new(if mat < 16 {
                    let albedo = random::<Color>() * random::<Color>();
                    Lambertian::new(albedo).into()
                } else if mat < 19 {
                    let albedo = 0.5 + random::<Color>() * 0.5;
                    let fuzz = random_range(0.0..0.5);
                    Metal::new(albedo, fuzz).into()
                } else {
                    Dielectric::new(1.5).into()
                });
                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5).into());
    world.add(Sphere::new(
        Point3::new(0., 1., 0.),
        1.0,
        Arc::clone(&material1),
    ));

    // let material2 = Arc::new(Dielectric::new(1.0 / 1.5).into());
    // world.add(Sphere::new(
    //     Point3::new(0., 1., 0.),
    //     0.8,
    //     Arc::clone(&material2),
    // ));

    let material3 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)).into());
    world.add(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.0,
        Arc::clone(&material3),
    ));

    let material4 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.1).into());
    world.add(Sphere::new(
        Point3::new(4., 1., 0.),
        1.0,
        Arc::clone(&material4),
    ));

    let world = BVHNode::from_hittable_list(world);

    let camera = Camera::new(
        16. / 9.,                 // aspect ratio
        400,                      // image width
        50,                       // samples per pixel
        50,                       // max depth
        20.0,                     // fov
        Point3::new(13., 2., 3.), // look from
        Point3::new(0., 0., 0.),  // look at
        Vec3::new(0., 1., 0.),    // camera up
        0.6,                      // defocus angle
        10.0,                     // focus distance
    );

    camera.render(&world, "output/render.png");
}

fn main() {
    match 1 {
        1 => setup_scattered_balls(),
        _ => ()
    }
}
