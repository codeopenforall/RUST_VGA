use std::fs::File;
use std::io::{self, Write};

struct Settings {
    secret: String,
}

impl Settings {
    fn new(secret: &str) -> Self {
        Settings {
            secret: secret.to_string(),
        }
    }

    fn secure(&self) -> Vec<u8> {
        let key: u8 = 0xAA;
        xor_encrypt(self.secret.as_bytes(), key)
    }
}

fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

fn store_data(data: &[u8]) -> io::Result<()> {
    let mut file = File::create("config.txt")?;
    file.write_all(data)?;
    Ok(())
}

fn run_main() {
    let settings = Settings::new("SuperSecret!");
    let encrypted_data = settings.secure();
    if let Err(e) = store_data(&encrypted_data) {
        eprintln!("Failed to store data: {}", e);
    }
}