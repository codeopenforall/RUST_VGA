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
