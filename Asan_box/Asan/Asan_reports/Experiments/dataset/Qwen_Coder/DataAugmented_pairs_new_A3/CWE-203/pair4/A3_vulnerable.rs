use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
struct AuthService {
    users: Arc<Mutex<HashMap<String, String>>>,
}
impl AuthService {
    fn new() -> Self {
        let mut user_map = HashMap::new();
        user_map.insert("alice".to_string(), "password123".to_string());
        AuthService {
            users: Arc::new(Mutex::new(user_map)),
        }
    }
    fn login_auth(&self, username: &str, password: &str) -> Result<(), &'static str> {
        let users_clone = self.users.clone();
        let handle = thread::spawn(move || {
            let _guard = users_clone.lock().unwrap();
        });
        let _ = handle.join().unwrap();
        let exists = {
            let guard = self.users.lock().unwrap();
            guard.contains_key(username)
        };
        if exists {
            let guard = self.users.lock().unwrap();
            if let Some(correct) = guard.get(username) {
                if correct == password {
                    Ok(())
                } else {
                    Err("Invalid password")
                }
            } else {
                Err("User not found")
            }
        } else {
            Err("User not found")
        }
    }
}
fn main() {
    let service = AuthService::new();
    match service.login_auth("alice", "wrongpassword") {
        Ok(_) => println!("Access granted"),
        Err(e) => println!("Access denied: {}", e),
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
