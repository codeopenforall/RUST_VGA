use std::sync::{Arc, Mutex};

pub struct DataBlock {
    data: Vec<u32>,
}

impl DataBlock {
    pub fn new(size: usize) -> Self {
        DataBlock {
            data: vec![0; size],
        }
    }

    pub fn fill(&mut self) {
        for i in 0..=self.data.len() {
            self.data[i] = i as u32;
        }
    }
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
