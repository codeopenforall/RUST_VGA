use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Store {
    data: Mutex<HashMap<String, String>>,
}

impl Store {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "secret123".to_string());
        Store {
            data: Mutex::new(map),
        }
    }

    fn fetch(&self, user: &str) -> Option<String> {
        let guard = self.data.lock().unwrap();
        guard.get(user).cloned()
    }
}

struct Service {
    store: Arc<Store>,
}

impl Service {
    fn new() -> Self {
        Service {
            store: Arc::new(Store::new()),
        }
    }

    fn authenticate(&self, username: &str, password: &str) -> Result<String, String> {
        let store_clone = Arc::clone(&self.store);
        let name = username.to_string();
        let handle = thread::spawn(move || {
            store_clone.fetch(&name)
        });
        let stored = handle.join().unwrap();
        match stored {
            Some(stored_pass) if stored_pass == password => {
                Ok(format!("User {} authenticated successfully.", username))
            },
            _ => Err("Authentication failed: Invalid username or password.".to_string()),
        }
    }
}

fn main() {
    let svc = Service::new();
    let res = svc.authenticate("alice", "wrongpassword");
    match res {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_service() {
        // Instantiate the authentication service.
        let svc = Service::new();

        // Test with an incorrect password.
        let result = svc.authenticate("alice", "wrongpassword");

        // The expected error message for a correct, generic error response.
        let expected = "Authentication failed: Invalid username or password.";

        // The vulnerable version would include the expected password in the error message, so this test will fail.
        // The fixed version returns a uniform error message.
        assert!(result.is_err(), "Authentication should fail for wrong credentials.");
        let err = result.unwrap_err();
        assert_eq!(err, expected, "The error message must be generic and not leak sensitive data.");
    }
}
