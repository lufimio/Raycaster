mod camera;
mod geometry;
mod hittable;
mod material;
mod texture;

use crate::{
    camera::Camera,
    geometry::{Color, Point3, Vec3},
    hittable::{HittableList, bvh::BVHNode, quad::Quad, sphere::Sphere},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    texture::{Texture, checker::Checker, image::Image},
};
use rand::{random, random_range};
use std::{f32, sync::Arc};

fn setup_scattered_balls() {
    let mut world = HittableList::empty();

    let ground_texture = Arc::new(
        Checker::new(
            0.32,
            Arc::new(Color::new(0.2, 0.3, 0.1).into()),
            Arc::new(Color::new(0.9, 0.9, 0.9).into()),
        )
        .into(),
    );
    let ground_material = Arc::new(Lambertian::new(Arc::clone(&ground_texture)).into());
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
                    let albedo: Arc<Texture> =
                        Arc::new((random::<Color>() * random::<Color>()).into());
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

    let material3 = Arc::new(Lambertian::new(Arc::new(Color::new(0.4, 0.2, 0.1).into())).into());
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

fn setup_quads() {
    let mut world = HittableList::empty();

    let left_red = Arc::new(Lambertian::new(Arc::new(Color::new(1.0, 0.2, 0.2).into())).into());
    world.add(Quad::new(
        Point3::new(-3., -2., 5.),
        Vec3::new(0., 0., -4.),
        Vec3::new(0., 4., 0.),
        Arc::clone(&left_red),
    ));

    let back_green = Arc::new(Lambertian::new(Arc::new(Color::new(0.2, 1.0, 0.2).into())).into());
    world.add(Quad::new(
        Point3::new(-2., -2., 0.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 4., 0.),
        Arc::clone(&back_green),
    ));

    let right_blue = Arc::new(Lambertian::new(Arc::new(Color::new(0.2, 0.2, 1.0).into())).into());
    world.add(Quad::new(
        Point3::new(3., -2., 1.),
        Vec3::new(0., 0., 4.),
        Vec3::new(0., 4., 0.),
        Arc::clone(&right_blue),
    ));

    let upper_orange = Arc::new(Lambertian::new(Arc::new(Color::new(1.0, 0.5, 0.0).into())).into());
    world.add(Quad::new(
        Point3::new(-2., 3., 1.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., 4.),
        Arc::clone(&upper_orange),
    ));

    let lower_teal = Arc::new(Lambertian::new(Arc::new(Color::new(0.2, 0.8, 0.8).into())).into());
    world.add(Quad::new(
        Point3::new(-2., -3., 5.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., -4.),
        Arc::clone(&lower_teal),
    ));

    let earth_texture = Arc::new(Image::new("assets/earthmap.jpg").into());
    let earth_surface = Arc::new(Lambertian::new(earth_texture).into());
    world.add(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.25, earth_surface));

    let camera = Camera::new(
        1.0,                     // aspect ratio
        1200,                    // image width
        100,                     // samples per pixel
        50,                      // max depth
        70.0,                    // fov
        Point3::new(0., 0., 9.), // look from
        Point3::new(0., 0., 0.), // look at
        Vec3::new(0., 1., 0.),   // camera up
        0.0,                     // defocus angle
        10.0,                    // focus distance
    );

    camera.render(&world, "output/render.png");
}

fn setup_earth() {
    let earth_texture = Arc::new(Image::new("assets/earthmap.jpg").into());
    let earth_surface = Arc::new(Lambertian::new(earth_texture).into());
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    let camera = Camera::new(
        16.0 / 9.0,                // aspect ratio
        1200,                      // image width
        100,                       // samples per pixel
        50,                        // max depth
        20.0,                      // fov
        Point3::new(-4., 12., 5.), // look from
        Point3::new(0., 0., 0.),   // look at
        Vec3::new(0., 1., 0.),     // camera up
        0.0,                       // defocus angle
        10.0,                      // focus distance
    );

    camera.render(&globe, "output/render.png");
}

fn main() {
    match 2 {
        1 => setup_scattered_balls(),
        2 => setup_quads(),
        3 => setup_earth(),
        _ => (),
    }
}
