struct Service {
    secret: String,
}

impl Service {
    fn new(secret: String) -> Self {
        Service { secret }
    }

    fn xor_enc(data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|b| b ^ key).collect()
    }

    fn process(&self) -> String {
        let copied = self.secret.as_bytes();
        let encrypted = Service::xor_enc(copied, 0xAA);
        encrypted.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // The test oracle instantiates the service with a known secret,
    // calls the process method and asserts that the output does NOT match
    // the original input. This test should fail for the vulnerable version
    // (which returns the clear text secret) and pass for the fixed version.
    #[test]
    fn test_secret_processing() {
        let input = "test_secret";
        let service = Service::new(input.to_string());
        let processed = service.process();
        // The processed result should differ from the original secret.
        // In the vulnerable version, they would be equal.
        assert_ne!(processed, input, "Sensitive data is not encrypted!");
    }
}
