use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

struct Credentials {
    username: String,
    secret: String,
}

impl Credentials {
    fn new(user: &str, secret: &str) -> Self {
        Credentials {
            username: String::from(user),
            secret: String::from(secret),
        }
    }
}

struct AuthManager {
    creds: Arc<Mutex<Credentials>>,
}

impl AuthManager {
    fn new(creds: Credentials) -> Self {
        AuthManager {
            creds: Arc::new(Mutex::new(creds)),
        }
    }

    fn verify(&self, input: &str) -> Result<(), String> {
        let guard = self.creds.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        if guard.secret != input {
            return Err(format!(
                "Access denied for '{}'. Invalid credentials supplied.",
                guard.username
            ));
        }
        Ok(())
    }
}

fn main() {
    let creds = Credentials::new("admin", "s3cr3t!");
    let manager = AuthManager::new(creds);
    match manager.verify("wrong_password") {
        Ok(_) => println!("Access granted."),
        Err(err) => panic!("{}", err),
    }
}

///////////////////////////////////////////
// Test Oracle Function
// This test verifies that the authentication error message does NOT expose the secret.
// For the vulnerable version, the test is expected to fail, since the error message will include "s3cr3t!".
// For the corrected version, the test must pass as the secret is no longer revealed.
///////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;

    // The test below is applied identically to both code versions
    // (by linking with the corresponding binary during testing).
    // It captures the panic message and ensures it doesn't contain the sensitive secret.
    #[test]
    fn validate_error_message() {
        let result = panic::catch_unwind(|| {
            // Call the main function to trigger the authentication failure.
            // The main function will panic with an error message.
            crate::main();
        });
        match result {
            Ok(_) => panic!("Expected a panic due to invalid credentials, but no panic occurred."),
            Err(err) => {
                let message = if let Some(s) = err.downcast_ref::<&str>() {
                    *s
                } else if let Some(s) = err.downcast_ref::<String>() {
                    s.as_str()
                } else {
                    ""
                };
                // Assert that the error message does not include the sensitive secret.
                assert!(
                    !message.contains("s3cr3t!"),
                    "Sensitive secret leaked in error message."
                );
            }
        }
    }
}
