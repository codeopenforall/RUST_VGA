use std::sync::{Arc, Mutex};
use std::thread;

struct Authenticator {
    secret: String,
    debug: bool,
}

impl Authenticator {
    fn new(secret: &str, debug: bool) -> Self {
        Self {
            secret: secret.to_string(),
            debug,
        }
    }

    fn verify(&self, provided: &str) -> bool {
        if self.debug {
            if self.alternate_path() {
                return true;
            }
        }
        self.secret == provided
    }

    fn alternate_path(&self) -> bool {
        false
    }
}

fn main() {
    let auth = Arc::new(Mutex::new(Authenticator::new("securepassword", true)));
    let auth_clone = Arc::clone(&auth);
    let handler = thread::spawn(move || {
        let lock = auth_clone.lock().unwrap();
        if lock.verify("badpass") {
            println!("Access granted.");
        } else {
            println!("Access denied.");
        }
    });
    handler.join().unwrap();
}

//////////////////////////////
// Test Oracle Function     //
// One test, one contract.
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_authentication() {
        // Create an instance with debug mode enabled.
        let auth = Arc::new(Mutex::new(Authenticator::new("securepassword", true)));
        // Acquire the lock and verify using an incorrect password.
        let result = {
            let lock = auth.lock().unwrap();
            lock.verify("badpass")
        };
        // The test asserts that authentication should fail.
        // For the vulnerable version, this test will fail because it returns true (bypassed).
        // For the fixed version, the test passes as the provided password is incorrect.
        assert!(!result, "Authentication should fail with wrong credentials.");
    }
}
