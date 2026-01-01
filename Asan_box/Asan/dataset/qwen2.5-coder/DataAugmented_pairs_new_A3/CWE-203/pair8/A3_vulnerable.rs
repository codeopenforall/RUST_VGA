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