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
        match users.get(username) {
            Some(stored_password) if stored_password == password => Ok(()),
            _ => Err("Invalid credentials".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // Test oracle: verifies that both an incorrect password for an existing user and
    // any attempt with a non-existing user produce the same error message.
    fn run_test<T: Fn(&str, &str) -> Result<(), String>>(auth_fn: T) {
        let err1 = auth_fn("alice", "wrong_password").unwrap_err();
        let err2 = auth_fn("bob", "whatever").unwrap_err();
        assert_eq!(err1, err2, "Error messages differ, potentially leaking user existence information");
    }

    #[test]
    fn test_authentication_oracle() {
        let system = Credentials::create();
        let auth_closure = |u: &str, p: &str| system.authenticate(u, p);
        run_test(auth_closure);
    }
}
