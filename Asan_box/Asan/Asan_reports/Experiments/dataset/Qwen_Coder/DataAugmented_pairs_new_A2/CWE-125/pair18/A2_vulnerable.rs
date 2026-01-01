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

//////////////////////////////
// Test Oracle Function     //
// (Separate from the code) //
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // This test invokes the extraction method using an index that is just out-of-range.
    // In the correctly fixed version, the call should return None.
    #[test]
    fn test_extraction_bounds() {
        let data = vec![10, 20, 30]; // Buffer with 3 elements.
        let processor = Processor::new(data);
        let index = 3; // Index is equal to the length => out-of-bounds.
        let proc_arc = Arc::new(processor);
        let proc_clone = Arc::clone(&proc_arc);
        
        let handle = thread::spawn(move || {
            // Use the extraction method.
            proc_clone.extract(index)
        });
        let result = handle.join().expect("Thread panicked");

        // The expected outcome for a safe extraction is None.
        // In the vulnerable version, this assertion will fail (as it returns Some(_)).
        assert_eq!(None, result, "Extraction did not properly check bounds");
    }
}
