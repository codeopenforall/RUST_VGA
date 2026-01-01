use std::vec::Vec;
use std::sync::{Arc, Mutex};
struct DataBlock {
    data: Vec<u32>,
}
impl DataBlock {
    fn new(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        unsafe {
            vec.set_len(size);
        }
        Self { data: vec }
    }
    fn fill(&mut self) {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            for i in 0..=self.data.len() {
                *ptr.add(i) = i as u32;
            }
        }
    }
    fn verify(&self) -> bool {
        let len = self.data.len();
        for i in 0..len {
            if self.data[i] != i as u32 {
                return false;
            }
        }
        true
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(DataBlock::new(10)));
    {
        let shared_clone = Arc::clone(&shared);
        let handle = std::thread::spawn(move || {
            let mut db = shared_clone.lock().unwrap();
            db.fill();
        });
        handle.join().unwrap();
    }
    let db = shared.lock().unwrap();
    println!("Buffer contents (vulnerable): {:?}", db.data);
}


///////////////////////////////////////////////
// Test oracle function for off-by-one error ///
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    // Import the DataBlock struct from the code under test.
    // Note: In real usage, ensure that the path is correctly set to the module containing DataBlock.
    use super::DataBlock;
    
    #[test]
    fn test_buffer() {
        const SIZE: usize = 10;
        let block = Arc::new(Mutex::new(DataBlock::new(SIZE)));
        
        // Spawn a thread to perform the fill operation.
        let block_clone = Arc::clone(&block);
        let handle = thread::spawn(move || {
            let mut db = block_clone.lock().unwrap();
            db.fill();
        });
        handle.join().unwrap();
        
        // After filling, verify that each element equals its index.
        let db = block.lock().unwrap();
        for i in 0..db.data.len() {
            assert_eq!(db.data[i], i as u32, "Buffer verification failed at index {}", i);
        }
    }
}
