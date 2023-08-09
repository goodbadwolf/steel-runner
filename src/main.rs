mod color;
mod math;
mod trace;

use once_cell::sync::Lazy;

use crate::math::*;
use crate::trace::{HitRecord, Hittable, HittableList, Ray, Sphere};
use crate::{
    color::write_color,
    math::{Color3, Float},
};
use std::fmt::Write;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color3 {
    static WHITE: Lazy<Color3> = Lazy::new(|| Color3::from(1.0, 1.0, 1.0));
    static BLUE: Lazy<Color3> = Lazy::new(|| Color3::from(0.5, 0.7, 1.0));

    let mut hit_record = HitRecord::new();
    if world.hit(ray, 0.0, INFINITY, &mut hit_record) {
        0.5 * (hit_record.normal + *WHITE)
    } else {
        let unit_direction = ray.direction.normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * *WHITE + t * *BLUE
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;

    let image_width = 1920u32;
    let image_height = (image_width as Float / aspect_ratio) as u32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as Float / image_height as Float);
    let focal_length = 1.0;
    let camera_center = Point3::from(0.0, 0.0, 0.0);

    let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width as Float);
    let pixel_delta_v = viewport_v / (image_height as Float);

    let viewport_upper_left =
        camera_center - Vec3::from(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut image_buffer = String::new();

    let mut world = HittableList::new();
    // Sphere at center
    world.add(Box::new(Sphere::from(&Point3::from(0.0, 0.0, -1.0), 0.5)));
    // Floor sphere
    world.add(Box::new(Sphere::from(
        &Point3::from(0.0, -100.5, -1.0),
        100.0,
    )));

    writeln!(&mut image_buffer, "P3\n{image_width} {image_height}\n255").unwrap();
    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + i as Float * pixel_delta_u + j as Float * pixel_delta_v;
            let ray_direction = (pixel_center - camera_center).normalized();
            let ray = Ray::from(&camera_center, &ray_direction);
            let pixel_color = ray_color(&ray, &world);
            write_color(&mut image_buffer, &pixel_color);
        }
    }
    println!("{}", image_buffer);
    eprintln!("Done.");
}
