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

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper to setup an engine with a preset account.
    fn setup_engine() -> Engine {
        let engine = Engine::new();
        {
            let mut db = engine.data.lock().unwrap();
            unsafe {
                let hash = "secret".bytes().fold(0u64, |accum, b| {
                    accum.wrapping_mul(31).wrapping_add(b as u64)
                });
                db.insert("alice".to_string(), Account { name: "alice".into(), secret: hash });
            }
        }
        engine
    }

    #[test]
    fn test_authentication_error_message() {
        let engine = setup_engine();
        // Attempt to authenticate with correct username but wrong password.
        let res = engine.verify("alice", "guess");
        // The fixed version should always return a generic error message.
        // Vulnerable version returns "Invalid password", which does not match.
        assert!(res.is_err());
        let err_msg = res.err().unwrap();
        assert_eq!(err_msg, "Authentication failed", "The error message should be generic to prevent information leaks");
    }
}
//////////////////////////////
// End of Test Oracle Function
//////////////////////////////
