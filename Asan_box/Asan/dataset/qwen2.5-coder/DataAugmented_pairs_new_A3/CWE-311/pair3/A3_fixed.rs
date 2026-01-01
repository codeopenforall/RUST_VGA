fn secure_transform(data: &str) -> String {
    let xor_key: u8 = 0xAA;
    data.bytes()
        .map(|b| format!("{:02X}", b ^ xor_key))
        .collect()
}

fn run_app() -> String {
    let config_clone = Config {
        key: "super_secret".to_string(),
    };
    secure_transform(&config_clone.key)
}

struct Config {
    key: String,
}