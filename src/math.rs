use std::fmt;
use std::marker::Copy;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[cfg(feature = "use_f64")]
pub type Float = f64;
#[cfg(feature = "use_f64")]
pub const PI: Float = std::f64::consts::PI;

#[cfg(not(feature = "use_f64"))]
pub type Float = f32;
#[cfg(not(feature = "use_f64"))]
pub const PI: Float = std::f32::consts::PI;

pub const INFINITY: Float = Float::INFINITY;
pub const NEG_INFINITY: Float = Float::NEG_INFINITY;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    e: [Float; 3],
}

pub struct Range {
    pub min: Float,
    pub max: Float,
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            e: [Float::default(); 3],
        }
    }

    pub fn from(e0: Float, e1: Float, e2: Float) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> Float {
        self.e[0]
    }

    pub fn y(&self) -> Float {
        self.e[1]
    }

    pub fn z(&self) -> Float {
        self.e[2]
    }

    pub fn length_squared(&self) -> Float {
        let x = self.e[0];
        let y = self.e[1];
        let z = self.e[2];
        x * x + y * y + z * z
    }

    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }
}

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> Float {
    lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
}

pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    let ux = lhs.x();
    let uy = lhs.y();
    let uz = lhs.z();
    let vx = rhs.x();
    let vy = rhs.y();
    let vz = rhs.z();
    Vec3::from(uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx)
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl Mul<Float> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Float) -> Self::Output {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl Mul<Vec3> for Float {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl Div<Float> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Float) -> Self::Output {
        Self {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];
        self.e[2] /= rhs.e[2];
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.e[0], self.e[1], self.e[2])
    }
}

impl Range {
    pub fn new() -> Self {
        Self {
            min: INFINITY,
            max: NEG_INFINITY,
        }
    }

    pub fn from(min: Float, max: Float) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, value: Float) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, value: Float) -> bool {
        self.min < value && value < self.max
    }
}

pub fn clamp<T: PartialOrd + Copy>(v: T, min: T, max: T) -> T {
    if v < min as T {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

pub type Point3 = Vec3;

pub type Color3 = Vec3;
