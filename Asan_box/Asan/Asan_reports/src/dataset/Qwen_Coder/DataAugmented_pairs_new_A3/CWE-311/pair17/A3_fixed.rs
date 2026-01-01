const XOR_KEY: u8 = 0xAA;

fn xor_encrypt(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    unsafe {
        let ptr = input.as_ptr();
        for i in 0..input.len() {
            let byte = *ptr.add(i);
            output.push(byte ^ XOR_KEY);
        }
    }
    output
}

fn get_transmitted() -> Vec<u8> {
    let data = b"SuperSecret";
    let encrypted = xor_encrypt(data);
    encrypted
}

fn main() {
    let transmitted = get_transmitted();
    println!("Encrypted transmitted data: {:?}", transmitted);
}

/* Test Oracle Function:
   This test verifies that the transmitted data is not the cleartext "SuperSecret".
   It should fail for the vulnerable version and pass for the fixed version.
*/

#[cfg(test)]
mod tests {
    // Import the get_transmitted function from the module under test.
    // Adjust the module name as necessary if the code is organized into separate crates or modules.
    use super::get_transmitted;

    #[test]
    fn test_sensitive_data_encryption() {
        let transmitted = get_transmitted();
        // If the transmitted bytes, when interpreted as UTF-8, equal "SuperSecret", then
        // encryption was not applied.
        assert_ne!(
            String::from_utf8_lossy(&transmitted),
            "SuperSecret",
            "Sensitive data is transmitted unencrypted!"
        );
    }
}
