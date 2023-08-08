mod color;
mod math;

use crate::{
    color::write_color,
    math::{Color3, Float},
};
use std::fmt::Write;

fn main() {
    let image_width = 256u32;
    let image_height = 256u32;

    let mut image_buffer = String::new();
    // Header for PPM file
    writeln!(&mut image_buffer, "P3\n{image_width} {image_height}\n255").unwrap();
    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = Color3::from(
                i as Float / (image_width - 1) as Float,
                j as Float / (image_height - 1) as Float,
                0.0 as Float,
            );
            write_color(&mut image_buffer, &pixel_color);
        }
    }
    println!("{}", image_buffer);
    eprintln!("Done.");
}
