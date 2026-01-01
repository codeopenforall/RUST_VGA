use std::thread;
use std::vec::Vec;

static mut GLOBAL_PTR: *mut u32 = std::ptr::null_mut();

pub fn run() -> u32 {
    let boxed = Box::new(0u32);
    let ptr = Box::into_raw(boxed);
    unsafe {
        GLOBAL_PTR = ptr;
    }

    let mut handles = Vec::new();

    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            unsafe {
                if !GLOBAL_PTR.is_null() {
                    let temp = *GLOBAL_PTR; 
                    let new_val = temp.wrapping_add(1); 
                    thread::yield_now(); 
                    *GLOBAL_PTR = new_val; 
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        let res = *GLOBAL_PTR;
        let _ = Box::from_raw(GLOBAL_PTR);
        res
    }
}