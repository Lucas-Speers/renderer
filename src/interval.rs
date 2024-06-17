
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn _empty() -> Self {
        Self { min: f64::INFINITY, max: -f64::INFINITY }
    }
    pub fn everything() -> Self {
        Self { min: -f64::INFINITY, max: f64::INFINITY }
    }
    pub fn _contains(&self, value: f64) -> bool {
        (self.min <= value) && (value <= self.max)
    }
    pub fn surrounds(&self, value: f64) -> bool {
        (self.min < value) && (value < self.max)
    }
    pub fn clamp(&self, value: f64) -> f64 {
        if value < self.min {self.min} else if value > self.max {self.max} else {value}
    }
}