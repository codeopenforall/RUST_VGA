fn secure_transform(data: &str) -> String {
    let xor_key: u8 = 0xAA;
    data.bytes()
        .map(|b| format!("{:02X}", b ^ xor_key))
        .collect()
}

fn run_app() -> String {
    let config_clone = Config {
        key: "super_secret".to_string(),
    };
    secure_transform(&config_clone.key)
}

struct Config {
    key: String,
}

///////////////////////////////////////////////////////////////////////////////
// Test Oracle: This test ensures that the encryption fix is effective.
// The oracle calls the shared function run_app() and verifies that the output differs
// from the original plaintext secret ("super_secret").
// For the vulnerable implementation the test would fail (output equals plaintext),
// whereas for the fixed implementation it will pass.
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Depending on which code is compiled (vulnerable or fixed),
    // run_app() either returns an unencrypted secret ("super_secret")
    // or an encrypted string.
    
    // Import the application run function.
    use super::run_app;

    #[test]
    fn encryption_test() {
        let transmitted = run_app();
        // The expected clear text value.
        let clear_text = "super_secret";
        // For fixed code, the transmitted value must not equal the clear text.
        // For vulnerable code, this assertion fails (i.e. the test will fail).
        assert_ne!(transmitted, clear_text, "Sensitive secret is transmitted unencrypted");
    }
}
