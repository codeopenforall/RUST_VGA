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

fn transmit(config: &Arc<Config>) -> Vec<u8> {
    let secret_ptr = config.secret.as_ptr();
    let secret_len = config.secret.len();
    let secret_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(secret_ptr, secret_len)
    };
    // Simulate encryption by adding a dummy byte to the beginning of the secret
    let mut encrypted_bytes = vec![0u8];
    encrypted_bytes.extend_from_slice(secret_bytes);
    encrypted_bytes
}

fn main() {
    let config = Arc::new(Config::new());
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        let data = transmit(&config_clone);
        println!("Transmitted data: {:?}", String::from_utf8_lossy(&data));
    });
    handle.join().unwrap();
}