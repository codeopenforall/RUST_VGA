use std::sync::{Arc, Mutex};
use std::thread;

pub struct Calculator;

impl Calculator {
    pub unsafe fn compute(&self, a: i32, b: i32) -> i32 {
        let a_ref: &i32 = &a;
        let b_ref: &i32 = &b;
        let numerator = *a_ref;
        let divisor = *b_ref; 
        numerator / divisor  
    }
}

pub unsafe fn perform_division(a: i32, b: i32) -> i32 {
    let calc = Calculator;
    calc.compute(a, b)
}

fn main() {
    let calc = Arc::new(Calculator);
    let data = Arc::new(Mutex::new(vec![(10, 2), (10, 0)])); 
    let mut handles = Vec::new();
    for _ in 0..data.lock().unwrap().len() {
        let calc_clone = calc.clone();
        let data_clone = data.clone();
        let handle = thread::spawn(move || {
            let pair = data_clone.lock().unwrap().pop().unwrap();
            unsafe {
                let res = calc_clone.compute(pair.0, pair.1);
                println!("Computed result: {}", res);
                res
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}

//////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;

    // This test oracle targets the perform_division function.
    // When given a divisor of zero, the vulnerable version will panic,
    // whereas the corrected version returns a safe fallback value (0).
    #[test]
    fn division_oracle() {
        // The perform_division function must be in scope from the binary under test.
        // For demonstration purposes, we assume it is linked from the current crate.
        // Replace `crate::perform_division` with the correct path if necessary.
        let result = panic::catch_unwind(|| unsafe { crate::perform_division(10, 0) });
        
        // In the fixed version, the result should be Ok(0),
        // while in the vulnerable version, a panic will be caught.
        match result {
            Ok(val) => assert_eq!(val, 0, "Expected fallback value of 0 when dividing by zero"),
            Err(_) => panic!("Test failed: Division by zero caused a panic, indicating a vulnerability."),
        }
    }
}
