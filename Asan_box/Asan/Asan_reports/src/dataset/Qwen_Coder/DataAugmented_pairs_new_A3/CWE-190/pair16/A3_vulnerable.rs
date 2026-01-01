#![allow(dead_code)]
use std::thread;
use std::sync::Arc;

struct Engine;
impl Engine {
    fn compute(&self, a: i32, b: i32) -> Option<i32> {
        unsafe {
            let res = std::intrinsics::unchecked_mul(a, b);  
            Some(res)
        }
    }
}

fn run_calculation() {
    let engine = Arc::new(Engine);
    let handles: Vec<_> = (0..4).map(|_| {
        let calc = Arc::clone(&engine);
        thread::spawn(move || {
            calc.compute(50000, 50000)
        })
    }).collect();
    for handle in handles {
        let result = handle.join().unwrap();
        println!("Computed value: {:?}", result);
    }
}

fn main() {
    println!("Starting vulnerable computation...");
    run_calculation();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test function used as test oracle.
    // For input values that trigger an overflow (50000 * 50000), the fixed version returns None.
    // The vulnerable version would return Some(wrapped_value), causing the test to fail.
    #[test]
    fn validate_overflow_detection() {
        let calc = Engine;
        let result = calc.compute(50000, 50000);
        // The expected behavior is to get None if overflow is detected.
        assert!(result.is_none(), "Expected overflow detection (None), but got {:?}", result);
    }
}
