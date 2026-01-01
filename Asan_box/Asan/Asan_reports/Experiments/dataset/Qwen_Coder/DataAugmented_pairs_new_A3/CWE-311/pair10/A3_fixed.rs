struct Config {
    data: String,
}

impl Config {
    fn new(secret: &str) -> Self {
        Config {
            data: secret.to_string(),
        }
    }

    fn xor_transform(input: &str, key: u8) -> String {
        let transformed: Vec<u8> = input.bytes().map(|b| b ^ key).collect();
        transformed.iter().map(|b| format!("{:02x}", b)).collect()
    }

    fn send(&self) -> String {
        let key: u8 = 0xAA;
        Self::xor_transform(&self.data, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_check() {
        // Using a known secret value.
        let config = Config::new("MySecret");
        let transmitted = config.send();
        // The test asserts that the transmitted result must not equal the plain secret.
        // In the insecure variant, the output will be "MySecret", triggering the assertion failure.
        // In the corrected variant, the output is an encrypted hex string.
        assert_ne!(transmitted, "MySecret", "Sensitive data is transmitted in cleartext!");
    }
}
