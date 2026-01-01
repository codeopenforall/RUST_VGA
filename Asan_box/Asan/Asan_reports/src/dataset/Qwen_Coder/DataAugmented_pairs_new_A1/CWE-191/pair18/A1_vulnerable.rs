use std::sync::{Arc, Mutex};
use std::thread;

struct Calculator {
    data: Mutex<u32>,
}

impl Calculator {
    fn new(initial: u32) -> Self {
        Calculator {
            data: Mutex::new(initial),
        }
    }

    fn decrement(&self, amount: u32) {
        let mut guard = self.data.lock().unwrap();
        let current = *guard;
        // Introduce vulnerability by removing the check for underflow
        *guard = current - amount;
    }

    fn fetch(&self) -> u32 {
        *self.data.lock().unwrap()
    }
}

fn run() -> u32 {
    let calc = Arc::new(Calculator::new(5));
    let calc_clone = Arc::clone(&calc);
    let handler = thread::spawn(move || {
        calc_clone.decrement(10);
    });
    handler.join().unwrap();
    calc.fetch()
}

fn main() {
    let _ = run();
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
