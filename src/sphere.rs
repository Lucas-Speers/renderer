use crate::{hittable::{HitRecord, Hittable}, interval::Interval, material::Material, ray::Ray, vec3::{dot, Vec3}};

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, matterial: Material) -> Hittable {
        Hittable::Sphere(Self {center, radius, mat: matterial})
    }
}

impl Sphere {
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.len_squared();
        let h = dot(ray.direction, oc);
        let c = oc.len_squared() - (self.radius*self.radius);
        let discriminant = (h*h) - (a*c);
        
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.powf(0.5);

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = ray.at(root);
        let mut normal = (ray.at(root) - self.center) / self.radius;

        let mut front_face = true;
        if ray.direction.dot(normal) > 0.0 {
            front_face = false;
            normal = -normal;
        }

        let hit = HitRecord {
            p,
            normal,
            t: root,
            front_face,
            mat: self.mat
        };

        Some(hit)
    }
}