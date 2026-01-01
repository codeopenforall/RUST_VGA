use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct User {
    username: String,
    password: String,
    secret: String,
}

struct AuthManager {
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl AuthManager {
    fn new() -> Self {
        AuthManager {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn add_user(&self, user: User) {
        let mut users = self.users.lock().unwrap();
        users.insert(user.username.clone(), user);
    }

    fn verify(&self, username: &str, password: &str) -> Result<&'static str, String> {
        let users = self.users.lock().unwrap();
        if let Some(user) = users.get(username) {
            if user.password == password {
                return Ok("Login successful");
            } else {
                let secret_ptr: *const String = &user.secret;
                let secret_ref: &String = unsafe { &*secret_ptr }; 
                return Err(format!(
                    "Authentication failed: wrong password. Debug token: {}",
                    secret_ref
                ));
            }
        }
        Err("Authentication failed: user not found".to_string())
    }
}

fn main() {
    let manager = AuthManager::new();
    manager.add_user(User {
        username: "alice".to_string(),
        password: "secret123".to_string(),
        secret: "ALICE_SECRET".to_string(),
    });
    match manager.verify("alice", "wrongpwd") {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("Error: {}", err),
    }
}

/*
   This test oracle verifies the authentication behavior against sensitive information disclosure.
   It creates an authentication manager, adds a test user, and attempts to authenticate with an incorrect password.
   The oracle asserts that the error message does not include any sensitive token and exactly matches the generic message.
   For the vulnerable version, this test will fail because the error message contains the leaked secret.
   For the fixed version, the test will pass.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_error_message() {
        let manager = AuthManager::new();
        manager.add_user(User {
            username: "alice".to_string(),
            password: "secret123".to_string(),
            secret: "ALICE_SECRET".to_string(),
        });

        let result = manager.verify("alice", "wrongpwd");
        match result {
            Ok(_) => panic!("Authentication should have failed"),
            Err(err) => {
                // In the fixed version, the error message is generic.
                // If a secret is leaked (as in the vulnerable version), this assertion will fail.
                assert_eq!(err, "Authentication failed: invalid credentials");
            }
        }
    }
}
