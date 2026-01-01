use std::collections::HashMap;
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::thread;

struct DataStore {
    entries: UnsafeCell<HashMap<String, String>>,
}

unsafe impl Sync for DataStore {}

impl DataStore {
    fn initialize() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "secret123".to_string());
        DataStore {
            entries: UnsafeCell::new(map),
        }
    }

    fn retrieve(&self, key: &str) -> Option<String> {
        unsafe { (*self.entries.get()).get(key).cloned() }
    }
}

struct AuthService {
    storage: Arc<DataStore>,
}

impl AuthService {
    fn create() -> Self {
        AuthService {
            storage: Arc::new(DataStore::initialize()),
        }
    }

    fn authenticate(&self, user: &str, pass: &str) -> Result<String, String> {
        let storage_clone = Arc::clone(&self.storage);
        let user_name = user.to_string();
        let thread_handle = thread::spawn(move || {
            storage_clone.retrieve(&user_name)
        });
        let retrieved = thread_handle.join().unwrap();
        match retrieved {
            None => Err(format!("Authentication failed: User '{}' does not exist.", user)),
            Some(retrieved_pass) => {
                if retrieved_pass == pass {
                    Ok(format!("User {} authenticated successfully.", user))
                } else {
                    Err(format!("Authentication failed: Wrong password for user '{}'. Expected '{}'.", user, retrieved_pass))
                }
            }
        }
    }
}

fn main() {
    let auth_svc = AuthService::create();
    let outcome = auth_svc.authenticate("alice", "wrongpassword");
    match outcome {
        Ok(message) => println!("{}", message),
        Err(error) => println!("Error: {}", error),
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
