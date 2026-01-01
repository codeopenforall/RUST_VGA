use std::collections::HashMap;

pub struct LoginManager {
    users: HashMap<String, String>,
}

impl LoginManager {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("admin".to_string(), "password123".to_string());
        LoginManager { users }
    }

    pub fn validate(&self, username: &str, password: &str) -> Result<(), String> {
        if let Some(stored_password) = self.users.get(username) {
            if stored_password == password {
                Ok(())
            } else {
                return Err(format!("User '{}' exists but provided wrong password.", username));
            }
        } else {
            return Err("Username does not exist.".to_string());
        }
    }
}