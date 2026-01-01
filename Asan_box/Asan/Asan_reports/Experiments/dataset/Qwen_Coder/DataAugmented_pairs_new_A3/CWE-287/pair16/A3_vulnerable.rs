use std::sync::{Arc, Mutex};

pub struct AuthManager {
    token: String,
}

impl AuthManager {
    pub fn new() -> Self {
        AuthManager {
            token: "valid_token123".to_string(),
        }
    }

    pub fn verify(&self, user_token: &str) -> bool {
        unsafe {
            if user_token.is_empty() {
                return false;
            }
            let expected_ptr = self.token.as_ptr();
            let user_ptr = user_token.as_ptr();
            *expected_ptr == *user_ptr
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // A helper function to create an AuthManager for testing.
    fn create_manager() -> AuthManager {
        AuthManager::new()
    }

    #[test]
    fn test_authentication() {
        // In a secure implementation, the following should pass:
        //  - Providing the correct token "valid_token123" returns true.
        //  - Providing a short token "v" returns false.
        let manager = create_manager();

        // Correct token
        assert!(manager.verify("valid_token123"), "Valid token must pass authentication");
        // Malicious short token that only matches the first byte of the token.
        assert!(!manager.verify("v"), "Incorrect token must fail authentication");
    }
}
