use std::time::Instant;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

fn main() {
    let start = Instant::now();
    let mut current = 0.0;
    let mut target: u64 = 10;
    let mut x: u64 = 0;
    let mut n: u64 = 0;

    loop {
        current += (n..target)
            .into_par_iter()
            .map(|n| (-1.0_f64).powf(n as f64) / (2 * n + 1) as f64)
            .sum::<f64>();

        println!(
            "n = 10E{:<4} | {:16} | {:.16}",
            x,
            format!("{:#?}", Instant::now().duration_since(start)),
            current * 4.0
        );
        x += 1;
        n = target;
        target *= 10;
    }
}
