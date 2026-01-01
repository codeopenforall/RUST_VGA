use std::panic;
use std::sync::{Arc, Mutex};
use std::thread;

trait Processor {
    fn run(&self, value: i32) -> Result<i32, String>;
}

struct Handler {
    data: Arc<Mutex<i32>>,
}

impl Processor for Handler {
    fn run(&self, value: i32) -> Result<i32, String> {
        let result = panic::catch_unwind(|| unsafe {
            let raw = &value as *const i32;
            let res = *raw;
            if res < 0 {
                panic!("Simulated panic in unsafe block: negative value");
            }
            res
        });
        match result {
            Ok(val) => Ok(val),
            Err(_) => Err("Caught panic in unsafe block".to_string()),
        }
    }
}

fn execute_task(value: i32) -> Result<i32, String> {
    let handler = Handler {
        data: Arc::new(Mutex::new(value)),
    };
    let shared_handler = Arc::new(handler);
    let handler_thread = shared_handler.clone();
    let join_handle = thread::spawn(move || {
        handler_thread.run(value)
    });
    match join_handle.join() {
        Ok(res) => res,
        Err(_) => Err("Thread panicked unexpectedly".to_string()),
    }
}

fn main() {
    match execute_task(-1) {
        Ok(output) => println!("Result: {}", output),
        Err(err) => println!("Handled error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    // Define a trait to uniformly query the error state of the output.
    trait Outcome {
        fn is_error(self) -> bool;
    }
    // For the fixed module, execute_task returns Result<i32, String>.
    impl Outcome for Result<i32, String> {
        fn is_error(self) -> bool {
            self.is_err()
        }
    }
    // For the vulnerable module, execute_task returns i32.
    impl Outcome for i32 {
        // In the vulnerable code, if the function returns a value, then no panic occurred.
        // However, as the negative input should have caused a panic, reaching here is an error.
        fn is_error(self) -> bool {
            false
        }
    }

    #[test]
    fn test_negative_input_handling() {
        // The oracle uses catch_unwind to intercept any unexpected panics.
        let outcome = panic::catch_unwind(|| crate::execute_task(-1));
        match outcome {
            // In the fixed version, the function call should not panic and should return an error result.
            Ok(result) => {
                assert!(result.is_error(), "Expected an error for negative input in safe handling");
            }
            // In the vulnerable version, a panic occurs, and the test will fail.
            Err(_) => panic!("Function panicked, which indicates unhandled exception vulnerability"),
        }
    }
}
