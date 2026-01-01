pub struct IdentityManager {
    secret: String,
}

impl IdentityManager {
    pub fn new(secret: String) -> Self {
        IdentityManager { secret }
    }

    pub fn validate(&self, input: &str) -> bool {
        if input.len() != self.secret.len() {
            return false;
        }
        let input_bytes = input.as_bytes();
        let secret_bytes = self.secret.as_bytes();
        let mut result = 0u8;
        for (a, b) in input_bytes.iter().zip(secret_bytes.iter()) {
            result |= a ^ b;
        }
        result == 0
    }
}

#[cfg(test)]
mod tests {
    use super::IdentityManager;
    use std::sync::Arc;

    #[test]
    fn test_authentication() {
        // Setup IdentityManager with a secure secret.
        let manager = IdentityManager::new("S3cr3t".to_string());
        // A malicious token that shares the first character 'S' but is not the valid secret.
        // With proper authentication, this should be rejected (i.e., validate returns false).
        let malicious_token = "S3abcd";
        let result = manager.validate(malicious_token);
        assert!(!result, "Authentication bypass vulnerability detected.");
    }
}
