struct Config {
    data: String,
}

impl Config {
    pub fn new(secret: &str) -> Self {
        Config {
            data: secret.to_string(),
        }
    }

    pub fn send(&self) -> String {
        unsafe {
            let raw_ptr = self.data.as_ptr();
            let len = self.data.len();
            let bytes = std::slice::from_raw_parts(raw_ptr, len);
            String::from_utf8_lossy(bytes).to_string()
        }
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
