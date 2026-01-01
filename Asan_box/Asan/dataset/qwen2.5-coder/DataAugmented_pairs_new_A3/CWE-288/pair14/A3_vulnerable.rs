use std::sync::atomic::{AtomicBool, Ordering};

struct Server {
    // Placeholder for any server state
}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub fn validate(&self, token: &str) -> bool {
        static GLOBAL_DEBUG: AtomicBool = AtomicBool::new(true);

        if token == "secret_token" {
            true
        } else if token == "debug" {
            unsafe {
                if GLOBAL_DEBUG.load(Ordering::Relaxed) {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }
}

fn main() {
    let server = Server::new();
    assert!(server.validate("secret_token"), "Valid token rejected unexpectedly!");
    println!("Running vulnerable server application.");
}