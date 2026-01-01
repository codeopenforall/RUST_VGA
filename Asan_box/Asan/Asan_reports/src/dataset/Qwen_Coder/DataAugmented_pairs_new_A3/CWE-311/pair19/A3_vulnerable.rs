use std::sync::Arc;
use std::fs::OpenOptions;

struct Settings {
    secret: String,
}

fn store_data(data: &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("secret_config.txt")?;
    
    let ptr = data.as_bytes().as_ptr();
    let settings = Settings { secret: data.to_string() };
    let shared = Arc::new(settings);
    
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn main() {
    let settings = Settings { secret: "my_super_secret_password".to_string() };
    let copy = settings.clone();
    let _ = store_data(&copy.secret);
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
