use std::sync::{Arc, Mutex};
use std::thread;

struct Container {
    buffer: Vec<u32>,
}

impl Container {
    fn new() -> Self {
        Container { buffer: vec![0; 10] }
    }

    fn update(&mut self, index: usize, value: u32) {
        if value < 1000 {
            self.buffer[index] = value;
        } else {
            eprintln!("Invariant violated: value must be less than 1000");
        }
    }
}

fn process(input: u32) {
    let shared = Arc::new(Mutex::new(Container::new()));
    let mut threads = vec![];
    for i in 0..3 {
        let shared_clone = Arc::clone(&shared);
        let inp = input;
        let handle = thread::spawn(move || {
            let mut guard = shared_clone.lock().unwrap();
            if i == 1 {
                guard.update(2, inp);
            } else {
                guard.update(i, 42);
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}

fn main() {
    process(1500);
}

/*
This test oracle function is designed to confirm the exploitability of the vulnerability in the vulnerable version 
and its remediation in the fixed version. The test calls the process() function with a deliberately invalid input (1500)
that should trigger a panic in the vulnerable variant, while the fixed variant should complete without panicking.

When linked with the vulnerable code, the test should fail because a panic occurs. Conversely, when linked with the fixed 
version, the test should pass.
*/
#[cfg(test)]
mod tests {
    use std::panic;

    // Import the process function.
    use super::process;

    #[test]
    fn oracle_test() {
        let result = panic::catch_unwind(|| {
            process(1500);
        });
        // In a secure implementation, the processing with invalid input should not panic.
        assert!(result.is_ok(), "Test oracle failed: operation panicked due to attacker-controlled input.");
    }
}
