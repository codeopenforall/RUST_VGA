use std::sync::{Arc, Mutex};
use std::thread;

fn compute_high(value: i32) -> i32 {
    return value + 5;
}

fn compute_low(value: i32) -> i32 {
    return value.wrapping_mul(2);
}

fn execute_tasks() -> Vec<i32> {
    let high_value = Arc::new(Mutex::new(20));
    let low_value = Arc::new(Mutex::new(10));

    let mut handles = vec![];

    // High priority task
    let high_clone = Arc::clone(&high_value);
    let handle_high = thread::spawn(move || {
        let mut value = high_clone.lock().unwrap();
        compute_high(*value)
    });
    handles.push(handle_high);

    // Low priority task
    let low_clone = Arc::clone(&low_value);
    let handle_low = thread::spawn(move || {
        let mut value = low_clone.lock().unwrap();
        compute_low(*value)
    });
    handles.push(handle_low);

    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    results
}