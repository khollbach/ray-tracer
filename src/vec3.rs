use std::{
    iter::zip,
    ops::{Add, Div, Mul, Neg, Sub, SubAssign},
};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    coords: [f64; 3],
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::new(0., 0., 0.);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { coords: [x, y, z] }
    }

    pub fn x(self) -> f64 {
        self.coords[0]
    }

    pub fn y(self) -> f64 {
        self.coords[1]
    }

    pub fn z(self) -> f64 {
        self.coords[2]
    }

    #[must_use]
    pub fn scale(self, a: f64) -> Self {
        Self {
            coords: self.coords.map(|c| c * a),
        }
    }

    pub fn dot_product(self, other: Self) -> f64 {
        zip(self.coords, other.coords).map(|(a, b)| a * b).sum()
    }

    #[must_use]
    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    pub fn norm_squared(self) -> f64 {
        self * self
    }

    pub fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn direct_product(self, other: Self) -> Self {
        let x = self.x() * other.x();
        let y = self.y() * other.y();
        let z = self.z() * other.z();
        Self::new(x, y, z)
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(coords: [f64; 3]) -> Self {
        Self { coords }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        let z = self.z() + other.z();
        Self::new(x, y, z)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

/// Dot product.
impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.dot_product(other)
    }
}

/// Scalar multiplication: `vec * scalar`.
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        self.scale(scalar)
    }
}

/// Scalar multiplication: `scalar * vec`.
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        vec.scale(self)
    }
}

/// Division by a scalar.
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        self * (1. / scalar)
    }
}
