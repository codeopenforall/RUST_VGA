use std::sync::{Arc, Mutex};
use std::thread;
use std::panic;

struct Runner {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Runner {
    fn new() -> Self {
        Self { data: Arc::new(Mutex::new(vec![10, 20, 30])) }
    }

    fn run_internal(&self) -> Result<(), &'static str> {
        let data_ref = self.data.clone();
        let handle = thread::spawn(move || {
            let result = panic::catch_unwind(|| {
                let guard = data_ref.lock().unwrap();
                if guard[0] < 100 {
                    panic!("Abnormal termination: unsafe access condition met");
                }
                let _ = 42;
            });
            result.map_err(|_| ())
        });
        match handle.join() {
            Ok(inner_result) => inner_result.map_err(|_| "Error: Panic caught in thread"),
            Err(_) => Err("Error: Thread panicked unexpectedly"),
        }
    }
}

pub fn app() {
    let runner = Runner::new();
    let _ = runner.run_internal();
}

fn main() {
    app();
}

#[cfg(test)]
mod tests {
    use std::panic;
    // The test assumes the crate provides a public function `app()` as the entry point.
    use crate::app;
    
    #[test]
    fn integration_test() {
        // Capture panics from the application entry point.
        let result = panic::catch_unwind(|| {
            app();
        });
        // For the fixed version the application should not panic.
        // For the vulnerable version, a panic will be caught causing this test to fail.
        assert!(result.is_ok(), "The application terminated abnormally due to an uncaught panic.");
    }
}
