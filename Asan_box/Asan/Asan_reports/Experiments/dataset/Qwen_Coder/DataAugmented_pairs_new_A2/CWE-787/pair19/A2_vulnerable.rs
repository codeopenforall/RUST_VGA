use std::sync::{Arc, Mutex};
use std::thread;
struct BufferManager {
    buffer: Vec<u8>,
}
impl BufferManager {
    fn new() -> Self {
        let mut data = Vec::with_capacity(10);
        data.resize(10, 0); 
        BufferManager { buffer: data }
    }
    fn apply(&mut self, idx: usize, value: u8) -> Result<(), String> {
        if idx < self.buffer.len() {
            self.buffer[idx] = value;
            Ok(())
        } else {
            Err("Index out of bounds".into())
        }
    }
    fn sum(&self) -> u32 {
        self.buffer.iter().map(|&b| b as u32).sum()
    }
}
fn main() {
    let manager = BufferManager::new();
    let shared = Arc::new(Mutex::new(manager));
    let handles: Vec<_> = (0..2)
        .map(|i| {
            let shared_ref = Arc::clone(&shared);
            thread::spawn(move || {
                let mut mgr = shared_ref.lock().unwrap();
                mgr.apply(9, 42 + i as u8).expect("Update failed");
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    let mgr = shared.lock().unwrap();
    println!("Sum: {}", mgr.sum());
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use super::BufferManager;

    #[test]
    fn test_update_sum() {
        // Initialize the BufferManager; initial sum should be 0.
        let initial_manager = BufferManager::new();
        let initial_sum = initial_manager.sum();

        let shared = Arc::new(Mutex::new(initial_manager));
        // Spawn a thread to update the last valid index.
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut mgr = shared_clone.lock().unwrap();
            // Update the last element (index 9) with value 42.
            mgr.apply(9, 42).expect("Update operation failed");
        });
        handle.join().unwrap();

        let mgr = shared.lock().unwrap();
        let new_sum = mgr.sum();
        // The contract: The correct behavior updates the sum by exactly 42.
        assert_eq!(new_sum, initial_sum + 42, "Sum did not increase as expected");
    }
}
