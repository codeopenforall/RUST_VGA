use std::sync::{Arc, Mutex};
use std::thread;

fn compute_high(value: i32) -> i32 {
    value.wrapping_mul(2)
}

fn compute_low(value: i32) -> i32 {
    value + 5
}

fn execute_tasks() -> Vec<i32> {
    let high_value = Arc::new(Mutex::new(20));
    let low_value = Arc::new(Mutex::new(10));

    let high_value_clone = Arc::clone(&high_value);
    let low_value_clone = Arc::clone(&low_value);

    let handle_high = thread::spawn(move || {
        let mut value = high_value_clone.lock().unwrap();
        compute_high(*value)
    });

    let handle_low = thread::spawn(move || {
        let mut value = low_value_clone.lock().unwrap();
        compute_low(*value)
    });

    let result_high = handle_high.join().unwrap();
    let result_low = handle_low.join().unwrap();

    vec![result_high, result_low, result_high, result_low]
}