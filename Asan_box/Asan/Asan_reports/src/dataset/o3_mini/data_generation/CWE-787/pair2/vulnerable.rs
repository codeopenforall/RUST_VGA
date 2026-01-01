use std::sync::{Arc, Mutex};
use std::thread;
struct BufferWrapper {
    buffer: Vec<u8>,
}
impl BufferWrapper {
    fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        unsafe { v.set_len(size); }
        BufferWrapper { buffer: v }
    }
    fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        unsafe {
            let ptr = self.buffer.as_mut_ptr().add(index);
            *ptr = value;
        }
        Ok(())
    }
}
fn main() {
    let wrapper = Arc::new(Mutex::new(BufferWrapper::new(10)));
    let wrapper_clone = Arc::clone(&wrapper);
    let handle = thread::spawn(move || {
        let mut lock = wrapper_clone.lock().unwrap();
        let _ = lock.update(15, 55);
    });
    handle.join().unwrap();
    println!("Execution complete: Vulnerability present.");
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
