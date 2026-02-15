// camera.rs

use std::io::stdout;

use crate::{
    colour::{Colour, write_colour},
    dot,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,  //default = 1.0
    pub image_width: usize, //default = 100

    //Private variables
    image_height: usize,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            centre: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        }
    }
    pub fn render(&mut self, world: &impl Hittable) {
        self.initialise();
        let mut out = stdout();
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_centre = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_centre - self.centre;
                let r = Ray::new(self.centre, ray_direction);
                let pixel_colour = self.ray_colour(&r, world);

                write_colour(&mut out, pixel_colour);
            }
        }
        println!("\rDone                                    \n");
    }
    fn initialise(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

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

    fn ray_colour(&self, r: &Ray, world: &impl Hittable) -> Colour {
        if let Some(rec) = world.hit(r, Interval::new(0.0, f64::INFINITY)) {
            return 0.5 * (Colour::from(rec.normal) + Colour::new(1.0, 1.0, 1.0));
        }
        let unit_direction: Vec3 = r.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }
}
