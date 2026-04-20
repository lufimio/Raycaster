use image::{ImageResult, Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    geometry::{
        Color, Interval, Point3, Ray, Vec3, random_in_unit_disk, random_on_hemisphere,
        sample_square,
    },
    hittable::World,
    material::Scatterable,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub fov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub cameraup: Vec3,
    pub defocus_angle: f64,
    pub focus_distance: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pixel00_loc: Point3,
    pixel_delta_u: Point3,
    pixel_delta_v: Point3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new<T: Into<f64>, U: Into<f64>, V: Into<f64>>(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        fov: T,
        lookfrom: Point3,
        lookat: Point3,
        cameraup: Vec3,
        defocus_angle: U,
        focus_distance: V,
    ) -> Self {
        let fov = fov.into();
        let defocus_angle = defocus_angle.into();
        let focus_distance = focus_distance.into();

        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let viewport_height = 2.0 * f64::tan(fov.to_radians() / 2.0) * focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (lookfrom - lookat).normalized();
        let u = Vec3::cross(cameraup, w).normalized();
        let v = Vec3::cross(w, u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_top_left = lookfrom - focus_distance * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_top_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = focus_distance * f64::tan(f64::to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio,
            fov,
            image_width,
            image_height,
            lookfrom,
            lookat,
            cameraup,
            defocus_angle,
            focus_distance,
            samples_per_pixel,
            max_depth,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &World, output_path: &str) {
        let bar = ProgressBar::new(
            (self.image_height * self.image_width * self.samples_per_pixel) as u64,
        )
        .with_style(
            ProgressStyle::with_template(
                "{wide_bar} {pos}/{len} [{elapsed_precise}/-{eta_precise}]",
            )
            .unwrap(),
        );

        let mut img = RgbImage::new(self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + self.get_ray_color(r, self.max_depth, &world);
                    bar.inc(1);
                }
                pixel_color = pixel_color / self.samples_per_pixel as f64;
                img.put_pixel(i, j, pixel_color.to_rgb());
            }
        }

        if let ImageResult::Err(error) = img.save(output_path) {
            eprintln!("Error writing image: {}", error)
        }

        bar.finish();
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = 0.5 * random_in_unit_disk();
        let pixel_center = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        Ray::new(
            if self.defocus_angle <= 0.0 {
                self.lookfrom
            } else {
                self.defocus_disk_sample()
            },
            pixel_center - self.lookfrom,
        )
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.lookfrom + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    fn get_ray_color(&self, r: Ray, depth: u32, world: &World) -> Color {
        if depth <= 0 {
            return Color::zero();
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
                return attenuation * self.get_ray_color(scattered, depth - 1, world);
            }
            return Color::zero();
        }

        let a = 0.5 * (r.direction.normalized().y + 1.0);
        (1.0 - a) * Color::new(1, 1, 1) + a * Color::new(0.5, 0.7, 1)
    }
}
