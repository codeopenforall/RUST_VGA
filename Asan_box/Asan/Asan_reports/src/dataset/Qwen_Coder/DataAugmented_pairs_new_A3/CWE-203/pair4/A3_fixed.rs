use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct AuthService {
    users: Arc<Mutex<HashMap<String, String>>>,
}

impl AuthService {
    pub fn new() -> Self {
        let users: HashMap<String, String> = [
            ("alice".to_string(), "password123".to_string()),
            ("bob".to_string(), "securepass".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        AuthService {
            users: Arc::new(Mutex::new(users)),
        }
    }

    pub fn login_auth(&self, username: &str, password: &str) -> Result<(), &'static str> {
        let users_clone = self.users.clone();
        let _guard = users_clone.lock().unwrap();
        let valid = {
            if let Some(correct) = _guard.get(username) {
                correct == password
            } else {
                false
            }
        };
        if valid {
            Ok(())
        } else {
            Err("Invalid credentials")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AuthService;

    // This test oracle function checks the authentication response. 
    // It should fail (i.e. produce a discrepancy in error message) for the vulnerable version 
    // by expecting a generic error message "Invalid credentials" while the vulnerable code returns
    // either "Invalid password" or "User not found". For the fixed version, the test passes.
    #[test]
    fn test_auth_error_message() {
        let service = AuthService::new();
        let result = service.login_auth("alice", "wrongpassword");
        assert!(result.is_err());
        // The expected error message in the fixed version is "Invalid credentials".
        // Vulnerable code exposes "Invalid password", causing the test to fail if not fixed.
        assert_eq!(result.unwrap_err(), "Invalid credentials");
    }
}
