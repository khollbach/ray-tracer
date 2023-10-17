use crate::{color::Color, scene::Ray, vec3::Vec3};

use super::Object;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub anchor: Vec3,
    pub normal: Vec3,
    pub color: Color,
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
    let t = -(normal * ray.start / rhs);
    if t >= 0. {
        Some(ray.start + t * ray.direction)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::Color, scene::Ray, vec3::Vec3};

    #[test]
    fn hit_test() {
        let obj = Plane {
            anchor: Vec3::ZERO,
            normal: Vec3::Z_AXIS,
            color: Color::WHITE,
        };
        let ray = Ray {
            start: -10. * Vec3::Z_AXIS,
            direction: Vec3::Z_AXIS,
        };
        let hit = obj.hit_test(ray);
        assert!(hit.is_some());
        dbg!(hit);
    }
}
