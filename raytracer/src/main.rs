//list modules that are part of the project - not an import closer to a makefile - only in main
mod colour;
mod ray;
mod vec3;

//Lets us use names locally
use crate::{
    colour::*,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::io::stdout;

fn ray_colour(ray: Ray) -> Colour {
    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a)*Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
}

fn main() {
    //Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    //Calculate image height
    let mut image_height = (image_width as f64 / aspect_ratio) as usize;
    image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_centre = Point3::zero();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let view_port_upper_left =
        camera_centre - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = view_port_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut out = stdout();

    println!("P3\n{image_width} {image_height}\n255");
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_centre =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_centre - camera_centre;
            let r = Ray::new(camera_centre, ray_direction);
            let pixel_colour = ray_colour(r);

            write_colour(&mut out, pixel_colour);
        }
    }
    println!("\rDone                                    \n");
}
