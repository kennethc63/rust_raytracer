//list modules that are part of the project - not an import closer to a makefile - only in main
mod camera;
mod colour;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

//Lets us use names locally
use crate::{
    camera::Camera,
    hittable_list::HittableList,
    sphere::Sphere,
    vec3::{Point3, dot},
};

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let mut cam = Camera::new(); // ???
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.render(&world);
}
