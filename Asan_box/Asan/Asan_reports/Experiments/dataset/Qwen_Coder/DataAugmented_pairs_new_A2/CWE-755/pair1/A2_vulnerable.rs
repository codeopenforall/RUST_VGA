use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
struct Data {
    value: i32,
}

impl Data {
    unsafe fn compute(input: i32) -> Result<i32, &'static str> {
        let ptr = &input as *const i32;
        if *ptr < 0 {
            Err("input negative")
        } else {
            Ok(*ptr * 2)
        }
    }
}

pub fn run_contract(inputs: &[i32]) -> Result<Vec<i32>, &'static str> {
    let shared = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    for &input in inputs {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let res = unsafe { Data::compute(input).unwrap_or_default() }; // Defect: unwrap_or_default should not be used here
            let mut vec = shared_clone.lock().unwrap();
            vec.push(res);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let vec = shared.lock().unwrap();
    Ok(vec.clone())
}

fn main() {
    let inputs = vec![-5, 10, -3, 7];
    match run_contract(&inputs) {
        Ok(results) => println!("Aggregated Results: {:?}", results),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exception_condition() {
        // Using a triggering input with a negative value.
        let inputs = vec![-1, 5];
        let result = run_contract(&inputs);
        // The contract expected for correct behavior is to return an error on negative input.
        // Therefore, if the function returns Ok, it indicates that the exception condition was swallowed.
        assert!(result.is_err(), "Expected error for negative input, got Ok: {:?}", result);
    }
}

// A standalone oracle function that can be used for external testing.
fn oracle() {
    let inputs = vec![-1, 5];
    let result = run_contract(&inputs);
    if result.is_ok() {
        panic!("Test oracle failed: expected error for negative input.");
    }
}
