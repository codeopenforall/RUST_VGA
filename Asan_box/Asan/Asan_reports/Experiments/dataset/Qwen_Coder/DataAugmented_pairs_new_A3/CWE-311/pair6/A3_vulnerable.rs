use std::sync::Arc;

struct Config {
    // Assume some configuration fields
}

impl Config {
    fn new() -> Self {
        Config {
            // Initialize with some default values
        }
    }
}

fn transmit(config: &Arc<Config>) -> Vec<u8> {
    // Simulate transmission of data
    b"sensitive_data".to_vec()
}

fn main() {
    let config = Arc::new(Config::new());
    let data = transmit(&config);
    println!("Transmitted data: {:?}", String::from_utf8_lossy(&data));
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
