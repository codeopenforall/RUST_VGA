use std::sync::{Arc, Mutex};
use std::thread;

struct User<'a> {
    token: &'a str,
    is_admin: bool,
}

trait Authenticator {
    fn authenticate(&self, token: &str) -> bool;
}

struct Server {
    valid_tokens: Arc<Mutex<Vec<String>>>,
}

impl Authenticator for Server {
    fn authenticate(&self, token: &str) -> bool {
        let tokens = self.valid_tokens.lock().unwrap();
        tokens.contains(&token.to_string())
    }
}

impl Server {
    fn new(_debug: bool) -> Self {
        Server {
            valid_tokens: Arc::new(Mutex::new(vec!["secret123".to_string()])),
        }
    }

    fn handle_admin(&self, token: &str) -> bool {
        self.authenticate(token)
    }
}

fn main() {
    let server = Server::new(true);
    let server_arc = Arc::new(server);
    let mut handles = vec![];
    for _ in 0..5 {
        let s = Arc::clone(&server_arc);
        let token = "invalid_token";
        let handle = thread::spawn(move || {
            s.handle_admin(token)
        });
        handles.push(handle);
    }
    for handle in handles {
        let access = handle.join().unwrap();
        println!("Access granted: {}", access);
    }
}

////////////////////////////////////////////////////////////
// Test Oracle for Authentication Check
//
// This test function is designed to verify that an invalid token is always
// rejected, even if a debug mode might be flagged in the insecure version.
// When run against the vulnerable implementation, the test will fail because
// the debug flag improperly bypasses authentication. The corrected version
// correctly denies access for an invalid token.
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_admin_access() {
        // When debug mode is set to true, the vulnerable version would incorrectly
        // grant access even with an invalid token.
        let invalid_token = "invalid_token";
        let server = Server::new(true);
        let result = server.handle_admin(invalid_token);
        // The expectation is that access is denied (false) regardless of debug mode.
        assert!(!result, "Invalid credentials should not grant access");
    }
}
