use std::sync::{Arc, Mutex};
use std::thread;

struct Worker {
    // Define the fields of the Worker struct
}

impl Worker {
    fn new(size: usize) -> Result<Self, String> {
        // Implement the new method to return a Result
        if size <= 0 {
            Err("Invalid size".to_string())
        } else {
            Ok(Worker { /* Initialize fields */ })
        }
    }

    fn update(&mut self, index: usize, value: u32) -> Result<(), String> {
        // Implement the update method to return a Result
        if index >= 100 {
            Err("Index out of bounds".to_string())
        } else {
            // Update logic
            Ok(())
        }
    }
}

fn process_input(size: isize) -> i32 {
    if size < 0 {
        eprintln!("Invalid input size: {}", size);
        return -1;
    }

    let worker_instance = match Worker::new(size as usize) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error during worker initialization: {}", e);
            return -1;
        }
    };

    let worker = Arc::new(Mutex::new(worker_instance));

    let mut handles = vec![];

    for i in 0..10 {
        let worker_clone = Arc::clone(&worker);
        let handle = thread::spawn(move || {
            let mut guard = worker_clone.lock().unwrap();
            if let Err(e) = guard.update((i * 2) as usize, (i * 10) as u32) {
                eprintln!("Update error in thread {}: {}", i, e);
                return -1;
            }
            0
        });
        handles.push(handle);
    }

    let mut result = 0;
    for handle in handles {
        if let Err(_) = handle.join() {
            eprintln!("A thread panicked during execution.");
            return -1;
        }
    }

    println!("Result: {}", result);
    result
}

fn main() {
    // Example usage
    let result = process_input(10);
    println!("Final result: {}", result);
}

/*
   Test oracle: This unit test is designed to detect the vulnerability.
   It calls process_input with a negative value which, in the vulnerable version,
   triggers a panic due to improper error handling, whereas in the fixed version, it 
   gracefully returns an error code (-1).

   The test structure below should be added as a separate test module and built/executed 
   independently from the vulnerable and fixed binaries.
*/
#[cfg(test)]
mod tests {
    use std::panic;

    // Import the process_input function from the binary under test.
    // For the vulnerable build, process_input returns i32 and panics on error.
    // For the fixed build, process_input returns -1 gracefully on error.
    use super::process_input;

    #[test]
    fn test_oracle() {
        // Using negative input to trigger error handling.
        let test_input = -5;

        // Catch unwinding to determine if the function panics (vulnerable) or returns gracefully (fixed).
        let result = panic::catch_unwind(|| process_input(test_input));

        if result.is_err() {
            // The vulnerable version will panic, causing the test to fail.
            panic!("Test oracle: Vulnerability detected - function panicked on invalid input.");
        } else {
            let output = result.unwrap();
            // The fixed version is expected to return -1 for a negative input.
            assert_eq!(output, -1, "Test oracle: Unexpected output. Expected -1 for invalid input.");
        }
    }
}
