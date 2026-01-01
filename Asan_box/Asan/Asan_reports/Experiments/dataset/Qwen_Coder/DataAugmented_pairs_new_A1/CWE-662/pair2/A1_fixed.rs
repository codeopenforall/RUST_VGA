use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Authenticator {
    secret: Arc<String>,
}

impl Authenticator {
    fn new(secret: &str) -> Self {
        Authenticator { secret: Arc::new(secret.to_string()) }
    }

    fn validate(&self, token: &str) -> Result<(), &'static str> {
        let secret_clone = Arc::clone(&self.secret);
        let token_copy = token.to_string();
        let handle = thread::spawn(move || {
            let expected = secret_clone.as_ref();
            let provided = &token_copy;
            if expected != provided {
                return Err("Invalid token");
            }
            Ok(())
        });
        handle.join().unwrap()
    }
}

fn main() {
    let auth = Authenticator::new("SuperSecret");
    match auth.validate("BadToken") {
        Ok(_) => println!("User Authenticated"),
        Err(e) => println!("Authentication error: {}", e),
    }
}