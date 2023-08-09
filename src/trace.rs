use crate::math::{dot, Float, Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: Float,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool;
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

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
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
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max <= root {
            root = (-half_b + sqrtd) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        true
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
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
