use std::time::Duration;

struct SharedConfig {
    sensitive: String,
}

fn send(data: &str) -> Result<(), String> {
    if data.contains("secret") {
        Err("Sensitive data leaked".to_string())
    } else {
        Ok(())
    }
}

fn process(data: &[u8], shared_config: &SharedConfig) -> Result<(), String> {
    unsafe {
        let ptr = data.as_ptr();
        println!("Transmitting from pointer: {:?}", ptr);
        let leaked = std::slice::from_raw_parts(ptr, data.len());
        println!("Logged Data (vulnerable): {:?}", leaked);
    }

    unsafe {
        let ptr = shared_config.sensitive.as_ptr();
        let slice = std::slice::from_raw_parts(ptr, shared_config.sensitive.len());
        let extracted = String::from_utf8_lossy(slice);
        send(&extracted)
    }
}

fn run() -> Result<(), String> {
    let shared_config = SharedConfig {
        sensitive: "super_secret_password".to_string(),
    };
    let data = b"some_data";
    process(data, &shared_config)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

/*
This test oracle validates the secure transmission behavior.

For the vulnerable version:
 - The process function will return an error since the sensitive string "super_secret_password"
   remains in cleartext and triggers the error condition in the send function.
 
For the fixed version:
 - The process function encrypts the sensitive data, ensuring it does not contain the substring "secret".
   Hence, the send function should complete successfully and return Ok.

The test below asserts that the result from run() (acting as process) is Ok.
*/

#[cfg(test)]
mod tests {
    // Import all items from the parent module.
    use super::*;

    #[test]
    fn test_secure_transmission() {
        // run() should yield Ok() in the secure (fixed) version,
        // and fail (i.e. yield Err) in the vulnerable version.
        let result = run();
        assert!(result.is_ok(), "Test failed: Transmission did not secure the sensitive data.");
    }
}
