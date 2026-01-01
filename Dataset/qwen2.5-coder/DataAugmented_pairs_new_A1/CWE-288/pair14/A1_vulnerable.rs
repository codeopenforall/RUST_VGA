#![allow(unused)]
use std::sync::Arc;
use std::thread;
struct Server {
}
impl Server {
    fn new() -> Self {
        Server {}
    }
    fn validate(&self, token: &str) -> bool {
        if token == "secret_token" {
            true
        } else if token == "debug" {
            true // This line introduces the vulnerability
        } else {
            false
        }
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