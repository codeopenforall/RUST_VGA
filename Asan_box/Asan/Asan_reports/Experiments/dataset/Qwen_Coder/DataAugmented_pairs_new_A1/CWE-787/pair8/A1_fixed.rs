use std::vec::Vec;
use std::thread;

struct BufferManager {
    buffer: Vec<u32>,
}

impl BufferManager {
    fn new() -> Self {
        BufferManager {
            buffer: vec![1, 1, 1, 1, 1],
        }
    }

    fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        if idx >= self.buffer.len() {
            return Err("Index out of bounds");
        }
        self.buffer[idx] = val;
        Ok(())
    }

    fn sum(&self) -> u32 {
        self.buffer.iter().sum()
    }
}

fn main() {
    let mut manager = BufferManager::new();
    let handle = thread::spawn(move || {
        let _ = manager.update(5, 42);
        println!("Sum: {}", manager.sum());
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    // Bring the definition of BufferManager into test scope.
    use super::*;

    #[test]
    fn test_contract() {
        let mut manager = BufferManager::new();
        let original = manager.sum();
        let res = manager.update(5, 42);
        // In the safe implementation, an out-of-bound update should be rejected with an error.
        assert!(res.is_err(), "Expected error for out-of-bound update");
        // The buffer must remain unchanged.
        assert_eq!(manager.sum(), original, "Buffer sum should remain unchanged on error");
    }
}
