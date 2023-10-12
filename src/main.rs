fn main() {
    print_circle(2.5);
}

fn print_circle(r: f64) {
    const X: i32 = 64;
    const Y: i32 = 48;
    for y in 0..Y {
        for x in 0..X {
            let dx = x - X / 2;
            let dy = y - Y / 2;
            let hit = ((dx.pow(2) + dy.pow(2)) as f64) < r.powf(2.);
            let symbol = if hit { 'x' } else { '.' };
            print!("{symbol}");
        }
        println!();
    }
}
