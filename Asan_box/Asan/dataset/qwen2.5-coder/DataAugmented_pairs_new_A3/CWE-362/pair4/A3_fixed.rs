use std::sync::{Arc, Mutex};

struct Data {
    value: Mutex<u32>,
}

impl Data {
    fn new(val: u32) -> Self {
        Data {
            value: Mutex::new(val),
        }
    }

    fn update(&self) {
        let mut lock = self.value.lock().unwrap();
        *lock += 1;
    }

    fn fetch(&self) -> u32 {
        *self.value.lock().unwrap()
    }
}