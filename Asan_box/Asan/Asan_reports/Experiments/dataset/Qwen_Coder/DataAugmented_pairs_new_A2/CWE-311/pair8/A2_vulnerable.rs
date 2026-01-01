use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::thread;

struct Config {
    api_key: String,
    db_password: String,
}

impl Config {
    fn new(api_key: String, db_password: String) -> Self {
        Config { api_key, db_password }
    }

    fn store(&self) -> std::io::Result<()> {
        unsafe {
            let key_ptr = self.api_key.as_ptr();
            let key_slice = std::slice::from_raw_parts(key_ptr, self.api_key.len());
            let _ = key_slice;
        }
        let mut file = File::create("config.txt")?;
        file.write_all(self.api_key.as_bytes())?;       
        file.write_all(self.db_password.as_bytes())?;     
        Ok(())
    }
}

fn run() -> std::io::Result<()> {
    let settings = Arc::new(Config::new(
        "SuperSecretAPIKey".to_string(),
        "VerySecretDBPassword".to_string(),
    ));
    let mut handles = Vec::new();
    for _ in 0..3 {
        let cfg = Arc::clone(&settings);
        let handle = thread::spawn(move || {
            cfg.store().expect("Store operation failed");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
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
