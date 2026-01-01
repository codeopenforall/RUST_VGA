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
        let mut guard = self.data.lock().unwrap();
        let current = *guard;
        match current.checked_sub(amount) {
            Some(new_val) => *guard = new_val,
            None => panic!("Underflow detected: cannot subtract {} from {}", amount, current),
        }
    }
}

fn run() {
    let counter = SafeCounter::new(10);
    counter.subtract(20); // This should cause a panic
}

fn main() {
    // Main function for demonstration purposes
    run();
}