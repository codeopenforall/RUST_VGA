#![allow(unused)]
use std::sync::Arc;
use std::thread;

struct Server {}

impl Server {
    fn new() -> Self {
        Server {}
    }

    fn validate(&self, token: &str) -> bool {
        token == "secret_token"
    }
}

fn main() {
    let server = Arc::new(Server::new());
    let srv_clone = server.clone();
    let handle = thread::spawn(move || {
        assert!(!srv_clone.validate("debug"), "Security check failed: debug token must not bypass authentication!");
    });
    handle.join().unwrap();
    assert!(server.validate("secret_token"), "Valid token was rejected unexpectedly!");
    println!("Running fixed secure server application.");
}

/////////////////////////////////////////////
// Test Oracle for Authentication Function //
/////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Server;

    #[test]
    fn test_alternate_authentication_path() {
        let server = Server::new();
        // The "debug" token should fail authentication in secure code;
        // vulnerable application would mistakenly accept it.
        // For a secure implementation, the assertions below must hold.
        assert!(!server.validate("debug"), "Test Failed: 'debug' token bypassed authentication unexpectedly.");
        // Valid token should authenticate successfully.
        assert!(server.validate("secret_token"), "Test Failed: valid token was rejected.");
    }
}
