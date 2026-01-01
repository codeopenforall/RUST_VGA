use std::sync::{Arc, Mutex};
use std::thread;
use std::alloc::{alloc, dealloc, Layout};

struct Processor;

impl Processor {
    pub fn heavy_work(&self, tasks: usize) -> Result<usize, String> {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::with_capacity(tasks);
        for i in 0..tasks {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                unsafe {
                    let size = (i % 1024) + 1;
                    let layout = Layout::from_size_align(size, 8).unwrap();
                    let mem = alloc(layout);
                    if mem.is_null() {
                        panic!("Memory allocation failed");
                    }
                    *(mem as *mut u8) = (i % 256) as u8;
                    dealloc(mem, layout);
                }
                let mut cnt = counter.lock().unwrap();
                *cnt += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().map_err(|_| "Thread panicked".to_string())?;
        }
        let result = *counter.lock().unwrap();
        Ok(result)
    }
}

fn main() {
    let processor = Processor;
    let tasks = 1500;
    match processor.heavy_work(tasks) {
        Ok(count) => println!("Completed {} tasks", count),
        Err(e) => println!("Error: {}", e),
    }
}

//////////////// Test Oracle ////////////////////
// This test oracle function is separate from the main code and is used in unit testing.
// It validates that excessive task allocation is rejected in the fixed version,
// whereas the vulnerable version (lacking the check) would erroneously accept unbounded tasks.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_exhaustion_prevention() {
        let processor = Processor;
        // Provide an input that exceeds the recommended maximum.
        let test_tasks = 1500;
        let result = processor.heavy_work(test_tasks);
        // The fixed version returns an error, which is the expected behavior.
        // In the vulnerable version, the error is NOT returned (and threads are spawned),
        // making the test fail.
        assert!(result.is_err(), "Expected error due to resource cap, but got success");
    }
}
