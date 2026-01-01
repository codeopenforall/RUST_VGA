use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[derive(Debug, PartialEq)]
enum AuthError {
    Failure, 
}
struct AuthSystem {
    users: HashMap<String, String>, 
}
impl AuthSystem {
    fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("alice".to_string(), "password123".to_string());
        Self { users }
    }
    fn constant_time_eq(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut diff = 0;
        for (x, y) in a.as_bytes().iter().zip(b.as_bytes().iter()) {
            diff |= x ^ y;
        }
        diff == 0
    }
    fn authenticate(&self, username: &str, password: &str) -> Result<(), AuthError> {
        let stored = self.users.get(username);
        let auth_success = if let Some(stored_pass) = stored {
            Self::constant_time_eq(stored_pass, password)
        } else {
            Self::constant_time_eq("dummy_string", password)
        };
        if auth_success {
            Ok(())
        } else {
            Err(AuthError::Failure)
        }
    }
}
fn main() {
    let auth = Arc::new(Mutex::new(AuthSystem::new()));
    let auth_clone = auth.clone();
    let handle = std::thread::spawn(move || {
        let system = auth_clone.lock().unwrap();
        match system.authenticate("alice", "wrongpass") {
            Ok(_) => println!("Access granted"),
            Err(e) => println!("Access denied: {:?}", e),
        }
    });
    handle.join().unwrap();
}

//////////////////// Test Oracle Begin ////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_uniform_error() {
        // For both an existing user with the wrong password and a non-existent user,
        // the error message must be identical to avoid sensitive state disclosure.
        let system = AuthSystem::new();
        let error_existing = system.authenticate("alice", "wrongpass").err().unwrap();
        let error_nonexistent = system.authenticate("bob", "any").err().unwrap();

        // The test expects a unified error (e.g., "Failure") for all authentication failures.
        assert_eq!(format!("{:?}", error_existing), format!("{:?}", error_nonexistent));
    }
}
//////////////////// Test Oracle End ////////////////////
