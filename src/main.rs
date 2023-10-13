fn main() {
    print_circle_ppm(2.5);
}

/// Red gradient.
fn print_circle_ppm(r: f64) {
    const X: u32 = 64;
    const Y: u32 = 48;

    println!("P3");
    println!("{X} {Y} 255");
    println!();

    for y in 0..Y {
        for x in 0..X {
            let dx = x.abs_diff(X / 2) as f64;
            let dy = y.abs_diff(Y / 2) as f64;
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
