use std::sync::Arc;
use std::thread;

struct Engine {
    value: i32,
}

impl Engine {
    fn new(val: i32) -> Self {
        Self { value: val }
    }

    fn compute(&self, divisor: i32) -> i32 {
        unsafe {
            let result = if divisor == 0 {
                self.value
            } else {
                self.value / divisor
            };
            result
        }
    }
}

fn main() {
    let engine = Arc::new(Engine::new(100));
    let engine_clone = engine.clone();
    let handle = thread::spawn(move || {
        let bad_input = 0;
        let res = engine_clone.compute(bad_input);
        println!("Computed result: {}", res);
    });
    let _ = handle.join();
    println!("Execution completed");
}

//////////////////////////////
// Test Oracle Function Start
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test applies to both code versions.
    // For input zero:
    // - The vulnerable version will panic (division by zero), causing the test to fail.
    // - The corrected version will safely return the original value (100), causing the test to pass.
    #[test]
    fn test_division_by_zero_handling() {
        let engine = Engine::new(100);
        let result = panic::catch_unwind(|| {
            engine.compute(0)
        });
        // In a correct implementation, no panic should occur and result should equal 100.
        if let Ok(val) = result {
            assert_eq!(val, 100, "Expected safe handling to return 100 for input 0");
        } else {
            panic!("Division by zero triggered a panic!");
        }
    }
}
//////////////////////////////
// Test Oracle Function End
//////////////////////////////
