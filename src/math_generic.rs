use core::f32;
use std::convert::From;
use std::fmt;
use std::marker::Copy;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3<T> {
    e: [T; 3],
}

impl<T: Default + Copy> Vec3<T>
where
    T: From<f32> + Into<f32>,
    T: From<f64> + Into<f64>,
{
    pub fn new() -> Self {
        Self {
            e: [T::default(); 3],
        }
    }

    pub fn from(e0: T, e1: T, e2: T) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> T {
        self.e[0]
    }

    pub fn y(&self) -> T {
        self.e[1]
    }

    pub fn z(&self) -> T {
        self.e[2]
    }

    pub fn length_squared(&self) -> T {
        let x: f64 = self.e[0].into();
        let y: f64 = self.e[1].into();
        let z: f64 = self.e[2].into();
        (x * x + y * y + z * z).into()
    }

    pub fn length(&self) -> T {
        let len_squared: f64 = self.length_squared().into();
        len_squared.sqrt().into()
    }

    pub fn dot(&self, rhs: Self) -> T {
        let x: f64 = self.e[0].into();
        let y: f64 = self.e[1].into();
        let z: f64 = self.e[2].into();
        let rx: f64 = rhs.e[0].into();
        let ry: f64 = rhs.e[1].into();
        let rz: f64 = rhs.e[2].into();
        (x * rx + y * ry + z * rz).into()
    }

    pub fn cross(&self, rhs: Self) -> Self {
        let ux: f64 = self.e[0].into();
        let uy: f64 = self.e[1].into();
        let uz: f64 = self.e[2].into();
        let vx: f64 = rhs.e[0].into();
        let vy: f64 = rhs.e[1].into();
        let vz: f64 = rhs.e[2].into();
        Self {
            e: [
                (uy * vz - uz * vy).into(),
                (uz * vx - ux * vz).into(),
                (ux * vy - uy * vx).into(),
            ],
        }
    }

    pub fn unit_vector(&self) -> Self {
        let len: f64 = self.length().into();
        let x: f64 = self.e[0].into();
        let y: f64 = self.e[1].into();
        let z: f64 = self.e[2].into();
        let x = x / len;
        let y = y / len;
        let z = z / len;
        let tx: T = x.into();
        let ty: T = y.into();
        let tz: T = z.into();
        Self { e: [tx, ty, tz] }
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
    T: Copy,
{
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

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
    T: Copy,
{
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

impl<T> Mul for Vec3<T>
where
    T: Mul<Output = T>,
    T: Copy,
{
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

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T>,
    T: Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T>,
    T: Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T>,
    T: Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: AddAssign<T>,
    T: Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl<T> MulAssign for Vec3<T>
where
    T: MulAssign<T>,
    T: Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl<T> DivAssign for Vec3<T>
where
    T: DivAssign<T>,
    T: Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];
        self.e[2] /= rhs.e[2];
    }
}

impl<T> fmt::Display for Vec3<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.e[0], self.e[1], self.e[2])
    }
}

pub type Vec3f = Vec3<f32>;
pub type Vec3d = Vec3<f64>;

pub type Point3<T> = Vec3<T>;
pub type Point3f = Point3<f32>;
pub type Point3d = Point3<f64>;

pub type Color3<T> = Vec3<T>;
pub type Color3f = Color3<f32>;
pub type Color3d = Color3<f64>;
