// camera.rs

use std::io::stdout;

use crate::{
    colour::{Colour, write_colour},
    dot,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    util::random_f64,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,        //default = 1.0   Ratio of image width over height
    pub image_width: usize,       //default = 100   Rendered image width in pixel count
    pub samples_per_pixel: usize, //default = 10    Count of random samples for each pixel
    pub max_depth: usize,         //default = 10    Maximum number of bounces into scene

    //Private variables
    image_height: usize,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,

            //private variables
            image_height: 0,
            centre: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            pixel_samples_scale: 0.0,
        }
    }
    pub fn render(&mut self, world: &impl Hittable) {
        self.initialise();
        let mut out = stdout();
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}     ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_colour += self.ray_colour(&r, self.max_depth, world);
                }
                write_colour(&mut out, self.pixel_samples_scale * pixel_colour);
            }
        }
        eprint!("\rDone                                    \n");
    }
    fn initialise(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.centre = Point3::zero();
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let view_port_upper_left =
            self.centre - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = view_port_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.centre;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn ray_colour(&self, r: &Ray, depth: usize, world: &impl Hittable) -> Colour {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            let direction = rec.normal + Vec3::random_unit_vector();
            return 0.1 * self.ray_colour(&Ray::new(rec.p, direction), depth - 1, world); //The first decimal is percentage reflectance
        }
        let unit_direction: Vec3 = r.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }
}
