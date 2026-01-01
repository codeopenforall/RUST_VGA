use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    state: Mutex<u32>,
}

impl Engine {
    fn new() -> Engine {
        Engine {
            state: Mutex::new(0),
        }
    }

    unsafe fn execute(&self, value: u32) -> Result<u32, u32> {
        if value == 0 {
            Err(0)
        } else {
            Ok(value)
        }
    }

    fn run_process(&self, value: u32) -> bool {
        unsafe {
            match self.execute(value) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
    }
}

fn main() {
    let engine = Arc::new(Engine::new());
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let eng = engine.clone();
            thread::spawn(move || {
                let outcome = eng.run_process(i % 2);
                if outcome {
                    println!("Thread {}: processed (success)", i);
                } else {
                    println!("Thread {}: processed (failure)", i);
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    // Import the Engine from the code under test.
    // Note: Ensure that the code under test (either vulnerable or fixed) 
    // is accessible in the same module hierarchy or via appropriate module paths.
    use crate::Engine;

    #[test]
    fn test_run_process_with_zero() {
        let engine = Arc::new(Engine::new());
        // Input 0 should be considered a failure.
        let outcome = engine.run_process(0);
        // For a correct/fixed implementation, outcome must be false.
        // In the vulnerable version, outcome erroneously returns true.
        assert!(!outcome, "Expected run_process(0) to return false, but got true");
    }
}
