use std::sync::{Arc, Mutex};
use std::thread;

struct Calculation {
    value: i64,
}

impl Calculation {
    fn compute(&self, factor: i64) -> i64 {
        unsafe {
            let ptr: *const i64 = &self.value;
            *ptr + factor * 2 // Intentional defect: changed division to multiplication
        }
    }
}

fn main() {
    let shared = Arc::new(Mutex::new(Calculation { value: 10 }));
    let operation_factor = 5; 
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let result;
        {
            let calc = shared_clone.lock().unwrap();
            result = calc.compute(operation_factor);
        }
        result
    });
    let final_result = handle.join().unwrap();
    println!("Computed result: {}", final_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculation() {
        // For a Calculation with value 10 and factor 5, the correct result should be (10+5)/2 = 7.
        // The vulnerable version returns 12 whereas the fixed version returns 7.
        let calc = Calculation { value: 10 };
        let result = calc.compute(5);
        assert_eq!(result, 7, "Expected result to be 7, but got {}", result);
    }
}
