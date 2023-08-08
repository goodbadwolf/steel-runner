fn main() {
    let image_width = 256u32;
    let image_height = 256u32;

    // Header for PPM file
    println!("P3\n{image_width} {image_height}\n255");
    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0f64;

            let ir = (255.0 * r).round() as u32;
            let ig = (255.0 * g).round() as u32;
            let ib = (255.0 * b).round() as u32;
            println!("{ir} {ig} {ib}");
        }
    }
}
