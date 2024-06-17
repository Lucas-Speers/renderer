use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::random;

#[derive(Clone, Copy, Debug)]
#[derive(Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {x, y, z}
    }
    pub const fn zero() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
    pub fn len(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).powf(0.5)
    }
    pub fn len_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }
    pub fn unit(&self) -> Self {
        *self / self.len()
    }
    pub fn dot(&self, other: Self) -> f64 {
        (self.x * other.x) +
        (self.y * other.y) +
        (self.z * other.z)
    }
    pub fn random_vector() -> Self {
        loop {
            let v = Vec3::new(random::<f64>()-0.5, random::<f64>()-0.5, random::<f64>()-0.5)*2.0;
            if v.len_squared() < 1.0 {
                return v;
            }
        }
    }
    pub fn _random_on_hemisphere(normal: Vec3) -> Vec3 {
        let v = Self::random_vector();
        if normal.dot(v) > 0.0 {v} else {-v}
    }
    pub fn near_zero(&self) -> bool {
        let near_zero = 1e-8;
        (self.x.abs() < near_zero) && (self.y.abs() < near_zero) && (self.z.abs() < near_zero)
    }
    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - normal*self.dot(normal)*2.0
    }
    pub fn refract(&self, normal: Vec3, ri: f64) -> Vec3 {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let perp = (*self + normal*cos_theta) * ri;
        let para = normal * -(1.0 - perp.len_squared()).abs().powf(0.5);
        perp + para
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    (a.x * b.x) +
    (a.y * b.y) +
    (a.z * b.z)
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x
    )
}

impl Add for Vec3 {
    type Output = Vec3;
    
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}