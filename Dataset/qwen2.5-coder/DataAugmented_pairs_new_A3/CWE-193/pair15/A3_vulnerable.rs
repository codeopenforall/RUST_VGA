use std::sync::{Arc, Mutex};

pub struct DataBlock {
    data: Vec<u32>,
}

impl DataBlock {
    pub fn new(size: usize) -> Self {
        DataBlock {
            data: vec![0; size],
        }
    }

    pub fn fill(&mut self) {
        for i in 0..=self.data.len() {
            self.data[i] = i as u32;
        }
    }
}