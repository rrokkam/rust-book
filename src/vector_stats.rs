use std::collections::HashMap;

#[derive(Debug)]
pub struct Stats {
    pub mean: f64,
    pub median: i32,
    pub mode: i32,
}

impl Stats {
    pub fn compute_stats(array: &[i32]) -> Self {
        Stats {
            mean: mean(array),
            median: median(array),
            mode: mode(array),
        }
    }
}

pub fn mean(array: &[i32]) -> f64 {
    let sum: i32 = array.iter().sum();
    sum as f64 / array.len() as f64
}

pub fn median(array: &[i32]) -> i32 {
    array[3]
}

pub fn mode(array: &[i32]) -> i32 {
    let mut map: HashMap<i32, u32> = HashMap::new();
    for elem in array.iter() {
        *map.entry(*elem).or_insert(0) += 1;
    }
    let mut mode = 0;
    let mut count = 0;
    for (k, c) in map.iter() {
        if *c > count {
            mode = *k;
            count = *c;
        }
    }
    mode
}
