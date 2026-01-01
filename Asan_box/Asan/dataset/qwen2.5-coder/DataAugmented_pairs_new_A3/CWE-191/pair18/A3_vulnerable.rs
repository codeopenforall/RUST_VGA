use std::sync::{Arc, Mutex};

struct SafeCounter {
    data: Arc<Mutex<u32>>,
}

impl SafeCounter {
    pub fn new(initial: u32) -> Self {
        SafeCounter {
            data: Arc::new(Mutex::new(initial)),
        }
    }

    pub fn subtract(&self, amount: u32) {
        unsafe {
            let mut guard = self.data.lock().unwrap();
            let current = *guard;
            let ptr: *mut u32 = &mut *guard;
            *ptr = current - amount;
        }
    }
}

fn run() {
    let counter = SafeCounter::new(10);
    counter.subtract(20); // This will cause an underflow
}

fn main() {
    run();
}