pub struct LoginManager {
    // Assume some internal state for managing users
}

impl LoginManager {
    pub fn new() -> Self {
        LoginManager {
            // Initialize with some users if needed
        }
    }

    pub fn validate(&self, username: &str, password: &str) -> Result<(), String> {
        // Simulate user existence check
        if self.user_exists(username) {
            // Simulate password check
            if self.correct_password(username, password) {
                Ok(())
            } else {
                Err("Invalid credentials.".to_string())
            }
        } else {
            Err("Invalid credentials.".to_string())
        }
    }

    fn user_exists(&self, username: &str) -> bool {
        // Placeholder for actual user existence check
        username == "admin"
    }

    fn correct_password(&self, username: &str, password: &str) -> bool {
        // Placeholder for actual password check
        username == "admin" && password == "correctpass"
    }
}

//////////////////////////////////////////////
// Test Oracle for Authentication Module
//
// This oracle verifies that the authentication module 
// returns a uniform error message ("Invalid credentials.") 
// for all failed login attempts. When executed against the 
// vulnerable version, the test will fail due to error message 
// discrepancies, whereas the corrected version will pass.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::LoginManager;

    #[test]
    fn test_authentication_uniform_error() {
        let manager = LoginManager::new();
        let err_existing = manager.validate("admin", "wrongpass").unwrap_err();
        let err_nonexisting = manager.validate("nonexistent", "anything").unwrap_err();
        // Assert that both error messages are exactly the same.
        assert_eq!(err_existing, "Invalid credentials.", "Error message for existing user is not uniform.");
        assert_eq!(err_nonexisting, "Invalid credentials.", "Error message for non-existing user is not uniform.");
    }
}
