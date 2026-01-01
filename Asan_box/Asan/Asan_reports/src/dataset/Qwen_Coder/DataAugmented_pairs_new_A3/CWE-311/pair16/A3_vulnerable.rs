struct Service {
    secret: String,
}

impl Service {
    fn new(secret: String) -> Self {
        Service { secret }
    }

    fn process(&self) -> String {
        let copied = self.secret.as_bytes().to_vec();
        String::from_utf8_lossy(&copied).into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_processing() {
        let input = "test_secret";
        let service = Service::new(input.to_string());
        let processed = service.process();
        assert_ne!(processed, input, "Sensitive data is not encrypted!");
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
