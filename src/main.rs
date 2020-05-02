#![warn(rust_2018_idioms)]

mod vector_stats;
use vector_stats::Stats;
// use pig_latin;
// use department_manager;

fn main() {
    let array = [20, 2, 234, 235, 5, 3, 4, 4, 6, 7, 2342];
    let stats = Stats::compute_stats(&array);
    println!("vector::stats for array {:?}", array);
    println!(
        "mean: {}, median: {}, mode: {}",
        stats.mean, stats.median, stats.mode
    );
}
