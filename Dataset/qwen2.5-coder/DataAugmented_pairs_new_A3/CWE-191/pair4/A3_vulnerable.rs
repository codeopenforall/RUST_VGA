use std::sync::{Arc, Mutex};

pub struct Counter {
    value: u32,
}

impl Counter {
    pub fn new(initial_value: u32) -> Self {
        Counter {
            value: initial_value,
        }
    }

    pub fn subtract(&mut self, amount: u32) {
        self.value = self.value.wrapping_sub(amount);
    }

    pub fn get(&self) -> u32 {
        self.value
    }
}

fn run(counter: Arc<Mutex<Counter>>, amount: u32, times: usize) {
    for _ in 0..times {
        let mut c = counter.lock().unwrap();
        c.subtract(amount);
    }
}