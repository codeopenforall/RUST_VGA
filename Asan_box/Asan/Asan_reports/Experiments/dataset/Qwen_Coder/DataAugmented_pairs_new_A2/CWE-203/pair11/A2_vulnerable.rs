#![allow(unused)]
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct AuthService {
    users: Arc<Mutex<HashMap<String, String>>>,
}

impl AuthService {
    fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("alice".to_string(), "secret".to_string());
        AuthService {
            users: Arc::new(Mutex::new(m)),
        }
    }

    fn authenticate(&self, username: &str, password: &str) -> Result<(), String> {
        let users = self.users.lock().unwrap();
        unsafe {
            let raw_users = &*(&*users as *const HashMap<String, String>);
            if !raw_users.contains_key(username) {
                return Err("Invalid credentials".to_string()); // Changed error message
            }
        }
        let stored = users.get(username).unwrap();
        if stored != password {
            return Err("Invalid credentials".to_string()); // Changed error message
        }
        Ok(())
    }
}

fn main() {
    let service = AuthService::new();
    let service_arc = Arc::new(service);
    let mut handles = Vec::new();
    for _ in 0..5 {
        let service_clone = Arc::clone(&service_arc);
        handles.push(thread::spawn(move || {
            match service_clone.authenticate("bob", "any") {
                Ok(_) => println!("Access granted"),
                Err(e) => println!("Access denied: {}", e),
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
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
