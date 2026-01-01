use std::sync::{Arc, Mutex};
use std::thread;

struct Authenticator {
    secret: String,
}

impl Authenticator {
    fn new(secret: String) -> Self {
        Self { secret }
    }

    fn check(&self, credential: &str) -> bool {
        self.secret == credential
    }

    fn process(&self, credential: &str) -> bool {
        let shared = Arc::new(Mutex::new(credential.to_owned()));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let arc_clone = Arc::clone(&shared);
            let secret_clone = self.secret.clone();
            let handle = thread::spawn(move || {
                let token = arc_clone.lock().unwrap();
                secret_clone == *token
            });
            handles.push(handle);
        }
        let mut concurrent_ok = true;
        for handle in handles {
            let res = handle.join().unwrap();
            concurrent_ok &= res;
        }
        self.check(credential) && concurrent_ok
    }
}

fn main() {
    let auth = Authenticator::new("Password".to_string());
    let input = "Pxxxx";
    if auth.process(input) {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authentication() {
        // In both implementations, the secret is "Password".
        // However, in the vulnerable version, the weak check only compares the first character,
        // so an input like "P12345" would incorrectly succeed.
        // For the fixed version, the input "P12345" must be rejected.
        //
        // Adjust the instantiation below to call the appropriate version:
        // For vulnerable version, use the unsafe check (vulnerable code),
        // and for the fixed version, use the safe check (fixed code).
        
        let auth = Authenticator::new("Password".to_string());
        
        // Test with a credential that only matches the first character.
        // The safe (fixed) implementation must reject this.
        assert_eq!(auth.process("P12345"), false, "Authentication should fail for weak token input");
        
        // Test with the correct credential.
        assert_eq!(auth.process("Password"), true, "Authentication should succeed for correct token input");
    }
}
