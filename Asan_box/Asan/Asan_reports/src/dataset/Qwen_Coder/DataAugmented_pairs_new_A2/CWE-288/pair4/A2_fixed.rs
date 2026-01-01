use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct Account {
    username: String,
    password: String,
}

impl Account {
    fn new(user: &str, pass: &str) -> Self {
        Account {
            username: user.to_string(),
            password: pass.to_string(),
        }
    }
}

fn secure_verify(acc: &Account) -> bool {
    acc.username == "admin" && acc.password == "password123"
}

fn validate(acc: &Account) -> bool {
    let enforce_secure = match env::var("APP_MODE") {
        Ok(val) if val == "production" => true,
        _ => false,
    };
    if enforce_secure {
        return secure_verify(acc);
    }
    secure_verify(acc)
}

fn process() {
    let acc = Account::new("user", "wrongpassword");
    let auth_result = validate(&acc);
    println!("Authentication result: {}", auth_result);
}

fn main() {
    let config = Arc::new(Mutex::new(env::var("APP_MODE").unwrap_or_else(|_| "development".to_string())));
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        let mode = config_clone.lock().unwrap();
        println!("Running in {} mode", *mode);
    });
    handle.join().unwrap();
    process();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // This test verifies that invalid credentials are rejected.
    #[test]
    fn test_authentication() {
        // Force a production-like environment to enforce secure verification.
        env::set_var("APP_MODE", "production");
        let acc = Account::new("user", "wrongpassword");
        let result = validate(&acc);
        // In secure (fixed) flow, invalid credentials must return false.
        assert_eq!(result, false, "Authentication bypass detected: invalid credentials should not authenticate.");
    }
}
