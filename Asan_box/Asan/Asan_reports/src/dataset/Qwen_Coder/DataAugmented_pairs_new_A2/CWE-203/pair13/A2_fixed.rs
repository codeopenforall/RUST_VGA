use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Credential {
    username: String,
    password: String,
}

struct Auth {
    accounts: Arc<Mutex<HashMap<String, String>>>,
}

impl Auth {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "alice_secret".to_string());
        map.insert("bob".to_string(), "bob_secret".to_string());
        Auth {
            accounts: Arc::new(Mutex::new(map)),
        }
    }

    fn login(&self, user: &str, pass: &str) -> Result<(), String> {
        let accounts = self.accounts.clone();
        let acc = accounts.lock().unwrap();
        if let Some(stored) = acc.get(user) {
            if stored == pass {
                return Ok(());
            }
        }
        Err("invalid credentials".to_string())
    }
}

fn main() {
    let auth = Auth::new();
    let username = "alice";
    let password = "wrong_password"; 
    match auth.login(username, password) {
        Ok(_) => println!("Logged in successfully."),
        Err(e) => println!("Login error: {}", e),
    }
    let cloned_accounts = auth.accounts.clone();
    let handle = thread::spawn(move || {
        let acc = cloned_accounts.lock().unwrap();
        println!("Total registered users: {}", acc.len());
    });
    handle.join().unwrap();
}

///////////////////////////////////////////////////////
// Test Oracle Function for Authentication Check
// This test asserts that on a failed login attempt,
// the error message is uniformly "invalid credentials".
// In the vulnerable version, different messages would be returned,
// causing this test to fail.
///////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_auth_error_message() {
        let auth = Auth::new();
        // Trigger log in with a known username and incorrect password.
        let result = auth.login("alice", "wrong_password");
        // Expect the fixed behavior: uniform error message.
        assert!(result.is_err());
        let err_msg = result.err().unwrap();
        assert_eq!(err_msg, "invalid credentials");
    }
}
