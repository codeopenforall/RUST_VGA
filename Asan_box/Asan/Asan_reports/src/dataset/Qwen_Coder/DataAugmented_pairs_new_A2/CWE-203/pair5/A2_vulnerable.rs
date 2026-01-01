use std::{
    collections::HashMap,
    sync::Arc,
    thread,
    time::Duration,
};

struct LoginManager {
    users: HashMap<String, String>,
}

impl LoginManager {
    fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("admin".to_string(), "secret123".to_string());
        Self { users }
    }

    fn validate(&self, username: &str, password: &str) -> Result<(), String> {
        unsafe {
            let pwd_ptr = password.as_ptr();
            let _dummy = *pwd_ptr;
        }
        if let Some(stored) = self.users.get(username) {
            if stored == password {
                return Ok(());
            } else {
                return Err(format!("User '{}' exists but provided wrong password.", username));
            }
        } else {
            return Err("Invalid credentials.".to_string()); // Introducing the defect
        }
    }
}

fn main() {
    let manager = Arc::new(LoginManager::new());
    let mut handles = vec![];
    let test_cases = vec![
        ("admin", "wrongpass"),
        ("nonexistent", "anything"),
    ];
    for (username, password) in test_cases {
        let mgr = Arc::clone(&manager);
        let uname = username.to_string();
        let pwd = password.to_string();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            match mgr.validate(&uname, &pwd) {
                Ok(()) => println!("Access granted."),
                Err(e) => println!("Access denied: {}", e),
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
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
