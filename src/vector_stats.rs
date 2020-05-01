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
    array[0]
}

pub fn mode(array: &[i32]) -> i32 {
    let mut map: HashMap<i32, u32> = HashMap::new();
    for elem in array.iter() {
        *map.entry(*elem).or_insert(0) += 1;
    }
    map.into_iter().max_by_key(|&(_, c)| c).unwrap().0
}
