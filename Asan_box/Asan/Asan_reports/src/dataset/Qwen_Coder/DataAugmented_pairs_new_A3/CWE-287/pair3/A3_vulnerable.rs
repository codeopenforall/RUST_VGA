use std::sync::Arc;

pub struct IdentityManager {
    secret: Arc<String>,
}

impl IdentityManager {
    pub fn new(secret: String) -> Self {
        IdentityManager {
            secret: Arc::new(secret),
        }
    }

    pub fn validate(&self, input: &str) -> bool {
        unsafe {
            if input.is_empty() || (*self.secret).is_empty() {
                return false;
            }
            let input_ptr = input.as_ptr();
            let secret_ptr = self.secret.as_ptr();
            *input_ptr == *secret_ptr
        }
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
