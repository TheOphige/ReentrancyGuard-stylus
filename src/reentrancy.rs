use stylus_sdk::prelude::*;

/// ReentrancyGuard struct to track reentrancy status
#[derive(Default)]
pub struct ReentrancyGuard {
    status: u8,
}

impl ReentrancyGuard {
    /// Initializes the guard with default status
    pub fn init(&mut self) {
        self.status = 1; // NOT_ENTERED
    }

    /// Executes a closure with reentrancy protection
    pub fn non_reentrant<F, T>(&mut self, f: F) -> Result<T, ReentrancyError>
    where
        F: FnOnce() -> Result<T, ReentrancyError>,
    {
        if self.status == 2 {
            return Err(ReentrancyError::ReentrantCall);
        }

        self.status = 2; // ENTERED
        let result = f();
        self.status = 1; // NOT_ENTERED
        result
    }
}

/// ReentrancyError enum to represent errors
#[derive(Debug)]
pub enum ReentrancyError {
    ReentrantCall,
}
