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