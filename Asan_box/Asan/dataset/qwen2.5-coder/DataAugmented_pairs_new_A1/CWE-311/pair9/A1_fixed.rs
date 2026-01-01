use std::sync::Arc;
use std::thread;
use std::fmt::Write;

struct Config {
    secret: String,
}

impl Config {
    fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }

    fn get_bytes(&self) -> Vec<u8> {
        self.secret.as_bytes().to_vec()
    }
}

pub fn transmit_secret() -> String {
    let config = Config::new("supersecret");
    let arc_config = Arc::new(config);
    let arc_clone = Arc::clone(&arc_config);
    let handle = thread::spawn(move || {
        let bytes = arc_clone.get_bytes();
        let mut encoded = String::new();
        for byte in bytes {
            write!(&mut encoded, "{:02x}", byte).unwrap();
        }
        encoded
    });
    handle.join().unwrap()
}

fn main() {
    let transmitted = transmit_secret();
    println!("Data transmitted: {}", transmitted);
}