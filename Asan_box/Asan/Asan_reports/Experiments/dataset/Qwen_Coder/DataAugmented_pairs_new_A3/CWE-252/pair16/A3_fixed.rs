use std::cell::UnsafeCell;
use std::sync::{Arc, Mutex};
use std::thread;
pub trait Task {
    fn execute(&self) -> Result<(), &'static str>;
}
pub struct Worker {
    manager: Arc<Manager>,
    val: u8,
}
impl Task for Worker {
    fn execute(&self) -> Result<(), &'static str> {
        let res = self.manager.append(self.val);
        unsafe {
            let ptr = self.manager.raw_data();
            let _ = *ptr;
        }
        res
    }
}
pub struct Manager {
    data: Mutex<Vec<u8>>,
    capacity: usize,
}
impl Manager {
    pub fn new(cap: usize) -> Self {
        Self {
            data: Mutex::new(Vec::with_capacity(cap)),
            capacity: cap,
        }
    }
    pub fn append(&self, byte: u8) -> Result<(), &'static str> {
        let mut guard = self.data.lock().unwrap();
        if guard.len() >= self.capacity {
            return Err("capacity exceeded");
        }
        guard.push(byte);
        Ok(())
    }
    pub unsafe fn raw_data(&self) -> *const u8 {
        self.data.lock().unwrap().as_ptr()
    }
    pub fn len(&self) -> usize {
        let guard = self.data.lock().unwrap();
        guard.len()
    }
}
fn run() {
    let mgr = Arc::new(Manager::new(10));
    let mut threads = vec![];
    for i in 0..20 {
        let worker = Worker {
            manager: mgr.clone(),
            val: i as u8,
        };
        let handle = thread::spawn(move || {
            worker.execute().expect("Worker execution failed");
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
    println!("Final data length: {}", mgr.len());
}
fn main() {
    run();
}

//////////////////////////////////////////////////////////////
// Test Oracle Function
// This oracle function simulates a unit test that verifies the
// behavior difference between the two versions. When triggered
// with an input that forces the capacity to be exceeded, the
// vulnerable version will silently discard errors and produce an
// incorrect final state, whereas the fixed version will panic
// (or in a real test environment, propagate the error) ensuring
// that the invariant is enforced.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // In this oracle test, we attempt to run the 'run' function.
    // For the vulnerable version, the ignored errors will let the test
    // complete (possibly with an incorrect final length), which we flag
    // as a failure. In the fixed version, the unchecked error is caught,
    // and a panic occurs, allowing the test to succeed in protecting the invariant.
    
    #[test]
    fn test_oracle() {
        // We expect the fixed version to panic due to capacity exceeded.
        // To simulate the trigger, spawn a thread that causes the invariant to fail.
        let result = panic::catch_unwind(|| {
            run(); // This will panic in the fixed version when a write fails.
        });
        // For the fixed version, the result should be an error.
        // Adjust the expectation based on which version is under test.
        //
        // For demonstration, we assert that a panic occurred (i.e., error was not ignored).
        assert!(result.is_err(), "Test failed: operation error was ignored");
    }
}
