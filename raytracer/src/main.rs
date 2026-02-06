
fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{image_width} {image_height}\n255");
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height-j);
        for i in 0..image_width{
            let r = j as f64 / (image_width-1) as f64;
            let g = i as f64 / (image_height-1) as f64;
            let b = 0.0;

            let ir = (255.99 * r) as usize;
            let ig = (255.99 * g) as usize;
            let ib = (255.99 * b) as usize;

            println!("{ir} {ig} {ib}");
        }
    }
    println!("\rDone                                    \n");

}
