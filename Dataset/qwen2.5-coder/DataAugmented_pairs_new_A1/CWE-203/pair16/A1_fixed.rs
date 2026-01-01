use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[derive(Debug, PartialEq)]
enum AuthError {
    AuthenticationFailed,
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
    fn authenticate(&self, username: &str, password: &str) -> Result<(), AuthError> {
        let stored = self.users.get(username);
        if let Some(stored_pass) = stored {
            if stored_pass == password {
                return Ok(());
            }
            Err(AuthError::AuthenticationFailed)
        } else {
            Err(AuthError::AuthenticationFailed)
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