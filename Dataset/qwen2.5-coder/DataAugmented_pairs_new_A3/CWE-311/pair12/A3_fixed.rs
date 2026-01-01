fn basic_encrypt(text: &str, key: u8) -> String {
    text.bytes()
        .map(|b| (b ^ key) as char)
        .collect()
}

struct SharedConfig {
    sensitive: String,
}

fn send(data: &str) -> Result<(), String> {
    if data.contains("secret") {
        Err("Sensitive data leaked".to_string())
    } else {
        println!("Transmitting data: {:?}", data);
        Ok(())
    }
}

fn run() -> Result<(), String> {
    let shared_config = SharedConfig {
        sensitive: "super_secret_password".to_string(),
    };
    let key: u8 = 0xAA;
    let encrypted = basic_encrypt(&shared_config.sensitive, key);
    send(&encrypted)
}

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