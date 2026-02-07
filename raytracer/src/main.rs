//list modules that are part of the project - not an import closer to a makefile - only in main
mod colour;
mod ray;
mod vec3;

//Lets us use names locally
use crate::colour::*;
use std::io::stdout;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut out = stdout();

    println!("P3\n{image_width} {image_height}\n255");
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_colour = Colour::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );

            write_colour(&mut out, pixel_colour);
        }
    }
    println!("\rDone                                    \n");
}
