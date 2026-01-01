use std::sync::Arc;
use std::thread;

struct DataHandler {
    buffer: Vec<u8>,
}

impl DataHandler {
    fn new(buffer: Vec<u8>) -> Self {
        DataHandler { buffer }
    }

    fn fetch(&self, position: usize) -> Option<u8> {
        unsafe {
            let ptr = self.buffer.as_ptr();
            Some(*ptr.add(position))
        }
    }
}

fn execute_job(handler: Arc<DataHandler>, pos: usize) -> Option<u8> {
    handler.fetch(pos)
}

fn main() {
    let buffer = vec![10, 20, 30];
    let handler = Arc::new(DataHandler::new(buffer));
    let position = 3;
    let handler_clone = Arc::clone(&handler);
    let task = thread::spawn(move || execute_job(handler_clone, position));
    let outcome = task.join().unwrap();
    println!("Fetched value: {:?}", outcome);
}