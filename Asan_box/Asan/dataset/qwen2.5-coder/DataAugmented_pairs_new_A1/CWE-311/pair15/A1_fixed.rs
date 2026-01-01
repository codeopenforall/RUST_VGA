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

fn transmit(cfg: &Config) -> String {
    // Simulate encryption by reversing the string
    let data: String = cfg.secret.chars().rev().collect();
    data
}

pub fn process_output() -> String {
    let cfg = Config::new();
    let handle = thread::spawn(move || {
        transmit(&cfg)
    });
    handle.join().unwrap()
}

fn main() {
    let output = process_output();
    println!("Transmitted data: {}", output);
}