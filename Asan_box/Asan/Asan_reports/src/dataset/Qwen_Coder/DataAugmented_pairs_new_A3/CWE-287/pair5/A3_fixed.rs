pub struct AuthManager {
    expected: String,
}

impl AuthManager {
    pub fn new(expected: &str) -> Self {
        AuthManager {
            expected: expected.to_string(),
        }
    }

    pub fn validate(&self, user_token: &str) -> bool {
        if self.expected.len() != user_token.len() {
            return false;
        }
        for i in 0..self.expected.len() {
            if self.expected.chars().nth(i) != user_token.chars().nth(i) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    // Bring the AuthManager from the current module into scope.
    // In practice, these tests would be compiled against either the vulnerable or fixed version.
    use super::AuthManager;

    #[test]
    fn test_authentication_token() {
        // Instantiate the authentication manager with the expected token.
        let manager = AuthManager::new("SuperSecret");
        // A malicious token that contains the correct prefix plus extra characters.
        let malicious_token = "SuperSecretExtra";
        // In the fixed implementation, this should be rejected.
        // In the vulnerable implementation, this would erroneously be accepted.
        let result = manager.validate(malicious_token);
        assert!(
            !result,
            "Authentication should fail when extra characters are appended to the expected token."
        );
    }
}
