use crate::{color::Color, scene::Ray, vec3::Vec3};

use super::Object;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub color: Color,
    pub center: Vec3,
    pub radius: f64,
}

impl Object for Sphere {
    fn hit_test(&self, ray: Ray) -> Option<Vec3> {
        sphere_intersection(ray, *self)
    }

    fn normal(&self, point: Vec3) -> Vec3 {
        (point - self.center).normalize()
    }

    fn color(&self) -> Color {
        self.color
    }
}

fn sphere_intersection(ray: Ray, sphere: Sphere) -> Option<Vec3> {
    // We shift the universe so that our sphere is at the center.
    // Then we use the existing function to hit test.
    // And then we shift everything back to normal.

    let c = ray.start - sphere.center;
    let d = ray.direction;
    let r = sphere.radius;
    let p = sphere_intersection_origin(c, d, r)?;
    Some(p + sphere.center)
}

/// Return a solution to the equations:
/// v = c + t d
/// v v = r^2
///
/// c is the camera's location.
/// d is the direction of a ray from the camera.
/// r is the radius of a sphere centered at 0.
///
/// If two solutions exist, return the one closer to the camera.
///
/// Don't return solutions behind the camera.
fn sphere_intersection_origin(c: Vec3, d: Vec3, r: f64) -> Option<Vec3> {
    let t: f64 = {
        let a = d.norm_squared();
        let b = 2. * c.dot_product(d);
        let c = c.norm_squared() - r.powf(2.);
        let solutions = solve_quadratic(a, b, c)?;
        solutions.into_iter().filter(|&t| t > 0.).next()?
    };
    Some(c + t * d)
}

/// Return 0 or 2 solutions to:
/// a x^2 + b x + c = 0
///
/// The solutions may be the same. They are sorted non-decreasing.
fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<[f64; 2]> {
    // formula: (-b +- sqrt(b^2 - 4ac)) / 2a

    let d = b.powf(2.) - 4. * a * c;
    if d < 0. {
        return None;
    }

    let soln1 = (-b - d.sqrt()) / (2. * a);
    let soln2 = (-b + d.sqrt()) / (2. * a);
    Some([soln1, soln2])
}
