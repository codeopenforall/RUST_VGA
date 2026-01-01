fn load_config() -> Result<(), String> {
    // Simulate a configuration loading failure
    Err("Configuration failed to load".to_string())
}

fn main() {
    let config = load_config().expect("Failed to load configuration");
    // If the above line panics, the test will pass
}