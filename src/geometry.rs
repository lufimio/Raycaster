use std::{
    f64,
    fmt::Display,
    ops::{Add, Div, Index, Mul, Neg, Sub},
};

use rand::{random, random_range};

pub fn sample_square() -> Vec3 {
    Vec3::new(random::<f64>() - 0.5, random::<f64>() - 0.5, 0)
}

pub fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 { linear.sqrt() } else { 0.0 }
}

pub fn random_unit_vector() -> Vec3 {
    let y: f64 = random_range(-1.0..1.0);
    let r: f64 = (1. - y * y).sqrt();
    let long: f64 = random_range(-f64::consts::PI..f64::consts::PI);
    Vec3::new(r * long.sin(), y, r * long.cos())
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let unit = random_unit_vector();
    if unit.dot(normal) > 0.0 { unit } else { -unit }
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new<S: Into<f64>, B: Into<f64>>(min: S, max: B) -> Self {
        Self {
            min: min.into(),
            max: max.into(),
        }
    }

    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn all() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn size(self) -> f64 {
        self.max - self.min
    }

    pub fn contains<T: Into<f64>>(self, x: T) -> bool {
        let x = x.into();
        self.min <= x && x <= self.max
    }

    pub fn surrounds<T: Into<f64>>(self, x: T) -> bool {
        let x = x.into();
        self.min < x && x < self.max
    }

    pub fn clamp(self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn random() -> Self {
        Self {
            x: random(),
            y: random(),
            z: random(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            x: random_range(min..max),
            y: random_range(min..max),
            z: random_range(min..max),
        }
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> Mul<T> for Vec3 where T: Into<f64> {
    type Output = Vec3;

    fn mul(self, other: T) -> Self::Output {
        let other = other.into();
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl<T> Div<T> for Vec3 where T: Into<f64> {
    type Output = Vec3;

    fn div(self, other: T) -> Self::Output {
        self * (1. / other.into())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index has to be between 0-2 inclusive."),
        }
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Color {
    pub fn write_ppm(self) {
        let r = linear_to_gamma(self.x);
        let g = linear_to_gamma(self.y);
        let b = linear_to_gamma(self.z);

        let intensity = Interval::new(0, 0.999);
        println!(
            "{} {} {}",
            (256. * intensity.clamp(r)) as u8,
            (256. * intensity.clamp(g)) as u8,
            (256. * intensity.clamp(b)) as u8,
        );
    }
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

    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
