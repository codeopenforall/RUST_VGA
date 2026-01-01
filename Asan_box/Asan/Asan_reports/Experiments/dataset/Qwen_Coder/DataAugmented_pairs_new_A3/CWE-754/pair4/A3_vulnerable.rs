use std::sync::mpsc;
use std::thread;
use std::time::Duration;
pub fn process(input: u32) -> Result<u32, &'static str> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        if input == 1 {
            thread::sleep(Duration::from_millis(100));
        }
        unsafe {
            let ptr = &input as *const u32;
            let value = *ptr;
            let _ = tx.send(value);
        }
    });
    match rx.recv_timeout(Duration::from_millis(50)) {
        Ok(v) => Ok(v),
        Err(e) => {
            if let mpsc::RecvTimeoutError::Timeout = e {
                Ok(0) // Vulnerable: Returning Ok(0) instead of an error on timeout
            } else {
                Err("Channel disconnected")
            }
        }
    }
}
fn main() {
    match process(2) {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}

//////////////////////////////
// Test Oracle Function
// This test fails for the vulnerable version (which incorrectly returns Ok(0) on a timeout)
// and passes for the fixed version (which returns an error on timeout).
//////////////////////////////

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_timeout_condition() {
        // Use input=1, which triggers a sleep that exceeds the recv_timeout duration.
        let result = process(1);
        // For a proper implementation, a timeout should yield an error.
        assert!(
            result.is_err(),
            "Expected an error due to timeout, but got a successful result."
        );
        if let Err(err) = result {
            assert_eq!(err, "Operation timed out", "Unexpected error message.");
        }
    }
}
