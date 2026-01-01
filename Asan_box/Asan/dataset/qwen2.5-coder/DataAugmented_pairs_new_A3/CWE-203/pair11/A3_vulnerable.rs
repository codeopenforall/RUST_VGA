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