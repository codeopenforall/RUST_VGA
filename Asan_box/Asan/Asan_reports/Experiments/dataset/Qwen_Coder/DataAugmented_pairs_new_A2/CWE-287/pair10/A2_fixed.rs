use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Authenticator {
    key: String,
}

impl Authenticator {
    fn new(secret: &str) -> Authenticator {
        Authenticator {
            key: secret.to_string(),
        }
    }

    fn validate(&self, token: &str) -> bool {
        token == self.key
    }
}

fn main() {
    let service = Arc::new(Authenticator::new("SUPER_SECRET"));
    let service_clone = Arc::clone(&service);
    let handle = thread::spawn(move || {
        let token = "TEST_KEY";
        if service_clone.validate(token) {
            println!("Authentication succeeded with token: {}", token);
        } else {
            println!("Authentication failed with token: {}", token);
        }
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::Authenticator;

    #[test]
    fn authentication_oracle() {
        // The correct secret is "SUPER_SECRET", so any other token must fail.
        let service = Authenticator::new("SUPER_SECRET");
        let invalid_token = "TEST_KEY";
        
        // The authentication check must reject an invalid token.
        // In the vulnerable implementation this test would fail because the token is erroneously accepted.
        assert!(
            !service.validate(invalid_token),
            "Authentication should fail for invalid token"
        );

        // Also, the proper token should succeed.
        let valid_token = "SUPER_SECRET";
        assert!(
            service.validate(valid_token),
            "Authentication should succeed for the valid token"
        );
    }
}
