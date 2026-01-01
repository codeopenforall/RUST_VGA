use std::thread;

pub struct Config {
    pub secret: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            secret: "supersecret".to_string(),
        }
    }
}

fn xor_cipher(data: &str, key: u8) -> String {
    data.bytes()
        .map(|b| (b ^ key) as char)
        .collect()
}

pub fn process_output() -> String {
    let cfg = Config::new();
    // Introduce the vulnerability by removing encryption
    let encrypted = cfg.secret.clone(); // Use the secret directly instead of encrypting
    // Simulate a concurrent environment. The thread now handles already-encrypted data.
    let handle = thread::spawn(move || encrypted);
    handle.join().unwrap()
}

fn main() {
    // The main function now transmits encrypted sensitive data.
    let output = process_output();
    println!("Transmitted data: {}", output);
}