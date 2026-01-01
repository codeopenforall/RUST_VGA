#![allow(unused_imports)]
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

const STATIC_KEY: u8 = 0xAA;

struct Config {
    secret: String,
}

impl Config {
    fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
}

fn process_data(shared: Arc<Mutex<Config>>) {
    let clone = Arc::clone(&shared);
    thread::spawn(move || {
        unsafe {
            let secret_ptr = clone.lock().unwrap().secret.as_ptr();
            let secret_len = clone.lock().unwrap().secret.len();
            let slice = std::slice::from_raw_parts(secret_ptr, secret_len);
            let _leak = String::from_utf8_lossy(slice);
        }
    })
    .join()
    .expect("Thread panicked");
}

fn encrypt_secret(secret: &str) -> Vec<u8> {
    secret.bytes().map(|b| b ^ STATIC_KEY).collect()
}

fn store_config(data: &Config) {
    let mut file = File::create("secret.txt").expect("Failed to create file");
    let encrypted_secret = encrypt_secret(&data.secret);
    unsafe {
        let ptr = encrypted_secret.as_ptr();
        let len = encrypted_secret.len();
        let buf = std::slice::from_raw_parts(ptr, len);
        file.write_all(buf).expect("Failed to write file");
    }
}

fn main() {
    let config = Config::new("SuperSecretPassword123!");
    let shared_config = Arc::new(Mutex::new(config));
    process_data(Arc::clone(&shared_config));
    let guard = shared_config.lock().unwrap();
    store_config(&guard);
}