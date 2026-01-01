struct Config {
    name: String,
    count: u32,
}

impl Config {
    pub fn new(name: String, count: u32) -> Self {
        Config { name, count }
    }

    pub fn valid(&self) -> bool {
        // Example validation logic
        !self.name.is_empty() && self.count > 0
    }
}

fn compute() -> bool {
    // Simulate some computation that might fail in the vulnerable version
    let cfg = Config::new("secure_config".to_owned(), 42);
    cfg.valid()
}

fn main() {
    let cfg = Config::new("secure_config".to_owned(), 42);
    let validity = compute();
    println!("Configuration valid: {}", validity);
}