use rand::random;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub struct MaterialRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> MaterialRecord {
        match self {
            Material::Metal(metal) => metal.scatter(ray, rec),
            Material::Lambertian(lambertian) => lambertian.scatter(ray, rec),
            Material::Dielectric(dielectric) => dielectric.scatter(ray, rec),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Material {
        Material::Metal(Metal { albedo, fuzz })
    }
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> MaterialRecord {
            
        let reflected = ray.direction.reflect(rec.normal) + (Vec3::random_vector() * self.fuzz);

        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo.clone();
        
        MaterialRecord { attenuation, scattered }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> MaterialRecord {

        let mut scatter_direction = rec.normal + Vec3::random_vector().unit();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        
        MaterialRecord { attenuation, scattered }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Material {
        Material::Dielectric(Dielectric { refraction_index })
    }
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> MaterialRecord {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        let ri = if rec.front_face {1.0/self.refraction_index} else {self.refraction_index};
        let cos_theta = (-ray.direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).powf(0.5);

        let cannot_refract = ri * sin_theta > 1.0;

        let direction: Vec3;

        if cannot_refract || Self::reflectance(cos_theta, ri) > random::<f64>() {
            direction = ray.direction.reflect(rec.normal);
        } else {
            direction = ray.direction.refract(rec.normal, ri);
        }

        let scattered = Ray::new(rec.p, direction);
        
        MaterialRecord { attenuation, scattered }
    }

    fn reflectance(cos: f64, ri: f64) -> f64 {
        let mut r0 = (1.0 - ri) / (1.0 + ri);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
    }
}

