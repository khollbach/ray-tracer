use std::fmt;

use crate::vec3::Vec3;

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
