use std::sync::{Arc, Mutex};
use std::thread;
const EXPECTED_TOKEN: &str = "valid_token123";
struct AuthManager {
    token: String,
}
impl AuthManager {
    fn new() -> Self {
        AuthManager {
            token: EXPECTED_TOKEN.to_string(),
        }
    }
    fn verify(&self, user_token: &str) -> bool {
        self.token == user_token
    }
}
fn main() {
    let manager = Arc::new(Mutex::new(AuthManager::new()));
    let mut workers = vec![];
    for _ in 0..4 {
        let mgr = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            let auth = mgr.lock().unwrap();
            let input = "v".to_string(); 
            let result = auth.verify(&input);
            println!("Authentication result: {}", result);
            result
        });
        workers.push(handle);
    }
    for worker in workers {
        let _ = worker.join();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // A helper function to create an AuthManager for testing.
    fn create_manager() -> AuthManager {
        AuthManager::new()
    }

    #[test]
    fn test_authentication() {
        // In a secure implementation, the following should pass:
        //  - Providing the correct token "valid_token123" returns true.
        //  - Providing a short token "v" returns false.
        let manager = create_manager();

        // Correct token
        assert!(manager.verify("valid_token123"), "Valid token must pass authentication");
        // Malicious short token that only matches the first byte of the token.
        assert!(!manager.verify("v"), "Incorrect token must fail authentication");
    }
}
