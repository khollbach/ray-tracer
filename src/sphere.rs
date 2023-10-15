use crate::{color::Color, vec3::Vec3};

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
