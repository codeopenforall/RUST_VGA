use std::thread;

unsafe fn process_value(val: i32) -> i32 {
    if val < 0 {
        panic!("Invalid value: negative input not allowed");
    }
    val * 2
}

fn execute_task(input: i32) -> Result<i32, String> {
    let handle = thread::spawn(move || {
        unsafe { process_value(input) }
    });
    handle.join().map_err(|_| "Thread join failed".to_string())
}

fn main() {
    let _ = execute_task(-1);
}

///////////////////////// Test Oracle /////////////////////////
// This test ensures that when a negative value is supplied, the function returns an error.
// In the vulnerable version, the test would fail (i.e. panic) due to an uncaught exception,
// while in the fixed version, the error is caught and the test passes.

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_abnormal_input() {
        // For abnormal input (-1), the fixed implementation is expected
        // to catch and return an error rather than panicking.
        let result = run_task(-1);
        assert!(result.is_err(), "Expected an error for negative input");
    }
}
