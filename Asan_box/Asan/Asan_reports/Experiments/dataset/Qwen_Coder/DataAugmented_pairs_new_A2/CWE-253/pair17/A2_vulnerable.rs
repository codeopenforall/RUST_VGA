use std::sync::{Arc, Mutex};
use std::thread;

struct Worker {
    data: Box<[u32]>,
}

impl Worker {
    fn new(size: usize) -> Result<Self, &'static str> {
        if size == 0 {
            Err("Invalid size for Worker initialization")
        } else {
            Ok(Worker { data: vec![1; size].into_boxed_slice() })
        }
    }

    fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        if idx >= self.data.len() {
            Err("Index out of bounds")
        } else {
            unsafe {
                let ptr = self.data.as_mut_ptr().add(idx);
                *ptr = val;
            }
            Ok(())
        }
    }
}

fn process_input(input: i32) -> i32 {
    let size = if input < 0 { 0 } else { 10 };
    let worker = Arc::new(Mutex::new(Worker::new(size).unwrap()));
    let mut handles = Vec::new();

    for i in 0..5 {
        let worker_arc = Arc::clone(&worker);
        handles.push(thread::spawn(move || {
            let mut guard = worker_arc.lock().unwrap();
            let ret = guard.update((i * 2) as usize, (i * 10) as u32);
            if ret.is_err() {
                ret.unwrap(); // This line is the injected defect
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let guard = worker.lock().unwrap();
    guard.data.iter().sum::<u32>() as i32
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    let result = process_input(input);
    println!("Result: {}", result);
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
