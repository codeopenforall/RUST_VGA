use std::fs::File;
use std::io::Write;
use std::ptr;

struct Settings {
    secret: String,
}

impl Settings {
    unsafe fn expose(&self) -> &[u8] {
        let ptr: *const u8 = self.secret.as_ptr();
        let len = self.secret.len();
        std::slice::from_raw_parts(ptr, len)
    }
}

fn store_data(data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create("config.txt")?;
    unsafe {
        let raw_data = data.as_ptr();
        let size = data.len();
        let slice = std::slice::from_raw_parts(raw_data, size);
        file.write_all(slice)?;
    }
    Ok(())
}

fn run_main() {
    let settings = Settings { secret: "SuperSecret!".to_string() };
    let data = unsafe { settings.expose() };
    if let Err(e) = store_data(data) {
        eprintln!("Error storing configuration: {}", e);
    }
}

fn main() {
    run_main();
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
