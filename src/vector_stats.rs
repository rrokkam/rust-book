//use std::collections::HashMap;
//use std::iter::Iterator;

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
    let sum = array.iter().fold(0, |acc, elem| acc + elem);
    sum as f64 / array.len() as f64
}

pub fn median(array: &[i32]) -> i32 {
    let a: &[i32] = array.clone();
    //    a.sort();
    a[a.len() / 2]
}

pub fn mode(array: &[i32]) -> i32 {
    array[0]
}
