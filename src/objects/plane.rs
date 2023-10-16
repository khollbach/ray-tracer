use crate::{color::Color, scene::Ray, vec3::Vec3};

use super::Object;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    anchor: Vec3,
    normal: Vec3,
    color: Color,
}

impl Object for Plane {
    fn hit_test(&self, ray: Ray) -> Option<Vec3> {
        plane_intersection(ray, *self)
    }

    fn normal(&self, _point: Vec3) -> Vec3 {
        self.normal
    }

    fn color(&self) -> Color {
        self.color
    }
}

fn plane_intersection(mut ray: Ray, plane: Plane) -> Option<Vec3> {
    // Shift the universe so the plane passes thru the origin.
    ray.start -= plane.anchor;

    let hit = plane_intersection_origin(ray, plane.normal)?;

    // Shift back.
    Some(hit + plane.anchor)
}

fn plane_intersection_origin(ray: Ray, normal: Vec3) -> Option<Vec3> {
    let rhs = normal * ray.direction;
    // todo: these f64 equality checks are surely not a good idea...
    // Understand the problem better and think of what to do about it.
    if rhs == 0. {
        if (ray.start.x(), ray.start.y(), ray.start.z()) == (0., 0., 0.) {
            return Some(Vec3::ZERO);
        } else {
            return None;
        }
    }
    let t = normal * ray.start / rhs;
    if t >= 0. {
        Some(ray.start + t * ray.direction)
    } else {
        None
    }
}
