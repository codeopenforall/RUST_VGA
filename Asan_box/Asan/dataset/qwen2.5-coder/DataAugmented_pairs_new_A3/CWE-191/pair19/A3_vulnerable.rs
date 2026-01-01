use std::sync::{Arc, Mutex};

pub struct Data {
    value: Mutex<i32>,
}

impl Data {
    pub fn new(initial_value: i32) -> Self {
        Data {
            value: Mutex::new(initial_value),
        }
    }

    pub fn subtract(&self, sub: i32) {
        let mut current = self.value.lock().unwrap();
        *current = *current - sub; // Vulnerable line: unchecked subtraction
    }

    pub fn get(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}