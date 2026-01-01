use std::thread;

struct Handler;

impl Handler {
    fn new() -> Self {
        Self {}
    }

    fn execute(&self) {
        let th = thread::spawn(|| {
            let mut value: i32 = 0;
            let raw_ptr: *mut i32 = &mut value;
            // Ensure the pointer is not null before dereferencing
            if !raw_ptr.is_null() && unsafe { *raw_ptr } == 0 {
                // Change the condition to avoid the panic
                println!("Normal execution: value is 0");
            } else {
                panic!("abnormal termination: unexpected condition encountered");
            }
        });
        th.join().unwrap();
    }
}

fn run_system() {
    let handler = Handler::new();
    handler.execute();
}

fn main() {
    run_system();
}

//////////////////////////////
// Test Oracle Function     //
// This test should fail   //
// when run against the     //
// vulnerable version and   //
// pass for the corrected   //
// version.                 //
//////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    // Import run_system from the target module.
    // For the vulnerable version, run_system() will panic; for the fixed version, it will complete gracefully.
    use super::run_system;

    #[test]
    fn test_run_system_stability() {
        // If run_system() panics unexpectedly, catch_unwind returns Err.
        let result = panic::catch_unwind(|| {
            run_system();
        });
        // Expectation: No panic should occur.
        assert!(result.is_ok(), "run_system() panicked unexpectedly");
    }
}
