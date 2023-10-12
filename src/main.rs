fn main() {
    print_red_circle_ppm(2.5);
}

// TODO: gradient ?

fn print_red_circle_ppm(r: f64) {
    const X: i32 = 64;
    const Y: i32 = 48;

    println!("P3");
    println!("{X} {Y} 255");
    println!();

    for y in 0..Y {
        for x in 0..X {
            let dx = x - X / 2;
            let dy = y - Y / 2;
            let hit = ((dx.pow(2) + dy.pow(2)) as f64) < r.powf(2.);
            let color = if hit { "255 0 0" } else { "0 0 0" };
            println!("{color}");
        }
        println!();
    }
}
