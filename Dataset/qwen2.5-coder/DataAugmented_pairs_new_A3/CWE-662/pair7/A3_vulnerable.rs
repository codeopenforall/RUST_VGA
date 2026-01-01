use std::sync::{Arc, Mutex};
use std::thread;

struct Core {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Core {
    fn new(size: usize) -> Self {
        Core {
            data: Arc::new(Mutex::new(vec![0; size])),
        }
    }

    fn update(&self, index: usize, value: i32) -> Result<(), &'static str> {
        let lock = self.data.lock().map_err(|_| "Lock poisoned")?;
        if index >= lock.len() {
            return Err("Index out of bounds");
        }
        unsafe {
            let ptr = lock.as_ptr() as *mut i32;
            *ptr.add(index) = value;
        }
        Ok(())
    }

    fn concurrent_update(&self, index: usize, value: i32) -> thread::JoinHandle<()> {
        let data_clone = self.data.clone();
        thread::spawn(move || {
            let lock = data_clone.lock().unwrap();
            unsafe {
                let ptr = lock.as_ptr() as *mut i32;
                *ptr.add(index) = value;
            }
        })
    }
}

fn main() {
    let engine = Core::new(10);
    if let Err(e) = engine.update(9, 42) {
        eprintln!("update error: {}", e);
        return;
    }
    let handle = engine.concurrent_update(8, 55);
    handle.join().unwrap();
    assert!(engine.update(15, 99).is_err(), "Expected error for out-of-bound update");
    println!("Processing complete");
}