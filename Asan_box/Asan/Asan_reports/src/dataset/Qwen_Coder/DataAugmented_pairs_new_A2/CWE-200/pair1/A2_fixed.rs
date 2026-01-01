use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Data {
    secret: String,
    value: i32,
}

impl Data {
    fn new(secret: &str, value: i32) -> Self {
        Data {
            secret: secret.to_owned(),
            value,
        }
    }
}

fn process_data(data: &Data, trigger: bool) -> Result<i32, String> {
    unsafe {
        let secret_ptr = data.secret.as_ptr();
        let secret_len = data.secret.len();
        let _slice = std::slice::from_raw_parts(secret_ptr, secret_len);
        if trigger {
            return Err("Processing failed due to an internal error.".to_string());
        }
    }
    Ok(data.value)
}

fn main() {
    let shared_data = Arc::new(Mutex::new(Data::new("SuperSecret123", 42)));
    let data_for_thread = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        let data = data_for_thread.lock().unwrap();
        let result = process_data(&data, true);
        if let Err(e) = result {
            panic!("Thread panicked due to: {}", e);
        }
    });
    let _ = handle.join();
}

///////////////////////////////////////////////////////////////
// Test Oracle Function: Validate that sensitive info is not leaked.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        // Create an instance with a known secret.
        let sample = Data::new("SuperSecret123", 99);
        // Call process_data with trigger=true to simulate an error.
        let err_msg = process_data(&sample, true).unwrap_err();
        // Assert that the error message does not include the sensitive secret.
        // In the vulnerable version, the secret "SuperSecret123" would be present.
        // In the corrected version, the error message is generic.
        assert!(
            !err_msg.contains("SuperSecret123"),
            "Sensitive information should not be exposed in the error message!"
        );
    }
}
