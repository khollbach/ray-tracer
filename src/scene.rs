use crate::{
    sphere::{Color, Sphere},
    vec3::Vec3,
};

pub struct Scene {
    spheres: Vec<Sphere>,
    camera_position: Vec3,
    camera_up: Vec3,
    camera_right: Vec3,
    focal_distance: f64,
    screen_width: u32,
    screen_height: u32,
    light_source: Vec3,
    light_color: Color,
}

const SPHERE_1: Sphere = Sphere {
    color: Color::GREEN,
    center: Vec3::new(-5., 2.5, 0.),
    radius: 10.,
};

const SPHERE_2: Sphere = Sphere {
    color: Color::BLUE,
    center: Vec3::new(5., -2.5, 0.),
    radius: 10.,
};

// todo: cross product of up and right
// todo: remove the assumption that z-axis+ means forward (may be hard)
const FORWARD: Vec3 = Vec3::new(0., 0., 1.);

impl Scene {
    pub fn example() -> Self {
        Self {
            spheres: vec![SPHERE_1, SPHERE_2],
            camera_position: Vec3::new(0., 0., -20.),
            camera_up: Vec3::new(0., 1., 0.),
            camera_right: Vec3::new(1., 0., 0.),
            focal_distance: 10.,
            screen_width: 64,
            screen_height: 48,
            light_source: Vec3::new(-10., 10., -20.),
            light_color: Color::new(255, 200, 255),
        }
    }

    /// Prints a PPM file to stdout.
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
        let center = self.camera_position + FORWARD.scale(self.focal_distance) + adjust;

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

        if let Some((sphere, p)) = self.cast(ray, f64::MAX) {
            // cast another ray, towards the light source
            let path = self.light_source - p;
            let ray = Ray {
                // TODO: this feels like a hack.
                // How else can we avoid hitting the current sphere
                // when we cast a ray? Maybe we skip it somehow?
                start: p + path.normalize().scale(0.1),
                direction: path,
            };
            let max_dist = path.norm();

            let intercepted = self.cast(ray, max_dist).is_some();
            if intercepted {
                // hidden in shadow
                Color::BLACK
            } else {
                // compute a color value
                let brightness = path.normalize() * sphere.normal(p);
                let color = self
                    .light_color
                    .direct_product(sphere.color)
                    .scale(brightness);
                color
            }
        } else {
            Color::BLACK
        }
    }

    fn cast(&self, ray: Ray, max_dist: f64) -> Option<(Sphere, Vec3)> {
        let dist = |p: Vec3| (p - ray.start).norm_squared().sqrt();

        let mut closest_hit = None;
        for &s in &self.spheres {
            if let Some(p) = sphere_intersection(ray, s) {
                if dist(p) > max_dist {
                    continue;
                }

                let is_closer = match closest_hit {
                    Some((_, curr)) => dist(p) < dist(curr),
                    None => true,
                };
                if is_closer {
                    closest_hit = Some((s, p));
                }
            }
        }
        closest_hit
    }
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    start: Vec3,
    direction: Vec3,
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
    let t = {
        let a = d.norm_squared();
        let b = 2. * c.dot_product(d);
        let c = c.norm_squared() - r.powf(2.);
        let solutions = solve_quadratic(a, b, c)?;
        solutions.into_iter().filter(|&t| t > 0.).next()?
    };
    Some(c + d.scale(t))
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
