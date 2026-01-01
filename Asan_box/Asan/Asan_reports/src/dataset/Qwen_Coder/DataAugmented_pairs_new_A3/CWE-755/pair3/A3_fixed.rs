use std::sync::{Arc, Mutex};
use std::thread;

pub struct Core {
    pub count: u32,
}

impl Core {
    pub unsafe fn update(&mut self, add: u32) -> u32 {
        self.count = self.count.wrapping_add(add);
        self.count
    }
}

pub fn analyze(input: &str) -> Result<u32, &'static str> {
    if input.is_empty() {
        Err("Empty input not allowed")
    } else {
        Ok(input.len() as u32)
    }
}

pub fn run_app(input: &str) -> Result<(), &'static str> {
    let factor = analyze(input)?;
    let core_state = Arc::new(Mutex::new(Core { count: 0 }));
    let mut workers = vec![];
    for _ in 0..5 {
        let state = Arc::clone(&core_state);
        let f = factor;
        let handle = thread::spawn(move || unsafe {
            let mut lock = state.lock().unwrap();
            lock.update(f)
        });
        workers.push(handle);
    }
    for worker in workers {
        let res = worker.join().unwrap();
        println!("Worker result: {}", res);
    }
    Ok(())
}

fn main() {
    if let Err(e) = run_app("") {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The test oracle verifies that processing an empty string fails as expected.
    // For the flawed implementation, run_app("") would return Ok(()), causing the test to fail.
    // For the corrected implementation, run_app("") returns Err("Empty input not allowed"), and the test passes.
    #[test]
    fn test_exception_propagation() {
        let result = run_app("");
        assert!(result.is_err(), "Expected an error when input is empty");
        // Optionally, check the error message.
        if let Err(msg) = result {
            assert_eq!(msg, "Empty input not allowed");
        }
    }
}
