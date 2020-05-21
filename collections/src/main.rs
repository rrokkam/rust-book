#![warn(rust_2018_idioms)]

mod department_manager;
mod pig_latin;
mod vector_stats;

fn main() {
    test_vector_stats();
    test_pig_latin();
    test_department_manager();
}

fn test_vector_stats() {
    let array = [20, 2, 234, 235, 5, 3, 4, 4, 6, 7, 2342];
    let stats = vector_stats::Stats::compute_stats(&array);
    println!("vector::stats for array {:?}", array);
    println!(
        "mean: {}, median: {}, mode: {}",
        stats.mean, stats.median, stats.mode
    );
}

fn test_pig_latin() {
    let text1 = "arstdhneio";
    println!("{} piglatinized is {}", text1, pig_latin::pig_latin(text1));
}

fn test_department_manager() {
    let input = [
        "Add Sally to Engineering",
        "Add Amir to Sales",
        "Add Michael to Marketing",
    ];
    department_manager::manage(&input);
}
