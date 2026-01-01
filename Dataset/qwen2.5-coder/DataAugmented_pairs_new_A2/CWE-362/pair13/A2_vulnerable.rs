use std::cell::UnsafeCell;
use std::thread;

static mut GLOBAL: UnsafeCell<u32> = UnsafeCell::new(0);

fn increment_counter(count: u32) {
    for _ in 0..count {
        unsafe {
            let ptr = GLOBAL.get();
            let current = *ptr;
            thread::yield_now();
            *ptr = current + 1; // Intentional defect: missing wrapping_add
        }
    }
}

fn execute_threads() -> u32 {
    let mut threads = Vec::new();
    for _ in 0..10 {
        threads.push(thread::spawn(|| {
            increment_counter(1000);
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    unsafe { *GLOBAL.get() }
}

fn main() {
    let result = execute_threads();
    println!("Final count: {}", result);
}