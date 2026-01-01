use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::Duration;

pub struct DataReader;

impl DataReader {
    pub fn obtain() -> Result<i32, &'static str> {
        let (tx, rx) = mpsc::channel();

        // Simulate a long-running task
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(2));
            tx.send(42).unwrap();
        });

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(value) => Ok(value),
            Err(RecvTimeoutError::Timeout) => return Err("timeout"),
            Err(RecvTimeoutError::Disconnected) => return Err("disconnected"),
        }
    }
}

//////////////////////////////////////////////////////////////////
// Test Oracle Function
// This oracle tests the data acquisition function. It verifies that, when
// a timeout condition is triggered (due to the intentionally short timeout),
// the function returns an error ("timeout"). In the vulnerable version, the
// function erroneously returns Ok(1), causing the test to fail; in the fixed
// version, the function correctly returns an error.
//////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::DataReader;

    #[test]
    fn test_timeout_condition() {
        // When the sensor read times out, the proper behavior in the fixed code is
        // to return an Err("timeout"). The vulnerable variant would incorrectly return Ok(1).
        let result = DataReader::obtain();
        assert_eq!(result, Err("timeout"), "The sensor read did not correctly fail on timeout");
    }
}
