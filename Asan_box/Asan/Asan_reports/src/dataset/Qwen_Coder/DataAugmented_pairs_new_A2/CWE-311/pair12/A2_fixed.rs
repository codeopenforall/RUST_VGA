use std::sync::Arc;
use std::thread;

struct Config {
    sensitive: String,
}

impl Config {
    fn new() -> Self {
        Config { sensitive: String::from("super_secret_password") }
    }
}

fn load_config() -> Config {
    Config::new()
}

fn basic_encrypt(text: &str, key: u8) -> String {
    text.bytes()
        .map(|b| (b ^ key) as char)
        .collect()
}

fn send(data: &str) -> Result<(), &'static str> {
    println!("Transmitting data: {:?}", data);
    if data.contains("secret") {
        Err("Transmission error: unencrypted sensitive data detected")
    } else {
        Ok(())
    }
}

fn run() -> Result<(), &'static str> {
    let config = Arc::new(load_config());
    let shared_config = Arc::clone(&config);
    let key: u8 = 0xAA; 
    let handle = thread::spawn(move || {
        let encrypted = basic_encrypt(&shared_config.sensitive, key);
        send(&encrypted)
    });
    handle.join().unwrap()
}

fn main() {
    match run() {
        Ok(_) => println!("Operation completed successfully."),
        Err(e) => println!("Operation failed: {}", e),
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
