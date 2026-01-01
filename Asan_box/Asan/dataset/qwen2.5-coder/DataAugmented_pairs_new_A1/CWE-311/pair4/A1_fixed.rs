use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    api_key: String,
}

impl Config {
    fn new(key: &str) -> Self {
        Self {
            api_key: key.to_owned(),
        }
    }

    fn transmit(&self) -> Vec<u8> {
        let ptr = self.api_key.as_ptr();
        let len = self.api_key.len();
        let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
        let mut encrypted = Vec::with_capacity(len);
        for &byte in bytes {
            encrypted.push(byte ^ 0xAA); // XOR with a fixed key
        }
        encrypted
    }
}

fn main() {
    let config = Arc::new(Config::new("SecretPassword123"));
    let shared = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];
    for _ in 0..4 {
        let config_clone = Arc::clone(&config);
        let shared_clone = Arc::clone(&shared);
        let handler = thread::spawn(move || {
            let transmitted = config_clone.transmit();
            let mut data = shared_clone.lock().unwrap();
            data.extend(transmitted);
        });
        threads.push(handler);
    }
    for handler in threads {
        handler.join().unwrap();
    }
    let result = shared.lock().unwrap();
    println!("Transmitted data: {}", String::from_utf8_lossy(&result));
}