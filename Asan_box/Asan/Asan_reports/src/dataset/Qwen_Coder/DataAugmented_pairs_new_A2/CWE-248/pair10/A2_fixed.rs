use std::thread;
use std::panic;
trait Operation {
    unsafe fn execute(&self, divisor: i32) -> i32;
}
struct Processor {
    base: i32,
}
impl Operation for Processor {
    unsafe fn execute(&self, divisor: i32) -> i32 {
        self.base / divisor
    }
}
fn perform_action(divisor: i32) -> Result<i32, &'static str> {
    let proc = Processor { base: 100 };
    let result = panic::catch_unwind(|| {
        unsafe { proc.execute(divisor) }
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
