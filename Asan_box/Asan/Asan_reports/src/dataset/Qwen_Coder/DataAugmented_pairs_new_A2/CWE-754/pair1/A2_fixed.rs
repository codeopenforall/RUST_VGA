use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

struct Operation;

impl Operation {
    pub fn run() -> Result<(), &'static str> {
        let (tx, rx) = channel();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(150)); // Increase sleep time to ensure timeout
            let _ = tx.send(42);
        });
        let timeout = Duration::from_millis(10);
        let _start = Instant::now();
        let result = rx.recv_timeout(timeout);
        match result {
            Ok(val) => {
                if val == 42 {
                    Ok(())
                } else {
                    Err("Unexpected result")
                }
            }
            Err(e) => {
                match e {
                    RecvTimeoutError::Timeout => Err("Operation timed out"),
                    RecvTimeoutError::Disconnected => Err("Channel disconnected unexpectedly"),
                }
            }
        }
    }
}

fn main() {
    match Operation::run() {
        Ok(_) => println!("Operation succeeded (fixed behavior)!"),
        Err(err) => println!("Operation failed: {}", err),
    }
}

/////////////////////////////////////////////////////////////
// Test Oracle Function
// This function acts as a unit-test oracle that is used to validate the 
// correctness of the operation. It calls the run method, and asserts that 
// the operation correctly reports a failure due to the timeout.
// In the vulnerable version, the test will fail because it erroneously
// indicates success on timeout, whereas in the fixed version the test passes.
/////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Operation;

    #[test]
    fn test_timeout_handling() {
        // Expecting an error due to timeout.
        let res = Operation::run();
        assert!(res.is_err(), "Expected an error because the operation timed out");
        if let Err(msg) = res {
            assert_eq!(msg, "Operation timed out", "Incorrect error message for timeout");
        }
    }
}
