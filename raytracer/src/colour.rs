//colour.rs
use std::{
    io::Write,
    ops::{Add, Mul},
};

use crate::{interval::Interval, vec3::Vec3};

pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour { r, g, b }
    }
}

//Conversion from vec3 to Colour
impl From<Vec3> for Colour {
    fn from(v: Vec3) -> Colour {
        Colour::new(v.x, v.y, v.z)
    }
}

impl Add for Colour {
    type Output = Colour;
    fn add(self, other: Colour) -> Colour {
        Colour::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;
    fn mul(self, other: Colour) -> Colour {
        Colour::new(self * other.r, self * other.g, self * other.b)
    }
}

const intensity: Interval = Interval::new(0.0, 0.999);

pub fn write_colour(out: &mut impl Write, pixel_colour: Colour) {
    let rbyte = (256.0 * intensity.clamp(pixel_colour.r)) as usize;
    let gbyte = (256.0 * intensity.clamp(pixel_colour.g)) as usize;
    let bbyte = (256.0 * intensity.clamp(pixel_colour.b)) as usize;

    //Could return an error
    writeln!(out, "{rbyte} {gbyte} {bbyte}").unwrap(); // assume it works, panic otherwise

    //One way to deal with it below
    // let result = writeln!(out, "{rbyte} {gbyte} {bbyte}");
    // match result {
    //     Ok(r) => {
    //         //it worked
    //     }
    //     Err(e) => {
    //         panic!("{e:?}");
    //     }
    // }
}
