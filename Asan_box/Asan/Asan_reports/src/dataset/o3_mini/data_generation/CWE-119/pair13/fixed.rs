use std::sync::{Arc, Mutex};
use std::thread;
struct Buffer {
    data: Vec<u8>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        Buffer { data: vec![0; size] }
    }
    pub fn update(&mut self, index: usize, val: u8) -> Result<(), &'static str> {
        if index < self.data.len() {           
            unsafe {
                let ptr = self.data.as_mut_ptr().add(index);
                *ptr = val;
            }
            Ok(())
        } else {
            Err("index out-of-bound")
        }
    }
    pub fn read(&self, index: usize) -> Result<u8, &'static str> {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_ptr().add(index);
                Ok(*ptr)
            }
        } else {
            Err("index out-of-bound")
        }
    }
}
fn main() {
    let buf = Arc::new(Mutex::new(Buffer::new(5)));
    let mut handles = vec![];
    for i in 0..10 {
        let buf_clone = Arc::clone(&buf);
        let handle = thread::spawn(move || {
            let mut locked = buf_clone.lock().unwrap();
            let _ = locked.update(i, i as u8);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Execution completed in fixed build.");
}


//////////////////////////////////////////////
// Test Oracle
// This test validates the proper behavior of the update function.
// It asserts that updating a valid index (4) succeeds and that updating an
// invalid index (5) returns an error. In the vulnerable code, the out-of-bound
// update erroneously succeeds, causing the test to fail. In the fixed code,
// the test passes.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_bounds() {
        let mut buf = Buffer::new(5);
        // Valid update within bounds.
        assert!(buf.update(4, 42).is_ok());
        // Out-of-bound update must be rejected.
        // In the vulnerable instance, this incorrectly returns Ok, failing the test.
        assert!(buf.update(5, 99).is_err());
    }
}
