use std::sync::{Arc, Mutex};
use std::thread;

struct BufferWrapper {
    buffer: Vec<u8>,
}

impl BufferWrapper {
    fn new(size: usize) -> Self {
        let v = vec![0u8; size];
        BufferWrapper { buffer: v }
    }

    fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index >= self.buffer.len() {
            return Err("Index out-of-bounds");
        }
        self.buffer[index] = value;
        Ok(())
    }
}

fn main() {
    let wrapper = Arc::new(Mutex::new(BufferWrapper::new(10)));
    let wrapper_clone = Arc::clone(&wrapper);
    let handle = thread::spawn(move || {
        let mut lock = wrapper_clone.lock().unwrap();
        match lock.update(15, 55) {
            Ok(_) => println!("Update unexpectedly succeeded."),
            Err(e) => println!("Handled error: {}", e),
        }
    });
    handle.join().unwrap();
    println!("Execution complete: Vulnerability fixed.");
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // This test oracle is designed to work with the common API defined in both versions.
    // Both implementations expose a method named update() with the same signature.
    // The test invokes update() with an out-of-bounds index and expects an error.
    // In the vulnerable version, the index is not checked and the call returns Ok(()),
    // causing the test to fail. In the fixed version, the out-of-bound condition is detected,
    // an error is returned, and the test passes.

    #[test]
    fn oracle_test() {
        let wrapper = Arc::new(Mutex::new(crate::BufferWrapper::new(10)));
        let wrapper_clone = Arc::clone(&wrapper);
        let handle = thread::spawn(move || {
            let mut lock = wrapper_clone.lock().unwrap();
            lock.update(15, 55)
        });
        let result = handle.join().unwrap();
        assert!(result.is_err(), "Out-of-bounds write should be rejected.");
    }
}
