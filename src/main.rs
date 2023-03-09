use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut current = 0.0;
    let mut target: u32 = 10;
    let mut x: u32 = 0;
    let mut n: u32 = 0;

    loop {
        if n == target {
            println!(
                "n = 10E{:<4} | {:16} | {:.16}",
                x,
                format!("{:#?}", Instant::now().duration_since(start)),
                current * 4.0
            );
            x += 1;
            target *= 10;
        }

        current += (-1.0_f64).powf(n as f64) / (2 * n + 1) as f64;
        n += 1;
    }
}
