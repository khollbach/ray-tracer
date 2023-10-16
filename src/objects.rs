mod plane;
mod sphere;

use crate::{color::Color, scene::Ray, vec3::Vec3};

pub use plane::Plane;
pub use sphere::Sphere;

pub trait Object {
    fn hit_test(&self, ray: Ray) -> Option<Vec3>;

    fn normal(&self, surface_point: Vec3) -> Vec3;

    fn color(&self) -> Color;
}
