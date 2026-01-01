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
            // Introducing a vulnerability: always return true in debug mode
            return true;
        }
        self.secret == provided
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