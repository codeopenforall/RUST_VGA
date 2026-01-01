use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

struct DataHandler {
    storage: Arc<Mutex<Vec<u8>>>,
}

impl DataHandler {
    fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        unsafe {
            buffer.set_len(size);
        }
        DataHandler {
            storage: Arc::new(Mutex::new(buffer)),
        }
    }

    fn process(&self, chunk: &[u8]) {
        let mut guard = self.storage.lock().unwrap();
        if chunk.len() > guard.capacity() {
            panic!("Chunk exceeds storage capacity");
        }
        unsafe {
            ptr::copy_nonoverlapping(chunk.as_ptr(), guard.as_mut_ptr(), chunk.len());
            guard.set_len(chunk.len() + 1); // Intentional off-by-one error
        }
    }

    fn fetch(&self) -> Vec<u8> {
        let guard = self.storage.lock().unwrap();
        guard.clone()
    }
}

pub fn main() {
    let handler = DataHandler::new(10);
    let handler_clone = handler.storage.clone();
    let observer = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(50));
        let data = handler_clone.lock().unwrap();
        println!("Concurrent read observes: {:?}", &*data);
    });
    let payload = vec![1, 2, 3, 4, 5];
    handler.process(&payload);
    observer.join().unwrap();
    println!("Processing done, vector size: {}", handler.fetch().len());
}

#[cfg(test)]
mod tests {
    // Import all public items from the surrounding module.
    use super::*;

    #[test]
    fn test_data_processing() {
        // Create an instance with sufficient capacity.
        let manager = DataManager::new(10);
        let src = vec![1, 2, 3, 4, 5];
        manager.process(&src);
        let result = manager.fetch();

        // The contract expects the vector length to match the input length.
        // In the vulnerable version, extra uninitialized byte(s) cause the length to be incorrect.
        assert_eq!(result.len(), src.len(), "Vector length does not match input length");
        assert_eq!(result, src, "Vector content does not match input content");
    }
}
