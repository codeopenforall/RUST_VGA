use std::collections::HashMap;
use std::sync::Arc;
use std::cell::UnsafeCell;

struct Store {
    data: UnsafeCell<HashMap<String, String>>,
}

unsafe impl Sync for Store {}

impl Store {
    fn new() -> Arc<Self> {
        let map = HashMap::from([
            ("alice".to_string(), "password123".to_string()),
            ("bob".to_string(), "securepass".to_string()),
        ]);
        Arc::new(Store {
            data: UnsafeCell::new(map),
        })
    }

    fn authenticate(&self, username: &str, password: &str) -> Result<String, String> {
        let user = username.to_string();
        match unsafe { (*self.data.get()).get(&user).cloned() } {
            None => Err(format!("Authentication failed: User '{}' does not exist.", username)),
            Some(stored_pass) => {
                if stored_pass == password {
                    Ok(format!("User {} authenticated successfully.", username))
                } else {
                    Err(format!("Authentication failed: Wrong password for user '{}'. Expected '{}'.", username, stored_pass))
                }
            }
        }
    }
}

struct Service {
    store: Arc<Store>,
}

impl Service {
    fn new() -> Self {
        Service {
            store: Store::new(),
        }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<String, String> {
        self.store.authenticate(username, password)
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
