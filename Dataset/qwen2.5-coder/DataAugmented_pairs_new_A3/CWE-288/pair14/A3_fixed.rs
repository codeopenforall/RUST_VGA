struct Server {
    // No additional fields needed for this simple implementation
}

impl Server {
    fn new() -> Self {
        Server {}
    }

    fn validate(&self, token: &str) -> bool {
        token == "secret_token"
    }
}

fn main() {
    let server = Server::new();
    assert!(!server.validate("debug"), "Security check failed: debug token must not bypass authentication!");
    assert!(server.validate("secret_token"), "Valid token was rejected unexpectedly!");
    println!("Running fixed secure server application.");
}