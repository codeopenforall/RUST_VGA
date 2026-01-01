use std::sync::{Arc, Mutex};

pub struct Data {
    values: Vec<u64>,
}

impl Data {
    pub fn new(size: usize) -> Self {
        let mut data = Data { values: Vec::with_capacity(size) };
        for i in 0..(size - 1) { // Intentional off-by-one error
            data.values.push(i as u64);
        }
        unsafe {
            data.values.set_len(size); // Intentional off-by-one error
        }
        data
    }

    pub fn sum(&self) -> u64 {
        self.values.iter().sum()
    }
}