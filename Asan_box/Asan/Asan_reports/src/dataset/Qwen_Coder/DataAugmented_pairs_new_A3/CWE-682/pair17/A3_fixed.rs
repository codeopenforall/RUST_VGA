#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    factor: i32,
}

impl Engine {
    fn new(f: i32) -> Self {
        Engine { factor: f }
    }

    unsafe fn compute(&self, input: i32) -> i32 {
        let diff = input - self.factor;
        (diff * 100) / input
    }
}

fn run(input: i32, factor: i32) -> i32 {
    let eng = Arc::new(Mutex::new(Engine::new(factor)));
    let eng_clone = Arc::clone(&eng);
    let handler = thread::spawn(move || {
        let engine = eng_clone.lock().unwrap();
        let ptr: *const i32 = &engine.factor as *const i32;
        let val = unsafe { *ptr };
        unsafe { engine.compute(input) }
    });
    handler.join().unwrap()
}

fn main() {
    let result = run(120, 100);
    println!("Final result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Test contract: For an input of 120 with factor 100,
    // the intended percentage gain is computed as ((120 - 100) * 100) / 120 = 2000 / 120 = 16 (integer division).
    #[test]
    fn calculation_test() {
        // Using the same function 'run' for both versions. In the vulnerable version,
        // the error in arithmetic results in 0, causing the test to fail.
        let result = run(120, 100);
        assert_eq!(result, 16, "Incorrect percentage calculation, expected 16 but got {}", result);
    }
}
