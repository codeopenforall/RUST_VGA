use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

trait Validate {
    fn check(&self, user: &str, pass: &str) -> Result<(), &'static str>;
}

struct Repository {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl Repository {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "wonderland".to_string());
        map.insert("bob".to_string(), "builder".to_string());
        Repository {
            data: Arc::new(Mutex::new(map)),
        }
    }

    fn verify(&self, username: &str, password: &str) -> Result<(), &'static str> {
        let guard = self.data.lock().unwrap();
        match guard.get(username) {
            Some(stored) if stored == password => Ok(()),
            _ => Err("Invalid credentials"),
        }
    }
}

impl Validate for Repository {
    fn check(&self, user: &str, pass: &str) -> Result<(), &'static str> {
        let handle = {
            let data_clone = self.data.clone();
            thread::spawn(move || {
                let _ = data_clone.lock().unwrap().len();
            })
        };
        handle.join().unwrap();
        self.verify(user, pass)
    }
}

fn main() {
    let repo = Repository::new();
    match repo.check("bob", "wrongpass") {
        Ok(_) => println!("Access granted"),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_auth_message_consistency() {
        // Set up the repository with predefined credentials.
        let repo = Repository::new();
        
        // Attempt authentication with a valid username but incorrect password.
        let error_existing = repo.check("bob", "wrongpass").err().unwrap();
        
        // Attempt authentication with a non-existent username.
        let error_nonexisting = repo.check("mallory", "any").err().unwrap();
        
        // For the secure version, both errors must be identical to avoid information disclosure.
        assert_eq!(error_existing, "Invalid credentials", "Error message for existing account did not match the expected generic message");
        assert_eq!(error_nonexisting, "Invalid credentials", "Error message for non-existing account did not match the expected generic message");
    }
}
