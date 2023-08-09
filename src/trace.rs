use crate::math::{dot, Float, Point3, Vec3};

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
    fn does_hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<RayHit>;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: Float,
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
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
    fn does_hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<RayHit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max <= root {
            root = (-half_b + sqrtd) / a;
            if root <= t_min || t_max <= root {
                return None;
            }
        }

        let mut hit = RayHit::from(&ray.at(root), root);
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

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn does_hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<RayHit> {
        let mut closest_hit = None;
        let mut closest_hit_t = t_max;
        for object in &self.objects {
            if let Some(hit) = object.does_hit(ray, t_min, closest_hit_t) {
                if hit.t < closest_hit_t {
                    closest_hit_t = hit.t;
                    closest_hit = Some(hit);
                }
            }
        }
        closest_hit
    }
}
