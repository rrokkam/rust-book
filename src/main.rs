#![warn(rust_2018_idioms)]

mod vector_stats;
use vector_stats::Stats;

mod pig_latin;
use pig_latin::pig_latin;
// use department_manager;

fn main() {
    test_vector_stats();
    test_pig_latin();
}

fn test_vector_stats() {
    let array = [20, 2, 234, 235, 5, 3, 4, 4, 6, 7, 2342];
    let stats = Stats::compute_stats(&array);
    println!("vector::stats for array {:?}", array);
    println!(
        "mean: {}, median: {}, mode: {}",
        stats.mean, stats.median, stats.mode
    );
}

fn test_pig_latin() {
    let text1 = "arstdhneio";
    println!("{} piglatinized is {}", text1, pig_latin(text1));
}
