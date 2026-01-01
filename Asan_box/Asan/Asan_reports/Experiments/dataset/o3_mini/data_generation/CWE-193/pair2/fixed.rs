use std::sync::{Arc, Mutex};
use std::thread;
struct DataBuffer {
    data: Vec<u8>,
}
impl DataBuffer {
    fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        for i in 0..size {
            data.push(i as u8);
        }
        Self { data }
    }
    fn process(&self) -> u8 {
        *self.data.last().expect("Vector should not be empty")
    }
}
fn main() {
    let buffer = Arc::new(Mutex::new(DataBuffer::new(10)));
    let clone = Arc::clone(&buffer);
    let handle = thread::spawn(move || {
        let guard = clone.lock().unwrap();
        guard.process()
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fencepost_error() {
        // Create a buffer of size 10.
        let buffer = DataBuffer::new(10);
        // The correct behavior is to return the last valid element (which is 9).
        let expected = 9;
        let result = buffer.process();
        // If the off-by-one error occurs, the returned result will be undefined (and likely not 9).
        assert_eq!(result, expected, "Test failed: off-by-one error detected in element access.");
    }
}
