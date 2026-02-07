//colour.rs
use std::io::Write;

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

pub fn write_colour(out: &mut impl Write, pixel_colour: Colour) {
    let rbyte = (255.99 * pixel_colour.r) as usize;
    let gbyte = (255.99 * pixel_colour.g) as usize;
    let bbyte = (255.99 * pixel_colour.b) as usize;

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
