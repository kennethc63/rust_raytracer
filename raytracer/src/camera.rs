// camera.rs

use std::io::stdout;

use crate::{
    colour::{Colour, write_colour},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    util::random_f64,
    vec3::{Point3, Vec3},
};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,        //default = 1.0   Ratio of image width over height
    pub image_width: usize,       //default = 100   Rendered image width in pixel count
    pub samples_per_pixel: usize, //default = 10    Count of random samples for each pixel
    pub max_depth: usize,         //default = 10    Maximum number of bounces into scene
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,

    //Private variables
    image_height: usize,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::zero(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            ..Default::default()
        }
        /*

            //private variables
            image_height: 0,
            centre: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            pixel_samples_scale: 0.0,
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),

        }
        */
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

        self.centre = self.lookfrom;

        // Determine viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = (Vec3::cross(self.vup, self.w)).unit_vector();
        self.v = Vec3::cross(self.w, self.u);

        // Calculate vectors across the horizontal and down the viewport edges
        let viewport_u = viewport_width * self.u; // Vector across viewport horizonal edge
        let viewport_v = viewport_height * -self.v; // Vector down viewport vertical edge

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel
        let view_port_upper_left =
            self.centre - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = view_port_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus basis vectors
        let defocus_radius = self.focus_dist * ((self.defocus_angle / 2.0).to_radians()).tan();
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.v;
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed randomly
        // sampled point around the pixel location i, j

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.centre
        } else {
            self.defocus_disk_sample()
        };
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
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                return attenuation * self.ray_colour(&scattered, depth - 1, world);
            } else {
                return Colour::new(0.0, 0.0, 0.0);
            }
        }
        let unit_direction: Vec3 = r.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk

        let p = Point3::random_in_unit_disk();
        self.centre + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}
