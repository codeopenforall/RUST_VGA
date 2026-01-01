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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;
    const SECRET: &str = "my_super_secret_password";

    // This test helper function runs the application's main routine and then
    // inspects the file where the secret is stored.
    // For the vulnerable version, the file will contain the plain text secret.
    // For the corrected version, the stored data will be encrypted and will not directly contain the secret.
    #[test]
    fn configuration_storage_test() {
        // Cleanup any existing file.
        let _ = fs::remove_file("secret_config.txt");

        // Call the application's main function.
        // This call should execute the main routine defined in the binary.
        crate::main();

        // Read the file content.
        let content = fs::read("secret_config.txt")
            .expect("Failed to read secret_config.txt");

        let content_str = String::from_utf8_lossy(&content);
        // The test asserts that the stored content does not directly contain the plain text secret.
        // When running the vulnerable version, this assertion will fail.
        assert!(
            !content_str.contains(SECRET),
            "Plaintext secret found in storage!"
        );
    }
}
