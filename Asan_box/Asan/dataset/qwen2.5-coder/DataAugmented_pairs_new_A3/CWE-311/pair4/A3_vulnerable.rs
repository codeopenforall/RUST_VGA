use std::sync::{Arc, Mutex};

struct Config {
    api_key: String,
}

impl Config {
    fn new(api_key: &str) -> Self {
        Config {
            api_key: api_key.to_string(),
        }
    }

    pub fn transmit(&self) -> Vec<u8> {
        unsafe {
            let ptr = self.api_key.as_ptr();
            let len = self.api_key.len();
            let bytes = std::slice::from_raw_parts(ptr, len);
            bytes.to_vec()
        }
    }
}

fn main() {
    let config = Config::new("SecretPassword123");
    let transmitted = config.transmit();
    println!("Transmitted data: {}", String::from_utf8_lossy(&transmitted));
}