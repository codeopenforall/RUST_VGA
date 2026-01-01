use std::sync::{Arc, Mutex};
use std::thread;

static mut GLOBAL_PTR: *mut u32 = std::ptr::null_mut();

pub fn run() -> u32 {
    let boxed = Box::new(0u32);
    let ptr = Box::into_raw(boxed);
    unsafe {
        GLOBAL_PTR = ptr;
    }

    let mutex = Arc::new(Mutex::new(()));
    let mut handles = vec![];

    for _ in 0..10 {
        let mutex_clone = Arc::clone(&mutex);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let _guard = mutex_clone.lock().unwrap();
                unsafe {
                    if !GLOBAL_PTR.is_null() {
                        let temp = *GLOBAL_PTR; 
                        let new_val = temp.wrapping_add(1); 
                        *GLOBAL_PTR = new_val; 
                    }
                }
            }
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    unsafe {
        let res = *GLOBAL_PTR;
        let _ = Box::from_raw(GLOBAL_PTR);
        res
    }
}

fn main() {
    let final_count = run();
    println!("Final counter value: {}", final_count);
}