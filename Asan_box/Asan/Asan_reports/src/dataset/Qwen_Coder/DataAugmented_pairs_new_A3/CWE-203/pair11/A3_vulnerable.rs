use std::collections::HashMap;

pub struct AuthService {
    users: HashMap<String, String>,
}

impl AuthService {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("alice".to_string(), "password123".to_string());
        AuthService { users }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<(), String> {
        if !self.users.contains_key(username) {
            return Err("User does not exist".to_string());
        }
        let stored = self.users.get(username).unwrap();
        if stored != password {
            return Err("Incorrect password".to_string());
        }
        Ok(())
    }
}

//////////////////////////////////////////
// Test Oracle for Authentication Module
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test expects the error message to be uniform ("Invalid credentials")
    // If the implementation reveals different error messages (e.g., "User does not exist"),
    // the test will fail.
    #[test]
    fn test_authentication_error_message() {
        let service = AuthService::new();
        // Attempt authentication with a non-existent user.
        let err = service.authenticate("bob", "any").unwrap_err();
        assert_eq!(err, "Invalid credentials", "Error message must be uniform to prevent information leakage");
    }
}
