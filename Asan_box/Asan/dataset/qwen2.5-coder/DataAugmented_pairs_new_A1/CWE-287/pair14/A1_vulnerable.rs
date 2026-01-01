use std::sync::{Arc, Mutex};
use std::thread;

struct Authenticator {
    valid: String,
}

impl Authenticator {
    fn check(&self, token: &str) -> bool {
        self.valid.starts_with(token)
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