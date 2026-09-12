// sphere.rs

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere<'a> {
    pub centre: Ray,
    pub radius: f64,
    pub mat: Box<dyn Material + 'a>, //Sphere owns the material its made of
}

impl<'a> Sphere<'a> {
    // Static Sphere
    pub fn new(centre: Point3, radius: f64, mat: impl Material + 'a) -> Sphere<'a> {
        Sphere {
            centre: Ray::new(centre, Vec3::zero()),
            radius: radius.max(0.0),
            mat: Box::new(mat),
        }
    }

    pub fn new_moving(
        centre1: Point3,
        centre2: Point3,
        radius: f64,
        mat: impl Material + 'a,
    ) -> Sphere<'a> {
        Sphere {
            centre: Ray::new(centre1, centre2 - centre1),
            radius: radius.max(0.0),
            mat: Box::new(mat),
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_centre = self.centre.at(r.time);
        let oc = current_centre - r.origin;
        let a = r.direction.length_squared();
        let h = Vec3::dot(r.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - current_centre) / self.radius;
        let mut rec = HitRecord::new(t, p, outward_normal, &*self.mat); // * removes from Box, & borrow
        rec.set_face_normal(r);
        Some(rec)
    }
}
