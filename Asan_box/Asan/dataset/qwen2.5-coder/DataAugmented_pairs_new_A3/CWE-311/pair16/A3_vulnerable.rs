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