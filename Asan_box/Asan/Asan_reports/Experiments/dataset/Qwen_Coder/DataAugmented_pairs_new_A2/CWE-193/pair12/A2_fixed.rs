use std::sync::{Arc, Mutex};
use std::thread;

struct DataBuffer {
    data: Vec<u32>,
}

impl DataBuffer {
    fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);
        Self { data }
    }

    fn populate(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = i as u32;
        }
    }

    fn total(&self) -> u32 {
        self.data.iter().sum()
    }
}

fn compute() -> u32 {
    let shared = Arc::new(Mutex::new(DataBuffer::new(10)));
    let shared_clone = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        let mut guard = shared_clone.lock().expect("failed to lock");
        guard.populate();
    });
    handler.join().expect("Thread panicked");
    let guard = shared.lock().expect("failed to lock after join");
    guard.total()
}

fn main() {
    let result = compute();
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    // Import compute from the outer scope.
    use super::compute;

    #[test]
    fn test_computation() {
        // The expected result is the sum of indices 0 through 9.
        // 0 + 1 + 2 + ... + 9 = 45.
        let result = compute();
        // This assertion should fail with the vulnerable version due to potential memory corruption,
        // and pass with the corrected implementation.
        assert_eq!(result, 45, "The computed sum does not match the expected value");
    }
}
