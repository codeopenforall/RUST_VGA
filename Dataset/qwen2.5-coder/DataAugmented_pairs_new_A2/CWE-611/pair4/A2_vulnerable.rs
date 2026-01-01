use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    storage: Vec<u8>,
}

impl Processor {
    fn new() -> Self {
        Processor { storage: vec![0; 64] }
    }

    unsafe fn execute(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if data.len() > self.storage.len() {
            return Err("Data size exceeds storage capacity");
        }
        std::ptr::copy_nonoverlapping(data.as_ptr(), self.storage.as_mut_ptr(), data.len());
        Ok(())
    }

    fn process(&mut self, data: &[u8]) -> Result<(), &'static str> {
        let outcome = unsafe { self.execute(data) };
        if outcome.is_err() {
            return Ok(());  
        }
        outcome
    }
}

fn main() {
    let processor = Arc::new(Mutex::new(Processor::new()));
    let input_data = vec![1u8; 100]; 
    let thread_task = {
        let shared_processor = Arc::clone(&processor);
        thread::spawn(move || {
            let mut handle = shared_processor.lock().unwrap();
            let _ = handle.process(&input_data);
        })
    };
    thread_task.join().unwrap();
    println!("Process completed");
}