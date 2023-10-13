use std::{
    iter::zip,
    ops::{Add, Sub},
};

fn main() {
    SCENE.render();
}

/// Red gradient.
fn print_circle_ppm(r: f64) {
    let x_max = 64;
    let y_max = 48;

    println!("P3");
    println!("{x_max} {y_max} 255");
    println!();

    for y in 0..y_max {
        for x in 0..x_max {
            let x_max = x_max as f64;
            let y_max = y_max as f64;
            let x = x as f64;
            let y = y as f64;

            let dx = x - x_max / 2.;
            let dy = y - y_max / 2.;
            let hit = dx.powf(2.) + dy.powf(2.) < r.powf(2.);

            if hit {
                let diam = r * 2.;
                let x_percent = (dx + r) / diam;
                let y_percent = (dy + r) / diam;
                let percent = (x_percent + y_percent) / 2.;
                let shade = (percent * 255.).trunc().clamp(0., 255.) as u8;
                println!("{shade} 0 0");
            } else {
                println!("0 0 0");
            }
        }
        println!();
    }
}

struct Scene {
    sphere_center: Vec3,
    sphere_radius: f64,
    camera_position: Vec3,
    camera_up: Vec3,
    camera_right: Vec3,
    focal_distance: f64,
    screen_width: u32,
    screen_height: u32,
}

const SCENE: Scene = Scene {
    sphere_center: Vec3::new(0., 0., 0.),
    sphere_radius: 5.,
    camera_position: Vec3::new(0., 0., -20.),
    camera_up: Vec3::new(0., 1., 0.),
    camera_right: Vec3::new(1., 0., 0.),
    focal_distance: 10.,
    screen_width: 64,
    screen_height: 48,
};

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

                // compute the intersection; if yes, black; else white.
                let c = self.camera_position;
                let r = self.sphere_radius;
                let hit = sphere_intersection(c, d, r).is_some();

                if hit {
                    println!("255 255 255");
                } else {
                    println!("0 0 0 ");
                };
            }
        }
    }
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
fn sphere_intersection(c: Vec3, d: Vec3, r: f64) -> Option<Vec3> {
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

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    coords: [f64; 3],
}

impl Vec3 {
    const ZERO: Self = Self::new(0., 0., 0.);

    const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { coords: [x, y, z] }
    }

    fn x(self) -> f64 {
        self.coords[0]
    }

    fn y(self) -> f64 {
        self.coords[1]
    }

    fn z(self) -> f64 {
        self.coords[2]
    }

    // todo: could we implement `*` for f64 and Vec3 ?
    fn scale(self, a: f64) -> Self {
        Self {
            coords: self.coords.map(|c| c * a),
        }
    }

    fn dot_product(self, other: Self) -> f64 {
        zip(self.coords, other.coords).map(|(a, b)| a * b).sum()
    }

    fn norm_squared(self) -> f64 {
        self.dot_product(self)
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(coords: [f64; 3]) -> Self {
        Self { coords }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        let z = self.z() + other.z();
        Self::new(x, y, z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other.scale(-1.)
    }
}
