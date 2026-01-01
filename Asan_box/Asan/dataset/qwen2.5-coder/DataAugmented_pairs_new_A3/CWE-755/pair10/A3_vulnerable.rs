use std::fs;

struct Handler {
    threshold: u32,
}

impl Handler {
    fn new() -> Self {
        let config_content = load_config("config.txt").unwrap_or_default(); 
        let threshold = config_content.trim().parse::<u32>().unwrap_or_default(); 
        Handler { threshold }
    }
}

fn load_config(filename: &str) -> Option<String> {
    fs::read_to_string(filename).ok()
}

fn main() {
    let instance = Handler::new();
    let _ = instance;
}