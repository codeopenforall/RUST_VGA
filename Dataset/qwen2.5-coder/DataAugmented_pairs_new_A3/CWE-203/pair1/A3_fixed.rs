use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Account {
    name: String,
    secret: u64,
}

struct Engine {
    data: Arc<Mutex<HashMap<String, Account>>>,
}

impl Engine {
    fn new() -> Self {
        Engine {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn verify(&self, username: &str, pwd: &str) -> Result<(), String> {
        let db = self.data.lock().unwrap();
        let account = db.get(username);

        unsafe {
            let computed = pwd.bytes().fold(0u64, |accum, b| {
                accum.wrapping_mul(31).wrapping_add(b as u64)
            });
            let expected = account.as_ref().map(|acc| acc.secret).unwrap_or(0);
            if computed != expected {
                return Err(String::from("Authentication failed"));
            }
        }
        Ok(())
    }
}