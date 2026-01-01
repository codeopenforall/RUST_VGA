struct Data {
    secret_ptr: *const u8,
    secret_len: usize,
}

impl Data {
    fn new(secret: &str, id: usize) -> Self {
        Data {
            secret_ptr: secret.as_ptr(),
            secret_len: secret.len(),
        }
    }
}

fn process_data(data: &Data, trigger: bool) -> Result<(), String> {
    if trigger {
        let _slice = unsafe { std::slice::from_raw_parts(data.secret_ptr, data.secret_len) };
        return Err("Processing failed due to an internal error.".to_string());
    }
    Ok(())
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
