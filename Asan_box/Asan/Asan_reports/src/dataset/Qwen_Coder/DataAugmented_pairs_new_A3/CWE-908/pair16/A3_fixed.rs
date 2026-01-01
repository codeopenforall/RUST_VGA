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

////////////////////////////////////////////////////////////
// Test Oracle Function
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // The test expects that compute() runs successfully and returns true in the fixed version.
    // In the vulnerable version, compute() will invoke undefined behavior (likely panicking).
    use super::compute;

    #[test]
    fn oracle() {
        let result = std::panic::catch_unwind(|| {
            compute()
        });
        // For the vulnerable version this should be an error (i.e. is_err() is true),
        // while for the fixed version the execution should succeed and return true.
        if result.is_err() {
            panic!("Test failed: The configuration processing panicked (vulnerability present)");
        }
        let valid = result.unwrap();
        assert!(valid, "Test failed: The configuration was not valid (unexpected value)");
    }
}
