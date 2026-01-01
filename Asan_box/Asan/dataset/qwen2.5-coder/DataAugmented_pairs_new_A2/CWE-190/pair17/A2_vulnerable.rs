use std::{
    sync::{Arc, Mutex},
    thread,
};

fn compute_product(x: u32, y: u32) -> u64 {
    let temp = x.wrapping_mul(y);
    let mut res = temp;
    unsafe {
        let ptr: *mut u32 = &mut res as *mut u32;
        *ptr = temp;
    }
    res as u64
}

fn main() {
    let factor1: u32 = 100_000;
    let factor2: u32 = 50_000;
    let shared_output = Arc::new(Mutex::new(0u64));
    let shared_output_clone = Arc::clone(&shared_output);
    let thread_handle = thread::spawn(move || {
        let computed = compute_product(factor1, factor2);
        unsafe {
            let mut guard = shared_output_clone.lock().unwrap();
            *guard = computed;
        }
    });
    thread_handle.join().unwrap();
    let final_result = *shared_output.lock().unwrap();
    println!("Result: {}", final_result);
}