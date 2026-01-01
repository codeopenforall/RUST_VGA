use std::sync::{Arc, Mutex};

struct Processor {
    data_ref: Arc<Mutex<Vec<i32>>>,
}

impl Processor {
    fn new(data_ref: Arc<Mutex<Vec<i32>>>) -> Self {
        Processor { data_ref }
    }

    fn run_internal(&self) {
        unsafe {
            let guard = self.data_ref.lock().unwrap();
            let ptr = guard.as_ptr();
            if *ptr < 100 {
                panic!("Abnormal termination: unsafe access condition met");
            }
        }
        let _ = 42;
    }
}

fn app() {
    let data = vec![50];
    let data_ref = Arc::new(Mutex::new(data));
    let proc = Processor::new(data_ref);
    proc.run_internal();
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
