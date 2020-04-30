#[derive(Debug)]
pub struct Stats {
    pub mean: i32,
    pub median: i32,
    pub mode: i32,
}

pub fn new(array: &[i32]) -> Stats {
    Stats {
        mean: mean(array),
        median: median(array),
        mode: mode(array),
    }
}

pub fn mean(array: &[i32]) -> i32 {
    array[0]
}

pub fn median(array: &[i32]) -> i32 {
    array[0]
}

pub fn mode(array: &[i32]) -> i32 {
    array[0]
}
