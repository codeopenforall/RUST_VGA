use std::sync::{Arc, Mutex};

pub struct Counter {
    count: Mutex<i32>,
}

impl Counter {
    pub fn new(initial_count: i32) -> Self {
        Counter {
            count: Mutex::new(initial_count),
        }
    }

    pub fn subtract(&self, amount: i32) -> Result<i32, &'static str> {
        let mut current = self.count.lock().unwrap();
        let res = current.wrapping_sub(amount);
        *current = res;
        Ok(res)
    }
}