use std::sync::Arc;
use std::thread;

struct Engine {
    base: u32,
    multiplier: u32,
}

impl Engine {
    fn new(b: u32, m: u32) -> Self {
        Engine { base: b, multiplier: m }
    }

    fn calculate(&self, addition: u32) -> u32 {
        unsafe {
            let ptr_base = &self.base as *const u32;
            let ptr_multiplier = &self.multiplier as *const u32;
            (*ptr_base).wrapping_add(*ptr_multiplier).wrapping_mul(addition) // Defect introduced here
        }
    }
}

fn main() {
    let engine = Arc::new(Engine::new(10, 20));
    let eng_clone = Arc::clone(&engine);
    let handle = thread::spawn(move || {
        eng_clone.calculate(5)
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle Function
// This test demonstrates that for the input parameters (base=10, multiplier=20, addition=5),
// the intended correct result is 10 * (20 + 5) = 250. The test framework will assert that the computed result
// is exactly 250. In the vulnerable version the result is 205, causing the test to fail.
// This oracle is independent from the main code in both code samples.
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_calculation() {
        let engine = Arc::new(Engine::new(10, 20));
        let eng_clone = Arc::clone(&engine);
        let handle = thread::spawn(move || {
            eng_clone.calculate(5)
        });
        let result = handle.join().unwrap();
        // Expected correct result: 10 * (20 + 5) = 250.
        assert_eq!(result, 250, "Calculation did not produce the correct result");
    }
}
