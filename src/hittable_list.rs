use crate::{hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray};

pub struct HittableList {
    list: Vec<Hittable>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {list: Vec::new()}
    }
    pub fn add(&mut self, object: Hittable) {
        self.list.push(object);
    }
}

impl HittableList {
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_rec = None::<HitRecord>;
        let mut closest = ray_t.max;

        for object in &self.list {
            if let Some(x) = object.hit(ray, Interval::new(ray_t.min, closest)) {
                closest = x.t;
                temp_rec = Some(x);
            }
        }
        
        temp_rec
    }
}