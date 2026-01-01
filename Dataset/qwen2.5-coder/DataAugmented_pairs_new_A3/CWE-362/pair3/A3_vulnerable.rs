use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

const NUM_THREADS: usize = 10;
const ITER: usize = 4000;

struct SharedData {
    value: UnsafeCell<i32>,
}

unsafe impl Sync for SharedData {}

impl SharedData {
    fn new() -> Self {
        SharedData {
            value: UnsafeCell::new(0),
        }
    }

    fn increment(&self) {
        unsafe {
            let current = *self.value.get();
            let next = current + 1;
            *self.value.get() = next;
        }
    }

    fn get_value(&self) -> i32 {
        unsafe { *self.value.get() }
    }
}

fn main() {
    let shared_data = Arc::new(SharedData::new());

    let mut handles = vec![];

    for _ in 0..NUM_THREADS {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            for _ in 0..ITER {
                data_clone.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = shared_data.get_value();
    println!("Final counter: {}", result);
    panic!("Race condition detected: expected {} but found {}", NUM_THREADS * ITER, result);
}