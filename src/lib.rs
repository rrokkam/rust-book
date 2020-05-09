use std::error;
use std::fmt;

#[derive(Debug)]
pub struct PoolCreationError {
    msg: &'static str,
}

impl error::Error for PoolCreationError {}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pool creation error: {}", self.msg)
    }
}

pub struct ThreadPool;

impl ThreadPool {
    /// Make a new ThreadPool with `size` threads.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            0 => Err(PoolCreationError {
                msg: "Can't create an empty ThreadPool.",
            }),
            _ => Ok(ThreadPool),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
