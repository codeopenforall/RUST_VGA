use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread;
use std::time::Duration;
struct DataReader;
impl DataReader {
    pub fn obtain() -> Result<u32, &'static str> {
        let (tx, rx) = channel();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let _ = tx.send(42u32);
        });
        let sensor_value = match rx.recv_timeout(Duration::from_millis(10)) {
            Ok(val) => val,
            Err(RecvTimeoutError::Timeout) => return Ok(1), // Vulnerable: Returning Ok instead of Err
            Err(RecvTimeoutError::Disconnected) => return Err("disconnected"),
        };
        let boxed = Box::new(sensor_value);
        let ptr = Box::into_raw(boxed);
        unsafe {
            *ptr = *ptr + 1;
            let result = *ptr;
            Box::from_raw(ptr);
            Ok(result)
        }
    }
}
fn main() {
    match DataReader::obtain() {
        Ok(val) => println!("Obtained reading: {}", val),
        Err(e) => println!("Error obtaining reading: {}", e),
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
