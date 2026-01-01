use std::fs::File;
use std::io::{self, Write};

fn encrypt(data: &str, key: u8) -> Vec<u8> {
    data.bytes().map(|b| b ^ key).collect()
}

fn decrypt(data: &[u8], key: u8) -> String {
    let decrypted: Vec<u8> = data.iter().map(|b| b ^ key).collect();
    String::from_utf8_lossy(&decrypted).into_owned()
}

fn main() -> io::Result<()> {
    let plain_secret = "my_super_secret";
    let key: u8 = 0xAA;
    let encrypted_data = encrypt(plain_secret, key);

    let mut file = File::create("secret.txt")?;
    file.write_all(&encrypted_data)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;

    // This test oracle checks that the sensitive secret is not stored in clear text.
    // For the vulnerable version, the file "secret.txt" will contain the plaintext
    // "my_super_secret", so the assertion will fail. In the fixed version, the file
    // contents are encrypted, so they will not match the plaintext.
    #[test]
    fn test_sensitive_storage_encryption() {
        // Remove any existing file.
        let _ = fs::remove_file("secret.txt");

        // Execute the program's main function.
        crate::main();

        // Read the contents of the file.
        let data = fs::read("secret.txt").expect("File 'secret.txt' should exist");
        let expected = b"my_super_secret";

        // The test asserts that the file content should NOT equal the sensitive plaintext.
        assert_ne!(data, expected, "Sensitive data is stored in clear text!");
    }
}
