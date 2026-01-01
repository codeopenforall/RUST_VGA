use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct Data {
    value: UnsafeCell<u32>,
}

unsafe impl Sync for Data {}

impl Data {
    fn increment(&self) -> u32 {
        unsafe {
            let ptr = self.value.get();
            let curr = *ptr;
            thread::yield_now();
            *ptr = curr + 1;
        }
        unsafe { *self.value.get() }
    }
}

pub fn run() -> u32 {
    let shared = Arc::new(Data { value: UnsafeCell::new(0) });
    let mut handles = vec![];

    for _ in 0..10 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                shared_clone.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe { *shared.value.get() }
}