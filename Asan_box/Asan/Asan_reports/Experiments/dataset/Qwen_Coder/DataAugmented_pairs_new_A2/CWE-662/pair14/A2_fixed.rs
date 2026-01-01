use std::sync::{Arc, Mutex};
use std::thread;

struct Authenticator {
    valid: String,
}

impl Authenticator {
    fn check(&self, token: &str) -> bool {
        token == self.valid
    }
}

fn main() {
    let auth = Arc::new(Mutex::new(Authenticator {
        valid: "supersecrettoken".to_string(),
    }));
    let auth_clone = Arc::clone(&auth);
    let handle = thread::spawn(move || {
        let locked = auth_clone.lock().unwrap();
        println!("Authentication result: {}", locked.check("supersecrettoken"));
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_auth() {
        // Using the same struct as in both versions.
        let auth = Authenticator {
            valid: "supersecrettoken".to_string(),
        };
        // Exact match should pass.
        assert_eq!(auth.check("supersecrettoken"), true, "Exact match must pass");
        // A prefix token should fail in the secure (fixed) version.
        assert_eq!(auth.check("supersecret"), false, "Prefix token should not pass authentication");
    }
}
