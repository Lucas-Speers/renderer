use crate::{interval::Interval, material::Material, ray::Ray, sphere::Sphere, vec3::Vec3};

#[derive(Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Material,
}

#[derive(Clone, Copy)]
pub enum Hittable {
    Sphere(Sphere),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, ray_t),
        }
    }
}