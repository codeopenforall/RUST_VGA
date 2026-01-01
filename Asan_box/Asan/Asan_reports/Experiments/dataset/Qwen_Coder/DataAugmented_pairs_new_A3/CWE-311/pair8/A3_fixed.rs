use std::fs::File;
use std::io::{self, Write};

struct Config {
    api_key: String,
    db_password: String,
}

impl Config {
    fn new(api_key: String, db_password: String) -> Self {
        Config { api_key, db_password }
    }

    fn store(&self) -> io::Result<()> {
        let mut file = File::create("config.txt")?;
        let combined = format!("{}{}", self.api_key, self.db_password);
        let encrypted = xor_encrypt(combined.as_bytes(), 0xAA);
        file.write_all(&encrypted)?;
        Ok(())
    }
}

fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_sensitive_storage() {
        let _ = fs::remove_file("config.txt");
        let cfg = Config::new(
            "SuperSecretAPIKey".to_string(),
            "VerySecretDBPassword".to_string()
        );
        cfg.store().expect("Store operation failed");
        let stored = fs::read("config.txt").expect("Failed to read stored configuration");

        let plain = "SuperSecretAPIKeyVerySecretDBPassword".as_bytes().to_vec();
        
        assert_ne!(stored, plain, "Configuration stored in cleartext!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    // This oracle test is designed to detect if the stored configuration is left in cleartext.
    // It creates a configuration, triggers the storage method, reads the stored file,
    // and asserts that the stored data does not match the plain concatenation of secrets.
    #[test]
    fn test_sensitive_storage() {
        // Clean up any pre-existing file.
        let _ = fs::remove_file("config.txt");
        let cfg = Config::new(
            "SuperSecretAPIKey".to_string(),
            "VerySecretDBPassword".to_string()
        );
        cfg.store().expect("Store operation failed");
        let stored = fs::read("config.txt").expect("Failed to read stored configuration");

        let plain = "SuperSecretAPIKeyVerySecretDBPassword".as_bytes().to_vec();
        
        // For the insecure version, the stored data matches the plain text,
        // causing this assertion to fail. The secure version should produce encrypted output.
        assert_ne!(stored, plain, "Configuration stored in cleartext!");
    }
}
