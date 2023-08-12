use crate::math::{clamp, Color3, Float};
use std::fmt::Write;

pub fn write_color(f: &mut dyn Write, color: &Color3, samples_per_pixel: u32) {
    const INTENSITY_MIN: Float = 0.0 as Float;
    const INTENSITY_MAX: Float = 1.0 as Float;

    let scale = 1.0 / samples_per_pixel as Float;
    let r = clamp(color.x() * scale, INTENSITY_MIN, INTENSITY_MAX);
    let g = clamp(color.y() * scale, INTENSITY_MIN, INTENSITY_MAX);
    let b = clamp(color.z() * scale, INTENSITY_MIN, INTENSITY_MAX);

    let ir = (255.0 * r).round() as u32;
    let ig = (255.0 * g).round() as u32;
    let ib = (255.0 * b).round() as u32;
    writeln!(f, "{} {} {}", ir, ig, ib).unwrap();
}
