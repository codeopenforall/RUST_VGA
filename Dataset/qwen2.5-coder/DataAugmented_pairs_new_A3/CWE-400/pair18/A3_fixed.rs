use std::sync::{Arc, Mutex};

const MAX_CAPACITY: usize = 1000;

struct Pool {
    data: Mutex<Vec<i32>>,
}

impl Pool {
    pub fn new() -> Self {
        Pool {
            data: Mutex::new(Vec::new()),
        }
    }

    pub fn add(&self, value: i32) {
        let mut vec = self.data.lock().unwrap();
        if vec.len() < MAX_CAPACITY {
            vec.push(value);
        }
    }

    pub fn count(&self) -> usize {
        let vec = self.data.lock().unwrap();
        vec.len()
    }
}

pub fn simulate() -> bool {
    let pool = Arc::new(Pool::new());
    for i in 0..1500 {
        pool.add(i);
    }
    pool.count() <= MAX_CAPACITY
}