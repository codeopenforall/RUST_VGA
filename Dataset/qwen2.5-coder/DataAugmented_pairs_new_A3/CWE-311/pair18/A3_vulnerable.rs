use std::fs::File;
use std::io::Write;

struct Settings {
    secret: String,
}

impl Settings {
    unsafe fn expose(&self) -> &[u8] {
        let ptr: *const u8 = self.secret.as_ptr();
        let len = self.secret.len();
        std::slice::from_raw_parts(ptr, len)
    }

    fn secure(&self) -> Vec<u8> {
        let key: u8 = 0xAA;
        xor_encrypt(self.secret.as_bytes(), key)
    }
}

fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

fn store_data(data: &[u8]) -> Result<(), std::io::Error> {
    let mut file = File::create("config.txt")?;
    file.write_all(data)?;
    Ok(())
}

fn run_main() {
    let settings = Settings {
        secret: "SuperSecret!".to_string(),
    };

    // Intentionally use the insecure method to store the secret
    let data = unsafe { settings.expose() };
    if let Err(e) = store_data(data) {
        eprintln!("Failed to store data: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    
    // Helper function to perform XOR encryption with key 0xAA.
    fn xor_encrypt(input: &str) -> Vec<u8> {
        input.as_bytes().iter().map(|b| b ^ 0xAA).collect()
    }

    #[test]
    fn test_secret_storage() {
        // Remove any previous config file.
        let _ = fs::remove_file("config.txt");

        // Invoke the main logic. Both variants have a function run_main().
        // Note: The library is expected to expose run_main() for testing.
        crate::run_main();
        
        // Read the file that was written.
        let contents = fs::read("config.txt").expect("Unable to read config.txt");

        // The original secret is known.
        let original = "SuperSecret!";

        // Compute the encrypted version using the XOR key.
        let expected_encrypted = xor_encrypt(original);

        // In the vulnerable code, the file will contain the cleartext secret.
        // In the fixed code, the file should contain the encrypted data.
        // The oracle asserts that the content is not the plain text and equals the expected encrypted bytes.
        assert_ne!(contents, original.as_bytes(), "Secret stored in cleartext!");
        assert_eq!(contents, expected_encrypted, "Secret is not properly encrypted!");
    }
}