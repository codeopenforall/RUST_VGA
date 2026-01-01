use std::collections::HashMap;

struct Repository {
    raw_map: HashMap<String, String>,
}

impl Repository {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("bob".to_string(), "password123".to_string());
        Repository { raw_map: map }
    }

    pub fn check(&self, username: &str, password: &str) -> Result<(), &str> {
        match self.raw_map.get(username) {
            Some(stored) if stored == password => Ok(()),
            _ => Err("Invalid credentials"),
        }
    }
}