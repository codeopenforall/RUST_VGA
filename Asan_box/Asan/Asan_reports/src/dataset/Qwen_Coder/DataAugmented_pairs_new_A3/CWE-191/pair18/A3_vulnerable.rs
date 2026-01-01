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

//////////////////////////////
// Unit Test Oracle Function
//////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // The test verifies that attempting to subtract a larger value
    // than the current value causes a panic. The corrected version should
    // panic (passing the test), while the vulnerable version would perform
    // an unsafe wrap-around subtraction (failing the test).
    #[test]
    fn underflow_test() {
        let result = panic::catch_unwind(|| {
            run();
        });
        assert!(result.is_err(), "Expected panic due to underflow, but operation completed normally");
    }
}
