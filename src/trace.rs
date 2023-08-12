use std::fmt::{Display, Write};

use once_cell::sync::Lazy;
use rand::Rng;

use crate::{
    color::write_color,
    math::{clamp, dot, Color3, Float, Point3, Range, Vec3},
};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

#[derive(Clone, Copy)]
pub struct RayHit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: Float,
    pub front_face: bool,
}

pub trait Hittable {
    fn does_hit(&self, ray: &Ray, ray_t: &Range) -> Option<RayHit>;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: Float,
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,

    center: Point3,
    viewport_width: Float,
    viewport_height: Float,
    viewport_u: Vec3,
    viewport_v: Vec3,
    viewport_top_left: Point3,
    focal_length: Float,
    pixel_top_left: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Point3::new(),
            direction: Vec3::new(),
        }
    }

    pub fn from(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn at(&self, t: Float) -> Point3 {
        self.origin + self.direction * t
    }
}

impl RayHit {
    pub fn from(point: &Point3, t: Float) -> Self {
        Self {
            point: *point,
            normal: Vec3::new(),
            t,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

impl Sphere {
    pub fn from(center: &Point3, radius: Float) -> Self {
        Self {
            center: *center,
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn does_hit(&self, ray: &Ray, ray_t: &Range) -> Option<RayHit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut t = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(t) {
            t = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(t) {
                return None;
            }
        }

        let mut hit = RayHit::from(&ray.at(t), t);
        let outward_normal = (hit.point - self.center) / self.radius;
        hit.set_face_normal(ray, &outward_normal);
        Some(hit)
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn from(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn does_hit(&self, ray: &Ray, ray_t: &Range) -> Option<RayHit> {
        let mut closest_hit = None;
        let mut closest_hit_t = ray_t.max;
        for object in &self.objects {
            if let Some(hit) = object.does_hit(ray, &Range::from(ray_t.min, closest_hit_t)) {
                if hit.t < closest_hit_t {
                    closest_hit_t = hit.t;
                    closest_hit = Some(hit);
                }
            }
        }
        closest_hit
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            image_width: 16,
            image_height: 9,
            samples_per_pixel: 1,
            center: Point3::new(),
            focal_length: 0.0,
            viewport_width: 0.0,
            viewport_height: 0.0,
            viewport_u: Vec3::new(),
            viewport_v: Vec3::new(),
            viewport_top_left: Point3::new(),
            pixel_top_left: Point3::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
        }
    }
    pub fn initialize(&mut self) {
        self.center = Point3::from(0.0, 0.0, 0.0);

        self.focal_length = 1.0;
        self.viewport_height = 2.0;
        self.viewport_width =
            self.viewport_height * (self.image_width as Float / self.image_height as Float);

        self.viewport_u = Vec3::from(self.viewport_width, 0.0, 0.0);
        self.viewport_v = Vec3::from(0.0, -self.viewport_height, 0.0);

        self.pixel_delta_u = self.viewport_u / (self.image_width as Float);
        self.pixel_delta_v = self.viewport_v / (self.image_height as Float);

        self.viewport_top_left = self.center
            - Point3::from(0.0, 0.0, self.focal_length)
            - self.viewport_u / 2.0
            - self.viewport_v / 2.0;
        self.pixel_top_left =
            self.viewport_top_left + self.pixel_delta_u / 2.0 + self.pixel_delta_v / 2.0;
    }

    pub fn render<FrameBuffer: Write + Display>(
        &mut self,
        world: &dyn Hittable,
        frame_buffer: &mut FrameBuffer,
    ) {
        self.initialize();

        writeln!(
            frame_buffer,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )
        .unwrap();
        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color3::from(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.get_color(&ray, world);
                }
                write_color(frame_buffer, &pixel_color, self.samples_per_pixel);
            }
        }
        println!("{}", frame_buffer);
        eprintln!("Done.");
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let pixel_center = self.pixel_top_left
            + self.pixel_delta_u * (x as Float)
            + self.pixel_delta_v * (y as Float);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = (pixel_sample - self.center).normalized();
        Ray::from(&ray_origin, &ray_direction)
    }

    fn get_color(&self, ray: &Ray, world: &dyn Hittable) -> Color3 {
        static WHITE: Lazy<Color3> = Lazy::new(|| Color3::from(1.0, 1.0, 1.0));
        static BLUE: Lazy<Color3> = Lazy::new(|| Color3::from(0.5, 0.7, 1.0));

        if let Some(hit) = world.does_hit(ray, &Range::from(0.0, Float::INFINITY)) {
            (hit.normal + *WHITE) * 0.5
        } else {
            let a = 0.5 * (ray.direction.y() + 1.0);
            (1.0 - a) * *WHITE + a * *BLUE
        }
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let px = rng.gen::<Float>() - 0.5;
        let py = rng.gen::<Float>() - 0.5;
        self.pixel_delta_u * px + self.pixel_delta_v * py
    }
}
