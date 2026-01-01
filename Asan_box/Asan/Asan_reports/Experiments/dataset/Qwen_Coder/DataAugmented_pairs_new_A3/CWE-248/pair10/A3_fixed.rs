use std::panic;
use std::thread;

struct Processor;

impl Processor {
    unsafe fn execute(&self, divisor: i32) -> i32 {
        if divisor == 0 {
            panic!("Division by zero");
        }
        10 / divisor
    }
}

static mut PROC: Processor = Processor;

fn perform_action(divisor: i32) -> Result<i32, &'static str> {
    let result = panic::catch_unwind(|| {
        unsafe { PROC.execute(divisor) }
    });
    match result {
        Ok(val) => Ok(val),
        Err(_) => Err("Operation failed: division by zero or invalid input"),
    }
}

fn main() {
    let handle = thread::spawn(|| {
        match perform_action(0) {
            Ok(result) => println!("Computed result: {}", result),
            Err(e) => println!("Error encountered: {}", e),
        }
    });

    handle.join().expect("Thread panicked unexpectedly");
}

///////////////////////////////////////////////////////////////
// Test Oracle Function:
// This unit test is meant to be compiled and run separately
// (e.g., via cargo test). When run against the vulnerable
// version, it will fail because the panic escapes; for the fixed
// version, the panic is caught and the test passes.
///////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::panic;

    // The test calls the main function of the module.
    // For the vulnerable variant, main() panics and the test
    // asserts a failure; for the fixed version, main() executes
    // gracefully.
    #[test]
    fn oracle() {
        let result = panic::catch_unwind(|| {
            // Call the program entry point.
            super::main();
        });
        // Expecting no panic (i.e. Ok) for the fixed code.
        assert!(result.is_ok(), "Program terminated abnormally");
    }
}
