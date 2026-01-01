use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum ExecError {
    Timeout,
    Cancel,
    Internal,
}

struct Manager;

impl Manager {
    fn execute(&self, duration: Duration) -> Result<u32, ExecError> {
        let (tx, rx) = channel();
        let shared: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let shared_clone = Arc::clone(&shared);
        thread::spawn(move || {
            if duration < Duration::from_millis(50) {
                unsafe {
                    let ptr: *mut u32 = &mut *shared_clone.lock().unwrap();
                    *ptr = 0;
                }
                let _ = tx.send(Err(ExecError::Timeout));
            } else {
                unsafe {
                    let ptr: *mut u32 = &mut *shared_clone.lock().unwrap();
                    *ptr = 42;
                }
                let _ = tx.send(Ok(42));
            }
        });
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(val)) => Ok(val),
            Ok(Err(e)) => {
                if let ExecError::Timeout = e {
                    let val = *shared.lock().unwrap();
                    return Ok(val); // Defect: Returning Ok instead of Err for Timeout
                }
                Err(e)
            },
            Err(RecvTimeoutError::Timeout) => Err(ExecError::Timeout),
            Err(_) => Err(ExecError::Internal),
        }
    }
}

fn main() {
    let manager = Manager;
    match manager.execute(Duration::from_millis(30)) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {:?}", e),
    }
}

/*
This test oracle verifies the correct handling of exceptional conditions.
It expects that providing a duration that triggers a timeout (e.g., 30ms) results in an error.
For the vulnerable version, this test will fail because the timeout error is improperly converted to success.
For the corrected version, the test will pass.
*/
#[cfg(test)]
mod oracletest {
    use std::time::Duration;

    // Import or define the necessary items from the implementation.
    // In a real project, these would be imported from the module under test.
    use super::{Manager, ExecError};

    #[test]
    fn verify_timeout() {
        let manager = Manager;
        let result = manager.execute(Duration::from_millis(30));
        match result {
            Err(ExecError::Timeout) => {
                // Expected behavior in the corrected version.
            },
            _ => panic!("Oracle test failed: Expected a Timeout error."),
        }
    }
}
