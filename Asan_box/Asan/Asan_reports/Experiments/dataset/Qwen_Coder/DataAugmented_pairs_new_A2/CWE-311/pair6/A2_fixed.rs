use std::sync::Arc;
use std::thread;

struct Config {
    secret: String,
}

impl Config {
    fn new() -> Self {
        Self {
            secret: "sensitive_data".to_string(),
        }
    }
}

fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

fn xor_decrypt(data: &[u8], key: u8) -> Vec<u8> {
    xor_encrypt(data, key)
}

fn transmit(config: &Arc<Config>) -> Vec<u8> {
    let secret_ptr = config.secret.as_ptr();
    let secret_len = config.secret.len();
    let secret_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(secret_ptr, secret_len)
    };
    xor_encrypt(secret_bytes, 0xAA)
}

fn main() {
    let config = Arc::new(Config::new());
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        let encrypted_data = transmit(&config_clone);
        let decrypted = xor_decrypt(&encrypted_data, 0xAA);
        println!("Decrypted data: {:?}", String::from_utf8_lossy(&decrypted));
    });
    handle.join().unwrap();
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
