use std::env;
use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    pub fn new(data: Vec<u8>) -> Self {
        Buffer { data }
    }

    pub fn fetch(&self, idx: usize) -> u8 {
        if idx == 0 || idx > self.data.len() {
            panic!("Index out of bounds");
        }
        self.data[idx - 1]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let index: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    let shared = Arc::new(Buffer::new(vec![10, 20, 30, 40]));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let val = shared_clone.fetch(index);
        println!("Thread Fetched value: {}", val);
    });
    handle.join().unwrap();
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test verifies the contract that an invalid index (0) should not be allowed.
    // In the corrected code, the fetch method will panic on index 0.
    // For the vulnerable implementation, the unsafe access might not panic as expected,
    // causing this test to fail.
    #[test]
    fn contract_test() {
        let buffer = Buffer::new(vec![10, 20, 30, 40]);
        let result = panic::catch_unwind(|| {
            buffer.fetch(0);
        });
        assert!(
            result.is_err(), 
            "Expected panic on invalid index, but the call did not panic."
        );
    }
}
