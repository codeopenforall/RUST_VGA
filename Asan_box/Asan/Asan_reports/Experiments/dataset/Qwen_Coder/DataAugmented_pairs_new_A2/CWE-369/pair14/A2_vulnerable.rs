use std::sync::Arc;
use std::thread;

struct Calculator {}

impl Calculator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compute(&self, a: i32, b: i32) -> i32 {
        unsafe {
            let ptr: *const i32 = &b as *const i32;
            let divisor = *ptr;
            if divisor == 0 {
                return 0; // Introducing a defect: returning 0 instead of panicking
            }
            a / divisor
        }
    }
}

fn main() {
    let calc = Calculator::new();
    let arc_calc = Arc::new(calc);
    let arc_clone = Arc::clone(&arc_calc);
    let handle = thread::spawn(move || {
        arc_clone.compute(10, 0)
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}

/*
   The test oracle below verifies that the computation service handles a division-by-zero request safely.
   For the vulnerable code, calling compute(100, 0) will trigger a divide-by-zero panic, causing the test to fail.
   For the corrected version, the check prevents the panic and returns 0, so the test passes.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn oracle_test() {
        let calc = Calculator::new();
        let result = panic::catch_unwind(|| calc.compute(100, 0));
        // In the corrected version, there is no panic; the fallback value (0) is returned.
        assert!(result.is_ok(), "Computation panicked when processing division by zero.");
        let value = result.unwrap();
        assert_eq!(value, 0, "Expected fallback value 0 on division by zero.");
    }
}
