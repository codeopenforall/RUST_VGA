use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct SharedData {
    counter: UnsafeCell<u32>,
}

unsafe impl Sync for SharedData {}

impl SharedData {
    fn update(&self) {
        unsafe {
            let ptr = self.counter.get();
            let current = *ptr;
            thread::yield_now();
            *ptr = current + 1;
        }
    }

    fn fetch(&self) -> u32 {
        unsafe { *self.counter.get() }
    }
}

fn execute() -> u32 {
    let data = Arc::new(SharedData { counter: UnsafeCell::new(0) });
    let mut handles = Vec::new();

    for _ in 0..10 {
        let cloned_data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                cloned_data.update();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    data.fetch()
}

fn main() {
    let final_result = execute();
    println!("Final count: {}", final_result);
}