use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    value: i32,
}

impl Engine {
    fn new(val: i32) -> Self {
        Self { value: val }
    }

    fn exec(&self, divisor: i32) -> Result<i32, &'static str> {
        if divisor == 0 {
            return Err("Division by zero");
        }
        unsafe {
            let result = self.value / divisor;
            Ok(result)
        }
    }
}

fn main() {
    let engine = Arc::new(Mutex::new(Engine::new(100)));
    let engine_clone = Arc::clone(&engine);
    let handle = thread::spawn(move || {
        let engine = engine_clone.lock().unwrap();
        let _ = engine.exec(0);
    });
    let _ = handle.join();
    println!("Operation completed.");
}

#[cfg(test)]
mod tests {
    use super::Engine;
    use std::panic;

    #[test]
    fn division_by_zero_test() {
        // Create an instance with a non-zero numerator.
        let engine = Engine::new(50);
        // Execute the division with zero as divisor.
        // In the corrected implementation, exec should return Err.
        // In the insecure implementation, a division by zero panic occurs.
        let result = panic::catch_unwind(|| engine.exec(0));
        match result {
            // For the corrected code: no panic occurs, and exec returns an Err.
            Ok(res) => {
                assert!(res.is_err(), "Expected an error result for division by zero");
            },
            // For the vulnerable code: a panic is caught.
            Err(_) => {
                panic!("Unexpected panic occurred");
            }
        }
    }
}
