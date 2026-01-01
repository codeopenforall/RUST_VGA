use std::thread;

unsafe fn compute(val: i32) -> i32 {
    if val < 0 {
        panic!("Invalid value: negative input not allowed");
    }
    val * 2
}

fn run_task(input: i32) -> Result<i32, String> {
    let handle = thread::spawn(move || {
        if input < 0 {
            Err("Invalid value: negative input not allowed".to_string())
        } else {
            Ok(unsafe { compute(input) })
        }
    });

    match handle.join() {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(e)) => Err(e),
        Err(_) => Err("Thread panicked".to_string()),
    }
}

fn main() {
    let _ = run_task(-1);
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
