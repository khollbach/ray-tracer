use std::fmt;

use crate::vec3::Vec3;

// todo: maybe add a way to work with color values as real numbers ?

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };

    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[must_use]
    pub fn direct_product(self, other: Color) -> Self {
        let product = self.to_vec3().direct_product(other.to_vec3());
        Self::from_vec3(product)
    }

    #[must_use]
    pub fn scale(self, multiplier: f64) -> Self {
        let scaled = self.to_vec3().scale(multiplier);
        Self::from_vec3(scaled)
    }

    fn to_vec3(self) -> Vec3 {
        let Self { r, g, b } = self;
        let x = r as f64 / 255.;
        let y = g as f64 / 255.;
        let z = b as f64 / 255.;
        Vec3::new(x, y, z)
    }

    fn from_vec3(rgb: Vec3) -> Self {
        let r = (rgb.x().clamp(0., 1.) * 255.) as u8;
        let g = (rgb.y().clamp(0., 1.) * 255.) as u8;
        let b = (rgb.z().clamp(0., 1.) * 255.) as u8;
        Self { r, g, b }
    }

    pub fn ppm(self) -> PpmColor {
        PpmColor(self)
    }
}

/// Wrapper for displaying a Color in PPM format.
pub struct PpmColor(Color);

impl fmt::Display for PpmColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Color { r, g, b } = self.0;
        write!(f, "{r} {g} {b}")
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub color: Color,
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn normal(self, point: Vec3) -> Vec3 {
        (point - self.center).normalize()
    }
}
