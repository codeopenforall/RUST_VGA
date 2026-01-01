use std::sync::Arc;

struct Config {
    // Define any necessary fields for Config
}

impl Config {
    fn new() -> Self {
        // Initialize the Config struct
        Config { /* fields */ }
    }
}

fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

fn xor_decrypt(data: &[u8], key: u8) -> Vec<u8> {
    xor_encrypt(data, key)
}

fn transmit(config: &Arc<Config>) -> Vec<u8> {
    let secret_bytes = b"sensitive_data";
    let encrypted_data = xor_encrypt(secret_bytes, 0xAA);
    encrypted_data
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_transmission() {
        let config = Arc::new(Config::new());
        let transmitted = transmit(&config);
        // In the vulnerable implementation, the transmitted bytes would exactly equal
        // b"sensitive_data", causing the assertion to fail.
        // In the secure implementation, transmitted data is encrypted and must not match the plaintext.
        assert_ne!(transmitted, b"sensitive_data".to_vec(), "Sensitive data should not be transmitted in plaintext");
    }
}