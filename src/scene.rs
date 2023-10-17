use std::{
    any::{self, Any, TypeId},
    ops::Deref,
};

use crate::{
    color::Color,
    error::Result,
    objects::{Object, Plane, Sphere},
    sdl,
    vec3::Vec3,
};

pub struct Scene {
    objects: Vec<Box<dyn Object>>,
    camera_position: Vec3,
    camera_up: Vec3,
    camera_right: Vec3,
    focal_distance: f64,
    screen_width: u32,
    screen_height: u32,
    light_source: Vec3,
    light_color: Color,
}

// todo: cross product of up and right
// todo: remove the assumption that z-axis+ means forward (may be hard)
const FORWARD: Vec3 = Vec3::new(0., 0., 1.);

impl Scene {
    pub fn from_sdl(text: &str) -> Result<Self> {
        let tree: sdl::Node = text.parse()?;
        Ok(Self {
            camera_position: tree.get_path("camera position")?.try_into()?,
            camera_up: tree.get_path("camera up")?.try_into()?,
            camera_right: tree.get_path("camera right")?.try_into()?,

            focal_distance: tree.get_path("focal-distance")?.try_into()?,
            screen_width: tree.get_path("screen width")?.try_into()?,
            screen_height: tree.get_path("screen height")?.try_into()?,

            light_source: tree.get_path("lights light position")?.try_into()?,
            light_color: tree.get_path("lights light color")?.try_into()?,

            objects: tree.get_path("objects")?.try_into()?,
        })
    }

    /// Prints a PPM file to stdout.
    // todo: render to a buffer, and then serialize the buffer to stdout.
    // this'll let you refactor the PPM code into its own module.
    pub fn render(&self) {
        println!("P3");
        println!("{} {} 255", self.screen_width, self.screen_height);
        println!();

        // iterate over all pixels in the screen
        for y in 0..self.screen_height {
            for x in 0..self.screen_width {
                let color = self.pixel_color(x, y);
                println!("{}", color.ppm());
            }
        }
    }

    fn top_left_pixel(&self) -> Vec3 {
        // find the center pixel of the screen
        let adjust = [0.5, -0.5, 0.].into();
        let center = self.camera_position + self.focal_distance * FORWARD + adjust;

        // compute the corner of the screen
        let dx = -(self.screen_width as f64) / 2.;
        let dy = self.screen_height as f64 / 2.;
        let top_left = center + [dx, dy, 0.].into();

        top_left
    }

    fn pixel_color(&self, x: u32, y: u32) -> Color {
        let top_left = self.top_left_pixel();

        let x = x as f64;
        let y = y as f64;
        let p = top_left + [x, -y, 0.].into();

        // compute the dir'n of the ray
        let start = self.camera_position;
        let direction = p - start;
        let ray = Ray { start, direction };

        if let Some((obj, p)) = self.cast(ray, f64::MAX) {
            // cast another ray, towards the light source
            let path = self.light_source - p;
            let ray = Ray {
                // TODO: this feels like a hack.
                // How else can we avoid hitting the current sphere
                // when we cast a ray? Maybe we skip it somehow?
                start: p + path.normalize() * 0.1,
                direction: path,
            };
            let max_dist = path.norm();

            let intercepted = self.cast(ray, max_dist).is_some();
            if intercepted {
                // hidden in shadow
                Color::BLACK
            } else {
                // compute a color value
                // The "insides" of a surface should also be visible --
                // hence the .abs() here.
                let brightness = (path.normalize() * obj.normal(p)).abs();
                let color = self
                    .light_color
                    .direct_product(obj.color())
                    .scale(brightness);
                color
            }
        } else {
            Color::BLACK
        }
    }

    fn cast(&self, ray: Ray, max_dist: f64) -> Option<(&dyn Object, Vec3)> {
        let dist = |p: Vec3| (p - ray.start).norm_squared().sqrt();

        let mut closest_hit = None;
        for obj in &self.objects {
            if let Some(p) = obj.hit_test(ray) {
                if dist(p) > max_dist {
                    continue;
                }

                let is_closer = match closest_hit {
                    Some((_, curr)) => dist(p) < dist(curr),
                    None => true,
                };
                if is_closer {
                    closest_hit = Some((obj.deref(), p));
                }
            }
        }
        closest_hit
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub start: Vec3,
    pub direction: Vec3,
}
