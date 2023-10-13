fn main() {
    print_circle_ppm(2.5);
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
