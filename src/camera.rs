use std::{fs::File, io::{BufWriter, Write}, sync::{mpsc, Arc, Mutex}, thread, usize};

use indicatif::ProgressBar;

use crate::{color::Color, hittable_list::HittableList, interval::Interval, ray::Ray, utils::{degrees_to_radians, sample_square, ThreadPool}, vec3::{cross, Vec3}};


#[derive(Debug, Clone, Copy)]
pub struct Camera {
    image_width: usize,
    image_hight: usize,
    aspect_ratio: f64,

    thread_count: usize,
    max_depth: usize,
    samples_per_pixel: usize,

    focal_length: f64,
    veiwport_hight: f64,
    veiwport_width: f64,

    depth_of_field_blur: f64,

    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    camera_center: Vec3,
}

impl Camera {

    pub fn render(self, world: HittableList) {
        
        let mut file_buf = BufWriter::new(File::create("image.ppm").expect("Could not create file"));
        file_buf.write_all(format!("P3\n{} {}\n255\n", self.image_width, self.image_hight).as_bytes()).expect("Could not write to file");
        
        let pool = ThreadPool::new(self.thread_count);
        let (sender, receiver) = mpsc::channel::<(Color, usize)>();
        
        let receiver = Mutex::new(receiver);
        
        let max_depth = self.max_depth;
        let samples = self.samples_per_pixel;

        let world_arc = Arc::new(world);
        
        let mut image: Vec<Color> = vec![Color::new(0.0, 0.0, 0.0); self.image_hight*self.image_width];
        let t = thread::spawn(move || {
            for _ in 0..(samples * image.len()) {
                let (c, pixel) = receiver.lock().unwrap().recv().unwrap();
                
                image[pixel] = image[pixel] + c;
            }

            for c in image {
                (c / samples as f64).write(&mut file_buf);
            }
        });


        let pb = ProgressBar::new(self.image_hight as u64);
        
        let mut pixel = 0;
        for h in 0..self.image_hight {
            for w in 0..self.image_width {
                
                let mut que = Vec::with_capacity(self.samples_per_pixel);
                
                for _ in 0..self.samples_per_pixel {
                    let new_ray = self.get_ray(w as f64, h as f64).unit();
                    let world_clone = world_arc.clone();
                    let sender = sender.clone();
                    que.push(move || {
                        sender.send((Camera::ray_color(&new_ray, world_clone, max_depth), pixel)).unwrap();
                    });
                }
                pool.execute(que);
                
                pixel += 1;
                
            }
            pb.inc(1);
        }

        t.join().unwrap();

        pb.finish_with_message("Done!");
        
    }

    fn get_ray(&self, w: f64, h: f64) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc + (self.pixel_delta_u * (w + offset.x)) + (self.pixel_delta_v * (h + offset.y));

        let blur = Vec3::random_vector() * self.depth_of_field_blur;

        Ray::new(self.camera_center + blur, pixel_sample - self.camera_center - blur)
    }

    fn ray_color(ray: &Ray, world: Arc<HittableList>, depth: usize) -> Color {
        if depth == 0 {return Color::new(0.0, 0.0, 0.0)}

        if let Some(x) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            let mat = x.mat.scatter(ray, &x);
            return Self::ray_color(&mat.scattered, world, depth-1) * mat.attenuation;
        }
        let hight = (ray.direction.unit().y + 1.0) * 0.5;
        (Color::new(1.0, 1.0, 1.0) * (1.0-hight)) + (Color::new(0.5, 0.7, 1.0) * hight)
    }
}


#[derive(Default, Clone, Copy)]
pub struct CameraBuilder {
    pub image_width: usize,
    pub image_hight: usize,
    pub aspect_ratio: f64,
    pub vfov: f64,
    pub camera_center: Vec3,
    pub look_at: Vec3,
    pub depth_of_field_blur: f64,

    pub thread_count: usize,
    pub max_depth: usize,
    pub samples_per_pixel: usize,
}

impl CameraBuilder {
    pub fn to_camera(self) -> Camera {
        if self.thread_count == 0 {
            panic!("`thread_count` cannot be zero");
        }
        if self.max_depth == 0 {
            panic!("`max_depth` cannot be zero");
        }
        if self.samples_per_pixel == 0 {
            panic!("`samples_per_pixel` cannot be zero");
        }

        let zero_aspect_ratio = self.aspect_ratio == 0.0;

        let image_hight;
        let image_width;
        let aspect_ratio;
        match (self.image_hight, self.image_width, zero_aspect_ratio) {
            (0, w, false) => {
                image_hight = ((w as f64)/self.aspect_ratio) as usize;
                image_width = w;
                aspect_ratio = self.aspect_ratio;
            },
            (h, 0, false) => {
                image_width = ((h as f64)*self.aspect_ratio) as usize;
                image_hight = h;
                aspect_ratio = self.aspect_ratio;
            },
            (h, w, true) => {
                aspect_ratio = w as f64 / h as f64;
                image_width = w;
                image_hight = h;
            },
            _ => panic!("Must have two of the following be non-zero: `image_hight`, `image_width`, or `aspect_ratio`"),
        }

        // viewport dimentions
        let focal_length = (self.camera_center - self.look_at).len();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta/2.0).tan();
        let veiwport_hight = 2.0 * h * focal_length;
        let veiwport_width = veiwport_hight * aspect_ratio;

        let vup = Vec3::new(0.0, 1.0, 0.0);

        let w = (self.camera_center - self.look_at).unit(); // forward
        let u = cross(vup, w).unit(); // camera right
        let v = cross(w, u); // camera up

        let viewport_u = u * veiwport_width;
        let viewport_v = v * -veiwport_hight;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_hight as f64;

        let veiwport_upper_left = self.camera_center - (w * focal_length) - (viewport_u/2.0) - (viewport_v/2.0); // top left of the screen
        let pixel00_loc = veiwport_upper_left + ((pixel_delta_u+pixel_delta_v) * 0.5); // top left pixel

        Camera { image_width, image_hight, aspect_ratio, thread_count: self.thread_count, max_depth: self.max_depth, samples_per_pixel: self.samples_per_pixel, pixel00_loc, pixel_delta_u, pixel_delta_v, camera_center: self.camera_center, focal_length, veiwport_hight, veiwport_width, depth_of_field_blur: self.depth_of_field_blur }
    }
}