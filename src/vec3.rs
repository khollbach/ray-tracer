use std::{
    iter::zip,
    ops::{Add, Sub, Mul},
};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    coords: [f64; 3],
}

impl Vec3 {
    const ZERO: Self = Self::new(0., 0., 0.);

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

    // todo: could we implement `*` for f64 and Vec3 ?
    #[must_use]
    pub fn scale(self, a: f64) -> Self {
        Self {
            coords: self.coords.map(|c| c * a),
        }
    }

    // todo: could we implement `*` for Vec3 and Vec3?
    pub fn dot_product(self, other: Self) -> f64 {
        zip(self.coords, other.coords).map(|(a, b)| a * b).sum()
    }

    #[must_use]
    pub fn normalize(self) -> Self {
        self.scale(1. / self.norm())
    }

    pub fn norm_squared(self) -> f64 {
        self.dot_product(self)
    }

    pub fn norm(self) -> f64 {
        self.dot_product(self).sqrt()
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

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other.scale(-1.)
    }
}

impl Mul for Vec3 {
    type Output = f64;

    /// Dot product.
    fn mul(self, other: Self) -> f64 {
        self.dot_product(other)
    }
}
