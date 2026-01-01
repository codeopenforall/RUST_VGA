use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

fn encrypt(data: &[u8]) -> Vec<u8> {
    let key: u8 = 0xAA; 
    data.iter().map(|b| b ^ key).collect()
}

struct Config {
    secret: String,
}

impl Config {
    fn new(secret: String) -> Self {
        Config { secret }
    }
}

fn process_config(conf: Arc<Mutex<Config>>) {
    let conf_copy = Arc::clone(&conf);
    let handler = thread::spawn(move || {
        let data = conf_copy.lock().unwrap();
        let raw = data.secret.as_bytes();
        // Introduce vulnerability by writing raw data instead of encrypted data
        let mut file = File::create("output.txt")
            .expect("Unable to create output file");
        file.write_all(raw)
            .expect("Failed to write sensitive data to file");
    });
    handler.join().unwrap();
}

fn main() {
    let config = Arc::new(Mutex::new(Config::new("SensitiveSecret".to_string())));
    process_config(config);
}