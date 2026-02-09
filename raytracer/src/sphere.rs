// sphere.rs

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{Point3, dot},
};

pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64) -> Sphere {
        Sphere { centre, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.centre - r.origin;
        let a = r.direction.length_squared();
        let h = dot(r.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let normal = (p - self.centre) / self.radius;
        let mut rec = HitRecord::new(t, p, normal);
        rec.set_face_normal(r);
        Some(rec)
    }
}
