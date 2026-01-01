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