use crate::vec3::Vec3;


/// Note to future self: the ray direction is relative to the ray origin and NOT 0,0,0!
/// 
/// AKA: changing the ray origin will NOT change the direction the ray is pointing
#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Vec3, direction: Vec3) -> Self  {
        Self {origin, direction}
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction*t
    }
    pub fn unit(&self) -> Self {
        Self {
            origin: self.origin,
            direction: self.direction.unit(),
        }
    }
}