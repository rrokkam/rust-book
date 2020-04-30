mod vector_stats;
// use pig_latin;
// use department_manager;

fn main() {
    let array = [2, 3, 4];
    let stats = vector_stats::new(&array);
    println!("vector::stats for array {:?}", array);
    println!(
        "mean: {}, median: {}, mode: {}",
        stats.mean, stats.median, stats.mode
    );
}
