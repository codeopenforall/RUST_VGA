use std::sync::{Arc, Mutex};
use std::thread;

struct Storage {
    data: Vec<u32>,
}

impl Storage {
    fn create() -> Self {
        Storage { data: vec![0; 10] }
    }
    fn modify(&mut self, idx: usize, val: u32) {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(idx) = val;
        }
        assert!(self.data[idx] < 1000, "Constraint breached: value must be below 1000");
    }
}

fn handle_input(value: u32) {
    let shared_storage = Arc::new(Mutex::new(Storage::create()));
    let mut thread_handles = vec![];

    for i in 0..3 {
        let storage_clone = Arc::clone(&shared_storage);
        let val = value;
        let thread = thread::spawn(move || {
            let mut lock = storage_clone.lock().unwrap();
            if i == 1 {
                lock.modify(2, val);
            } else {
                lock.modify(i, 42);
            }
        });
        thread_handles.push(thread);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}

fn main() {
    handle_input(1500);
}