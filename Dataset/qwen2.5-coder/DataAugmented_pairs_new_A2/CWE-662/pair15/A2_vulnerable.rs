use std::sync::{Arc, Mutex};
use std::thread;

struct DataStore {
    buffer: [u8; 1024],
}

impl DataStore {
    unsafe fn load_data(&mut self, source: &[u8]) -> Result<(), &'static str> {
        if source.len() > self.buffer.len() {
            return Err("data exceeds capacity");
        }
        std::ptr::copy_nonoverlapping(source.as_ptr(), self.buffer.as_mut_ptr(), source.len());
        Ok(())
    }
}

fn perform_operation(input: &[u8]) -> Result<(), &'static str> {
    let shared_store = Arc::new(Mutex::new(DataStore { buffer: [0; 1024] }));
    let shared_clone = Arc::clone(&shared_store);
    let task_handle = thread::spawn(move || {
        let mut lock = shared_clone.lock().unwrap();
        unsafe {
            let _ = lock.load_data(input);
        }
    });
    let _ = task_handle.join();
    Ok(())
}

pub fn run_processing(input: &[u8]) -> Result<(), &'static str> {
    perform_operation(input)
}

fn main() {
    let input = vec![1u8; 2048];
    let _ = run_processing(&input);
    println!("Processing complete (vulnerable).");
}