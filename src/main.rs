#![allow(unused)]

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod utils;
mod material;

use color::Color;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use sphere::Sphere;
use vec3::Vec3;
use camera::CameraBuilder;


fn main() {
    let mut world = HittableList::new();
    let matte_blue = Material::Lambertian(Lambertian { albedo: Color::new(0.1, 0.2, 0.5) });
    let matte_yellow = Material::Lambertian(Lambertian { albedo: Color::new(0.8, 0.7, 0.0) });
    let matte_white = Material::Lambertian(Lambertian { albedo: Color::new(1.0, 1.0, 1.0) });
    let metal = Material::Metal(Metal { albedo: Color::new(0.8, 0.8, 0.8), fuzz: 0.0 });
    let metal_fuzzy = Material::Metal(Metal { albedo: Color::new(0.8, 0.8, 0.8), fuzz: 1.0 });
    let glass = Material::Dielectric(Dielectric { refraction_index: 1.5 });
    let glass_inside = Material::Dielectric(Dielectric { refraction_index: 0.75 });

    world.add(Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, matte_yellow)); // floor
    world.add(Sphere::new(Vec3::new( 0.0, 0.0,    -1.2), 0.5,   matte_blue));   // center
    world.add(Sphere::new(Vec3::new(-1.0, 0.0,    -1.0), 0.5,   glass));        // left
    world.add(Sphere::new(Vec3::new(-1.0, 0.0,    -1.0), 0.4,   glass_inside)); // left (inside)
    world.add(Sphere::new(Vec3::new( 1.0, 0.0,    -1.0), 0.5,   metal_fuzzy));  // right

    let camera = CameraBuilder {
        camera_center: Vec3::new(-2.0, 2.0, 1.0),
        look_at: Vec3::new(0.0, 0.0, -1.0),
        image_width: 800,
        aspect_ratio: 16.0/9.0,
        vfov: 50.0,
        depth_of_field_blur: 0.15,
        thread_count: 32,
        max_depth: 50,
        samples_per_pixel: 100,
        ..Default::default()
    }.to_camera();

    camera.render(world);

    println!("Done!");
}
