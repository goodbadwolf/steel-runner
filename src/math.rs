use std::fmt;
use std::marker::Copy;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

pub type Float = f64;
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    e: [Float; 3],
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

    pub fn dot(&self, rhs: Self) -> Float {
        let x = self.e[0];
        let y = self.e[1];
        let z = self.e[2];
        let rx = rhs.e[0];
        let ry = rhs.e[1];
        let rz = rhs.e[2];
        x * rx + y * ry + z * rz
    }

    pub fn cross(&self, rhs: Self) -> Self {
        let ux = self.e[0];
        let uy = self.e[1];
        let uz = self.e[2];
        let vx = rhs.e[0];
        let vy = rhs.e[1];
        let vz = rhs.e[2];
        Self {
            e: [
                (uy * vz - uz * vy),
                (uz * vx - ux * vz),
                (ux * vy - uy * vx),
            ],
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }
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

pub type Point3 = Vec3;

pub type Color3 = Vec3;
