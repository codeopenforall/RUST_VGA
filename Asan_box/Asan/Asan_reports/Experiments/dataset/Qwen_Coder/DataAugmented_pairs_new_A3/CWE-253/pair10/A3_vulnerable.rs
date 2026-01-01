use std::sync::{Arc, Mutex};
use std::thread;
struct DataBuffer {
    data: Box<[u32]>,
}
impl DataBuffer {
    fn new(size: usize) -> Self {
        let vec = vec![0; size].into_boxed_slice();
        DataBuffer { data: vec }
    }
    fn modify(&mut self, index: usize, new_val: u32) -> Result<(), &'static str> {
        if index >= self.data.len() {
            return Err("Index out of bounds");
        }
        unsafe {
            let ptr = self.data.as_mut_ptr().add(index);
            *ptr = new_val;
        }
        Ok(())
    }
}
pub fn process_update() -> bool {
    let shared = Arc::new(Mutex::new(DataBuffer::new(10)));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let result = shared_clone.lock().unwrap().modify(10, 42);
        if result.is_err() {
            true
        } else {
            false
        }
    });
    handle.join().unwrap()
}
fn main() {
    let outcome = process_update();
    if outcome {
        println!("Operation succeeded.");
    } else {
        println!("Operation failed.");
    }
}

//////////////////////////////////////////////////////////////
// Test Oracle Function
// This test should be compiled separately from the vulnerable
// and fixed executables. It calls the process_update() function
// and asserts that the outcome is false, which is the expected
// result (operation failure) when an invalid index is provided.
// In the vulnerable code, the function erroneously returns true,
// causing the test to fail.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Adjust the module path as necessary to refer to the function.
    use super::process_update;

    #[test]
    fn validate_update_outcome() {
        // For a request to update an out-of-bound index, the correct behavior is to fail.
        // This test will fail in the vulnerable code (which incorrectly reports success)
        // and pass in the fixed code.
        assert_eq!(
            process_update(), 
            false, 
            "Expected the out-of-bound update to fail, but it reported success."
        );
    }
}
