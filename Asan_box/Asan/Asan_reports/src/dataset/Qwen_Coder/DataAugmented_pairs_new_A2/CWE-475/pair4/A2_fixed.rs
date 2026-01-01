-------------------------------------------------
#![allow(unused)]
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;
struct Executor {
    storage: Box<[u8]>,
}
impl Executor {
    fn new(capacity: usize) -> Self {
        Self {
            storage: vec![0u8; capacity].into_boxed_slice(),
        }
    }
    fn transfer(&mut self, source: &[u8]) -> Result<(), &'static str> {
        let input_len = source.len();
        let desired = input_len.checked_add(10).ok_or("Length overflow")?;
        if desired > self.storage.len() || desired > source.len() {
            return Err("Requested copy length is invalid; preconditions not met.");
        }
        unsafe {
            let dest_ptr = self.storage.as_mut_ptr();
            let src_ptr = source.as_ptr();
            ptr::copy_nonoverlapping(src_ptr, dest_ptr, desired);
        }
        Ok(())
    }
    fn execute(&mut self, source: Vec<u8>) -> Result<(), &'static str> {
        let shared = Arc::new(Mutex::new(self));
        let mut handles = Vec::new();
        for _ in 0..2 {
            let shared_clone = Arc::clone(&shared);
            let source_clone = source.clone();
            let handle = thread::spawn(move || {
                let mut locked = shared_clone.lock().unwrap();
                locked.transfer(&source_clone).unwrap();
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        Ok(())
    }
}
fn main() {
    let payload = vec![42u8; 15];
    let mut runner = Executor::new(30); 
    runner.execute(payload).expect("Execution failed due to invalid preconditions.");
    println!("Completed execution (fixed).");
}
-------------------------------------------------


-------------------------------------------------
#[cfg(test)]
mod tests {
    use std::panic;

    // The test oracle calls the program's entry point.
    // For the vulnerable version, out-of-bound copying should trigger undefined behavior
    // (which we simulate by capturing a potential panic). For the fixed version, the check ensures
    // that the operation completes successfully.
    #[test]
    fn test_precondition_enforcement() {
        // We call the main function of the compiled binary.
        // In a controlled environment, the vulnerable version is expected to fail (panic)
        // while the fixed version passes.
        let result = panic::catch_unwind(|| {
            // Replace the following `crate::main()` call with the appropriate main function call
            // from the compiled binary. For demonstration, we call main directly.
            super::main();
        });
        // Assert that the invocation completed without panicking.
        // When testing the vulnerable binary, this assertion will fail.
        assert!(result.is_ok(), "Invariant violated: operation did not complete as expected.");
    }
}
-------------------------------------------------
