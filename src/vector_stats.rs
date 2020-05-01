//use std::collections::HashMap;

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
    let mut total = 0;
    let mut len = 0;
    for elem in array.iter() {
        len += 1;
        total += elem;
    }
    total as f64 / len as f64
}

pub fn median(array: &[i32]) -> i32 {
    let a: &[i32] = array.clone();
//    a.sort();
    a[a.len() / 2]
}

pub fn mode(array: &[i32]) -> i32 {
    array[0]
}
