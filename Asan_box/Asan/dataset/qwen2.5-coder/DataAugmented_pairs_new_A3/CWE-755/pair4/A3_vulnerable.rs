fn load_config() -> Result<String, String> {
    Err("Failed to load configuration".to_string())
}

fn main() {
    let config = load_config().unwrap_or_default();
    println!("Config: {}", config);
}