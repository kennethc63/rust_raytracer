// hittable.rs

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f64, p: Point3, normal: Vec3) -> HitRecord {
        HitRecord { t, p, normal }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
