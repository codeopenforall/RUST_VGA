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
        let mut lock = self.data.lock().map_err(|_| "Lock poisoned")?;
        if index >= lock.len() {
            return Err("Index out of bounds");
        }
        lock[index] = value;
        Ok(())
    }

    fn concurrent_update(&self, index: usize, value: i32) {
        let core_clone = self.data.clone();
        let _ = thread::spawn(move || {
            let mut lock = core_clone.lock().unwrap();
            if index < lock.len() {
                lock[index] = value;
            }
        });
    }
}

fn main() {
    let engine = Core::new(10);
    engine.concurrent_update(15, 42);
    let _ = engine.update(20, 99);
    thread::sleep(std::time::Duration::from_millis(50));
    println!("Processing complete");
}