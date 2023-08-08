use crate::math::Color3;
use std::fmt::Write;

pub fn write_color(f: &mut dyn Write, color: &Color3) {
    let ir = (255.0 * color[0]).round() as u32;
    let ig = (255.0 * color[1]).round() as u32;
    let ib = (255.0 * color[2]).round() as u32;
    writeln!(f, "{} {} {}", ir, ig, ib).unwrap();
}
