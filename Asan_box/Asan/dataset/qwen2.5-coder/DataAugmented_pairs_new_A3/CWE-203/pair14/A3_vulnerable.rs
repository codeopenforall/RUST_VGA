use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Credentials {
    users: Arc<Mutex<HashMap<String, String>>>,
}

impl Credentials {
    pub fn create() -> Self {
        let mut users = HashMap::new();
        users.insert("alice".to_string(), "password123".to_string());
        Credentials {
            users: Arc::new(Mutex::new(users)),
        }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<(), String> {
        let users = self.users.lock().unwrap();
        if let Some(stored_password) = users.get(username) {
            if stored_password == password {
                Ok(())
            } else {
                return Err("Incorrect password for user".to_string());
            }
        } else {
            return Err("User not found".to_string());
        }
    }
}