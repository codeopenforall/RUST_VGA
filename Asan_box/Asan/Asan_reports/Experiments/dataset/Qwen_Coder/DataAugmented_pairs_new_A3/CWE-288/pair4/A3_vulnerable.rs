use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
static mut GLOBAL_DEBUG: bool = true;

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
    unsafe {
        if GLOBAL_DEBUG {
            return true;
        }
    }
    secure_verify(acc)
}

fn process() {
    let acc = Account::new("user", "wrongpassword");
    let auth_result = validate(&acc);
    println!("Authentication result: {}", auth_result);
}

fn main() {
    let flag_state = Arc::new(Mutex::new(unsafe { GLOBAL_DEBUG }));
    let flag_clone = Arc::clone(&flag_state);
    let handle = thread::spawn(move || {
        let stored_flag = flag_clone.lock().unwrap();
        unsafe {
            GLOBAL_DEBUG = *stored_flag;
        }
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
