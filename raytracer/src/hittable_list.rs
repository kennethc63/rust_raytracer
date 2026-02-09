// hittable_list.rs

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

//sort of like creating an array in memory so needs to know the size of what is going in
//Box is a memory pointer to the heap
//we are storing a list of pointers to the objects
//hittable list owns all the objects so when it gets destroyed the objects all also get destroyed
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: impl Hittable + 'static) {
        //what is static?
        self.objects.push(Box::new(object))
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = ray_tmax;
        for object in self.objects.iter() {
            if let Some(hrec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = hrec.t;
                rec = Some(hrec);
            }
        }
        rec
    }
}
