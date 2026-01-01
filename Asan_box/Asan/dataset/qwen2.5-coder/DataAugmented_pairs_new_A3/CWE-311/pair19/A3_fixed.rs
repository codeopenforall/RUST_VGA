use std::fs::File;
use std::io::{self, Write};
use std::sync::Arc;

fn xor_transform(input: &str, key: u8) -> Vec<u8> {
    input.bytes().map(|b| b ^ key).collect()
}

fn store_data(data: &[u8]) -> io::Result<()> {
    let mut file = File::create("secret_config.txt")?;
    file.write_all(data)?;
    Ok(())
}

struct Settings {
    secret: String,
}

fn main() {
    let settings = Settings {
        secret: "my_super_secret_password".to_string(),
    };
    let encryption_key = 0xAA;
    let encrypted = xor_transform(&settings.secret, encryption_key);
    let shared = Arc::new(encrypted);
    let copy = Arc::clone(&shared);
    let _ = store_data(&copy);
}