//list modules that are part of the project - not an import closer to a makefile - only in main
mod camera;
mod colour;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

//Lets us use names locally
use crate::{
    camera::Camera,
    colour::Colour,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Colour::new(0.8, 0.8, 0.0));
    let material_centre = Lambertian::new(Colour::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.50);
    let material_bubble = Dielectric::new(1.00 / 1.50);
    let material_right = Metal::new(Colour::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_centre,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;

    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}
