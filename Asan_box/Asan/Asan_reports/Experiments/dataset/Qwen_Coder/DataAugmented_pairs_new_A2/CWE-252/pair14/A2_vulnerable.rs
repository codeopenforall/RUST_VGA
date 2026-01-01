use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
struct Executor {
    flag: Arc<AtomicBool>,
}
impl Executor {
    fn new() -> Self {
        Executor { flag: Arc::new(AtomicBool::new(false)) }
    }
    fn run(&self, input: u32) -> bool {
        let flag = self.flag.clone();
        let handle = thread::spawn(move || {
            unsafe {
                let _ = Self::unsafe_op(input);
            }
            flag.store(true, Ordering::Relaxed);
        });
        handle.join().unwrap();
        self.flag.load(Ordering::Relaxed)
    }
    unsafe fn unsafe_op(input: u32) -> Result<(), &'static str> {
        if input == 42 {
            Err("Operation encountered an error")
        } else {
            Ok(())
        }
    }
}
fn process_input(input: u32) -> bool {
    let exec = Executor::new();
    exec.run(input)
}
fn main() {
    let input = 42;
    let outcome = process_input(input);
    println!("Outcome: {}", outcome);
}

/*
   The oracle function validates the operation's correctness in both implementations.
   It invokes process_input with the critical input value (42) that causes unsafe_op to return an error.

   For the vulnerable version, due to the unchecked return value, the flag is set to true, causing the test to fail.
   For the fixed version, the flag remains false when an error occurs, so the test passes.

   This function should be used in a separate test harness.
*/
#[cfg(test)]
mod tests {
    use super::process_input;

    #[test]
    fn test_error_propagation() {
        // For critical input that triggers the error.
        let result = process_input(42);
        // In the secure (fixed) version, result should be false.
        assert!(!result, "Expected false when operation fails, but got true");
    }

    #[test]
    fn test_success_case() {
        // For an input that does not trigger the error.
        let result = process_input(10);
        // For both vulnerable and fixed version, a successful operation should yield true.
        assert!(result, "Expected true for successful operation.");
    }
}
