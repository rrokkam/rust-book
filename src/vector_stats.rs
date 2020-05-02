#![warn(rust_2018_idioms)]

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
            mean: Stats::mean(array),
            median: Stats::median(array),
            mode: Stats::mode(array),
        }
    }

    fn mean(array: &[i32]) -> f64 {
        let sum: i32 = array.iter().sum();
        sum as f64 / array.len() as f64
    }

    fn median(array: &[i32]) -> i32 {
        let mut vec: Vec<i32> = array.to_owned();
        vec.sort();
        vec[vec.len() / 2]
    }

    fn mode(array: &[i32]) -> i32 {
        let mut map: HashMap<i32, u32> = HashMap::new();
        for elem in array.iter() {
            *map.entry(*elem).or_insert(0) += 1;
        }
        map.into_iter()
            .max_by_key(|&(_, c)| c)
            .map(|(elem, _)| elem)
            .unwrap()
    }
}
