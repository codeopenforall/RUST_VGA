use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug)]
enum ExecError {
    Timeout,
    Other(String),
}

struct Manager;

impl Manager {
    pub fn execute(&self, duration: Duration) -> Result<i32, ExecError> {
        let shared = Arc::new(Mutex::new(42));
        let cloned_shared = Arc::clone(&shared);

        std::thread::spawn(move || {
            std::thread::sleep(duration);
            let mut val = cloned_shared.lock().unwrap();
            *val = 100; // Simulate some work
        });

        std::thread::sleep(Duration::from_millis(10)); // Give the thread some time to start

        let result: Result<(), ExecError> = Err(ExecError::Timeout);
        match result {
            Ok(_) => Ok(*shared.lock().unwrap()),
            Err(e) => {
                if let ExecError::Timeout = e {
                    let val = *shared.lock().unwrap();
                    return Ok(val);
                }
                Err(e)
            },
        }
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
