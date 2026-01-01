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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    // Test oracle function: In the fixed version, out-of-bound updates are rejected
    // and do not corrupt the vector. In the vulnerable version, the unchecked operations
    // may corrupt adjacent memory. This test should pass only for the secured implementation.
    #[test]
    fn integrity_check() {
        let engine = Core::new(10);
        
        // Valid update: set index 5 to 77.
        assert!(engine.update(5, 77).is_ok(), "Valid update failed");
        
        // Trigger an out-of-bound update concurrently.
        let _ = engine.concurrent_update(15, 88);
        thread::sleep(Duration::from_millis(100));
        
        // Reacquire the data safely.
        let data = engine.data.lock().unwrap();
        
        // Confirm that the valid update remains intact.
        assert_eq!(data[5], 77, "Valid update was unexpectedly altered");
        
        // Ensure that the out-of-bound update did not inadvertently modify any in-bounds element.
        if data.iter().any(|&x| x == 88) {
            panic!("Memory corruption detected: out-of-bound update modified valid memory");
        }
    }
}
