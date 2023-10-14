mod vec3;

use core::fmt;

use vec3::Vec3;

fn main() {
    Scene::example().render();
}

struct Scene {
    spheres: Vec<Sphere>,
    camera_position: Vec3,
    camera_up: Vec3,
    camera_right: Vec3,
    focal_distance: f64,
    screen_width: u32,
    screen_height: u32,
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

impl Scene {
    fn example() -> Self {
        Self {
            spheres: vec![SPHERE_1, SPHERE_2],
            camera_position: Vec3::new(0., 0., -20.),
            camera_up: Vec3::new(0., 1., 0.),
            camera_right: Vec3::new(1., 0., 0.),
            focal_distance: 10.,
            screen_width: 64,
            screen_height: 48,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    const RED: Self = Self { r: 255, g: 0, b: 0 };
    const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    const BLUE: Self = Self { r: 0, g: 0, b: 255 };

    const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };

    fn ppm(self) -> PpmColor {
        PpmColor(self)
    }
}

/// Wrapper for displaying a Color in PPM format.
struct PpmColor(Color);

impl fmt::Display for PpmColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Color { r, g, b } = self.0;
        write!(f, "{r} {g} {b}")
    }
}

#[derive(Debug, Copy, Clone)]
struct Sphere {
    color: Color,
    center: Vec3,
    radius: f64,
}

// todo: cross product of up and right
// todo: remove the assumption that z-axis+ means forward (may be hard)
const FORWARD: Vec3 = Vec3::new(0., 0., 1.);

impl Scene {
    /// Prints a PPM file to stdout.
    fn render(&self) {
        println!("P3");
        println!("{} {} 255", self.screen_width, self.screen_height);
        println!();

        // find the center pixel of the screen
        let adjust = [0.5, -0.5, 0.].into();
        let center = self.camera_position + FORWARD.scale(self.focal_distance) + adjust;

        // compute the corner of the screen
        let dx = -(self.screen_width as f64) / 2.;
        let dy = self.screen_height as f64 / 2.;
        let top_left = center + [dx, dy, 0.].into();

        // iterate over all pixels in the screen
        for y in 0..self.screen_height {
            for x in 0..self.screen_width {
                let x = x as f64;
                let y = y as f64;
                let p = top_left + [x, -y, 0.].into();

                // compute the dir'n of the ray: d
                let d = p - self.camera_position;

                if let Some(color) = self.cast(d) {
                    println!("{}", color.ppm());
                } else {
                    println!("0 0 0 ");
                };
            }
        }
    }

    fn cast(&self, direction: Vec3) -> Option<Color> {
        let ray = Ray { start: self.camera_position, direction };
        let dist = |p: Vec3| (p - ray.start).norm_squared();

        let mut closest_hit = None;
        for &s in &self.spheres {
            if let Some(p) = sphere_intersection(ray, s) {
                let is_closer = match closest_hit {
                    Some((curr, _)) => dist(p) < dist(curr),
                    None => true,
                };
                if is_closer {
                    closest_hit = Some((p, s.color));
                }
            }
        }
        closest_hit.map(|(_, color)| color)
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
fn sphere_intersection_origin(c: Vec3, d: Vec3, r: f64) -> Option<Vec3> {
    let t = {
        let a = d.norm_squared();
        let b = 2. * c.dot_product(d);
        let c = c.norm_squared() - r.powf(2.);
        solve_quadratic(a, b, c)?[0]
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
