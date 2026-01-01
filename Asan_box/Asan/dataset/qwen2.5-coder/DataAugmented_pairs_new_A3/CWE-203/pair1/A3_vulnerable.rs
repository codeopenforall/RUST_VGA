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

        match account {
            None => {
                return Err(String::from("User does not exist"));
            }
            Some(acc) => {
                unsafe {
                    let computed = pwd.bytes().fold(0u64, |accum, b| {
                        accum.wrapping_mul(31).wrapping_add(b as u64)
                    });
                    if computed != acc.secret {
                        return Err(String::from("Invalid password"));
                    }
                }
                Ok(())
            }
        }
    }
}