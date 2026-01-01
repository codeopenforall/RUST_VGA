use std::sync::{Arc, Mutex};
use std::thread;

struct DataHandler;
impl DataHandler {
    pub fn handle(&self, input: &[u8]) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(input.len());
        unsafe {
            std::ptr::copy_nonoverlapping(input.as_ptr(), buffer.as_mut_ptr(), input.len());
            buffer.set_len(input.len() - 1); // Intentional off-by-one error
        }
        buffer
    }
}

fn execute(input: &[u8]) -> Vec<u8> {
    let handler = Arc::new(DataHandler);
    let dataset = Arc::new(input.to_vec());
    let result = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];
    for _ in 0..4 {
        let handler_clone = Arc::clone(&handler);
        let dataset_clone = Arc::clone(&dataset);
        let result_clone = Arc::clone(&result);
        let thread_handle = thread::spawn(move || {
            let processed = handler_clone.handle(&dataset_clone);
            let mut lock = result_clone.lock().unwrap();
            *lock = processed;
        });
        threads.push(thread_handle);
    }
    for thread_handle in threads {
        thread_handle.join().unwrap();
    }
    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}

fn main() {
    let input = b"abcdef";
    let output = execute(input);
    println!("Processed length: {}", output.len());
}