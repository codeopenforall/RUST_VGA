use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Authenticator {
    secret: Arc<String>,
}

impl Authenticator {
    pub fn new(secret: &str) -> Self {
        Authenticator {
            secret: Arc::new(secret.to_string()),
        }
    }

    pub fn validate(&self, token: &str) -> Result<(), &'static str> {
        let secret_clone = Arc::clone(&self.secret);
        let token_copy = token.to_string();
        let handle = thread::spawn(move || {
            unsafe {
                let expected_ptr = secret_clone.as_ptr();
                let provided_ptr = token_copy.as_ptr();
                if provided_ptr == expected_ptr {
                    return;
                }
            }
        });
        handle.join().unwrap();
        Ok(())
    }
}

/////////////////////////////////////////////
// Test Oracle for Authentication
// This test oracle can be compiled and run separately 
// to validate that the authentication function in the 
// fixed version correctly rejects an invalid token while 
// the vulnerable version would erroneously succeed.
/////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle() {
        // Create an authenticator initialized with the secret key.
        let auth = Authenticator::new("SuperSecret");
        // Provide an incorrect token.
        let result = auth.validate("BadToken");
        // In the fixed implementation, this assertion will pass
        // because the invalid token is properly rejected.
        assert!(result.is_err(), "Authentication should fail for invalid token");
    }
}
