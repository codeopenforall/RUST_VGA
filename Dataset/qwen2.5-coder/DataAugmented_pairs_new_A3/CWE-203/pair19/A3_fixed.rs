use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Store {
    data: Mutex<HashMap<String, String>>,
}

impl Store {
    fn new(map: HashMap<String, String>) -> Arc<Self> {
        Arc::new(Store {
            data: Mutex::new(map),
        })
    }

    fn get_user_password(&self, user: &str) -> Option<String> {
        let guard = self.data.lock().unwrap();
        guard.get(user).cloned()
    }
}

pub struct Service {
    store: Arc<Store>,
}

impl Service {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "password123".to_string());
        Service {
            store: Store::new(map),
        }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<String, String> {
        match self.store.get_user_password(username) {
            Some(stored_pass) if stored_pass == password => {
                Ok(format!("User {} authenticated successfully.", username))
            },
            _ => Err("Authentication failed: Invalid username or password.".to_string()),
        }
    }
}