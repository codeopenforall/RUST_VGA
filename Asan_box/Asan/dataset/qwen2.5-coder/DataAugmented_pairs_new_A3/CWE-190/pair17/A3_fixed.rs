use std::sync::{Arc, Mutex};

fn multiply_and_store(a: u32, b: u32, shared_result: Arc<Mutex<u64>>) {
    let result = (a as u64)
        .checked_mul(b as u64)
        .expect("Multiplication overflow detected");

    let mut lock = shared_result.lock().unwrap();
    *lock = result;
}

fn main() {
    let shared_result = Arc::new(Mutex::new(0));
    let shared_result_clone = Arc::clone(&shared_result);

    multiply_and_store(10_000, 500_000, shared_result_clone);

    let result = *shared_result.lock().unwrap();
    println!("Result: {}", result);
}