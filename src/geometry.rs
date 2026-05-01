use glam::Vec3;
use image::Rgb;
use rand::{random, random_range};
use std::{f32, ops::Add};

pub fn linear_to_gamma(linear: f32) -> f32 {
    if linear > 0.0 { linear.sqrt() } else { 0.0 }
}

pub fn gamma_to_linear(gamma: f32) -> f32 {
    gamma * gamma
}

pub fn random_in_unit_disk() -> Vec3 {
    let a = random_range(0.0..(2.0 * f32::consts::PI));
    Vec3::new(a.cos(), a.sin(), 0.0)
}

pub fn random_unit_vector() -> Vec3 {
    let y: f32 = random_range(-1.0..1.0);
    let r: f32 = (1. - y * y).sqrt();
    let long: f32 = random_range(-f32::consts::PI..f32::consts::PI);
    Vec3::new(r * long.sin(), y, r * long.cos())
}

pub fn random_cosine_direction() -> Vec3 {
    let r1: f32 = random();
    let r2: f32 = random();

    let phi = 2. * f32::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    let z = f32::sqrt(1. - r2);

    Vec3::new(x, y, z)
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min: min.into(),
            max: max.into(),
        }
    }

    pub fn empty() -> Self {
        Self {
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
        }
    }

    pub fn all() -> Self {
        Self {
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }

    pub fn containing(a: Interval, b: Interval) -> Interval {
        Interval::new(f32::min(a.min, b.min), f32::max(a.max, b.max))
    }

    pub fn expand(self, delta: f32) -> Interval {
        Interval::new(self.min - delta / 2., self.max + delta / 2.)
    }

    pub fn size(self) -> f32 {
        self.max - self.min
    }

    pub fn contains(self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }
}

impl Add<f32> for Interval {
    type Output = Interval;

    fn add(self, rhs: f32) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

impl Add<Interval> for f32 {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Self::Output {
        rhs + self
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn color_to_rgb(color: Color) -> Rgb<u8> {
    let r = color.x.is_nan().then_some(0.0).unwrap_or(color.x);
    let g = color.y.is_nan().then_some(0.0).unwrap_or(color.y);
    let b = color.z.is_nan().then_some(0.0).unwrap_or(color.z);

    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    let intensity = Interval::new(0.0, 0.999);
    Rgb([
        (256. * intensity.clamp(r)) as u8,
        (256. * intensity.clamp(g)) as u8,
        (256. * intensity.clamp(b)) as u8,
    ])
}

pub fn rgb_to_color(rgb: Rgb<u8>) -> Color {
    let r = gamma_to_linear(rgb[0] as f32 / 255.0);
    let g = gamma_to_linear(rgb[1] as f32 / 255.0);
    let b = gamma_to_linear(rgb[2] as f32 / 255.0);
    Color::new(r, g, b)
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
