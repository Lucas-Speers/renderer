use std::{fs::File, io::{BufWriter, Write}, ops::{Add, Div, Mul}};

use crate::interval::Interval;

#[derive(Clone, Debug, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b}
    }

    fn to_gamma(linear: f64) -> f64 {
        if linear > 0.0 {
            return linear.powf(0.5);
        }

        return 0.0;
    }

    pub fn write(&self, file: &mut BufWriter<File>) {
        let i = Interval::new(0.0, 0.999);
        let r = (i.clamp(Self::to_gamma(self.r)) * 256.0) as u8;
        let g = (i.clamp(Self::to_gamma(self.g)) * 256.0) as u8;
        let b = (i.clamp(Self::to_gamma(self.b)) * 256.0) as u8;
        file.write_all(format!("{r} {g} {b}\n").as_bytes()).expect("Could not write to file");
    }
}

impl Add<Color> for Color {
    type Output = Color;
    
    fn add(self, rhs: Color) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: (self.r * rhs),
            g: (self.g * rhs),
            b: (self.b * rhs),
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Self {
            r: (self.r * rhs.r),
            g: (self.g * rhs.g),
            b: (self.b * rhs.b),
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            r: (self.r / rhs),
            g: (self.g / rhs),
            b: (self.b / rhs),
        }
    }
}