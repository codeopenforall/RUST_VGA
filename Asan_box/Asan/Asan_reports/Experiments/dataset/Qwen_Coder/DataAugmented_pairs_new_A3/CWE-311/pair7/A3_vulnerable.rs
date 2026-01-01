use std::fs::File;
use std::io::Write;

struct SecretData {
    secret: String,
}

impl SecretData {
    fn new(secret: &str) -> Self {
        SecretData {
            secret: secret.to_string(),
        }
    }
}

fn main() {
    let data = SecretData::new("SuperSecretPassword123!");
    let mut file = File::create("secret.txt").expect("Failed to create file");

    let slice = data.secret.as_bytes();
    let _leak = String::from_utf8_lossy(slice);

    unsafe {
        let ptr = data.secret.as_ptr();
        let len = data.secret.len();
        let buf = std::slice::from_raw_parts(ptr, len);
        file.write_all(buf).expect("Failed to write file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // The expected plain secret for comparison.
    const PLAIN_SECRET: &str = "SuperSecretPassword123!";
    // The static key used in the fixed implementation.
    const STATIC_KEY: u8 = 0xAA;

    // A helper function to perform XOR encryption (mirroring the one in the fixed code).
    fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|b| b ^ key).collect()
    }

    // Test function that asserts the stored file content is not the cleartext secret.
    #[test]
    fn test_secret_storage() {
        // Remove file if exists.
        let _ = std::fs::remove_file("secret.txt");

        // Run the main function from the implementation.
        // Note: In a real test harness, you'd separate binary and library code.
        main();

        // Read the file content.
        let content = fs::read("secret.txt").expect("Unable to read secret.txt");

        // For the vulnerable implementation, the file would contain the plain secret.
        // For the fixed implementation, it should contain the XOR encrypted data.
        if content == PLAIN_SECRET.as_bytes() {
            panic!("Test failed: Sensitive data stored in cleartext.");
        }

        // Additionally, we can check that decryption returns the original secret.
        let decrypted = xor_encrypt(&content, STATIC_KEY);
        assert_eq!(decrypted, PLAIN_SECRET.as_bytes(), "Decrypted data does not match original secret.");
    }
}
