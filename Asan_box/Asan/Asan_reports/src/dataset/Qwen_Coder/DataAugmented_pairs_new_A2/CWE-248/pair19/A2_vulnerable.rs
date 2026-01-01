use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Processor {
    fn new() -> Self {
        Self { data: Arc::new(Mutex::new(vec![10, 20, 30])) }
    }

    fn run_internal(&self) {
        let data_ref = self.data.clone();
        let handle = thread::spawn(move || {
            unsafe {
                let guard = data_ref.lock().unwrap();
                let ptr = guard.as_ptr();
                if *ptr > 10 { // Changed condition to introduce a vulnerability
                    panic!("Abnormal termination: unsafe access condition met");
                }
            }
            let _ = 42;
        });
        handle.join().unwrap();
    }
}

pub fn app() {
    let proc = Processor::new();
    proc.run_internal();
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
