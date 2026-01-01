use std::sync::{Arc, Mutex};
use std::thread;

pub struct MemoryHandler {
    data: Vec<u8>,
}

impl MemoryHandler {
    pub fn new() -> Self {
        MemoryHandler { data: vec![0u8; 100] }
    }

    pub fn update(&mut self, idx: usize, value: u8) -> bool {
        if idx < self.data.len() {
            self.data[idx] = value;
            true
        } else {
            false
        }
    }

    pub fn fetch(&self, idx: usize) -> u8 {
        if idx < self.data.len() {
            self.data[idx]
        } else {
            0
        }
    }
}

fn main() {
    let handler = Arc::new(Mutex::new(MemoryHandler::new()));
    let mut threads = vec![];
    for i in 0..10 {
        let h = Arc::clone(&handler);
        threads.push(thread::spawn(move || {
            let mut mgr = h.lock().unwrap();
            let idx = 95 + i;
            let result = mgr.update(idx, 42);
            if !result {
                eprintln!("Attempted to update invalid index: {}", idx);
            }
        }));
    }
    for th in threads {
        th.join().unwrap();
    }
    let mgr = handler.lock().unwrap();
    println!("Buffer value at index 95: {}", mgr.fetch(95));
}

#[cfg(test)]
mod tests {
    use super::MemoryHandler;

    // Test oracle function: For the fixed version, updating an out-of-bound index should return false.
    // For the vulnerable version, it would incorrectly return true and corrupt memory.
    #[test]
    fn test_update_bounds() {
        let mut mgr = MemoryHandler::new();
        // In-range update - should succeed.
        assert_eq!(mgr.update(50, 99), true, "Valid index update should succeed");
        // Out-of-bound update - should be rejected in the fixed version.
        assert_eq!(mgr.update(150, 99), false, "Out-of-bound update should fail");
    }
}
