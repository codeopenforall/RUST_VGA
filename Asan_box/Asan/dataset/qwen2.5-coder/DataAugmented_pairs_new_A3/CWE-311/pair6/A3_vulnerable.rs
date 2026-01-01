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