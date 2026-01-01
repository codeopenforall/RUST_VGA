use std::collections::HashMap;

pub struct Repository {
    raw_map: HashMap<String, String>,
}

impl Repository {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("bob".to_string(), "password123".to_string());
        Repository { raw_map: map }
    }

    pub fn check(&self, username: &str, password: &str) -> Result<(), &str> {
        if let Some(stored) = self.raw_map.get(username) {
            if stored == password {
                Ok(())
            } else {
                return Err("Password does not match");
            }
        } else {
            return Err("Username does not exist");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_auth_message_consistency() {
        // Set up the repository with predefined credentials.
        let repo = Repository::new();
        
        // Attempt authentication with a valid username but incorrect password.
        let error_existing = repo.check("bob", "wrongpass").err().unwrap();
        
        // Attempt authentication with a non-existent username.
        let error_nonexisting = repo.check("mallory", "any").err().unwrap();
        
        // For the secure version, both errors must be identical to avoid information disclosure.
        assert_eq!(error_existing, "Invalid credentials", "Error message for existing account did not match the expected generic message");
        assert_eq!(error_nonexisting, "Invalid credentials", "Error message for non-existing account did not match the expected generic message");
    }
}
