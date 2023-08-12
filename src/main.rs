mod color;
mod math;
mod trace;

use crate::math::{Float, Point3};
use crate::trace::{Camera, HittableList, Sphere};

fn main() {
    let world = HittableList::from(vec![
        // Sphere at center
        Box::new(Sphere::from(&Point3::from(0.0, 0.0, -1.0), 0.5)),
        // Floor sphere
        Box::new(Sphere::from(&Point3::from(0.0, -100.5, -1.0), 100.0)),
    ]);

    const ASPECT_RATIO: Float = 16.0 / 9.0;
    let mut camera = Camera::new();
    camera.image_width = 800;
    camera.image_height = (camera.image_width as Float / ASPECT_RATIO) as u32;
    camera.samples_per_pixel = 100;

    let mut frame_buffer = String::new();
    camera.render(&world, &mut frame_buffer);
}
