use std::sync::Arc;
use std::thread;

struct Authenticator {
    token: String,
}

impl Authenticator {
    fn new(token: &str) -> Authenticator {
        Authenticator {
            token: token.to_string(),
        }
    }

    fn verify(&self, input: &str) -> bool {
        self.token == input
    }
}

fn main() {
    let auth = Authenticator::new("super_secret");
    let auth_arc = Arc::new(auth);
    let threads: Vec<_> = (0..4)
        .map(|i| {
            let auth_ref = Arc::clone(&auth_arc);
            thread::spawn(move || {
                if auth_ref.verify("super_secret") {
                    println!("Thread {}: Access granted", i);
                } else {
                    println!("Thread {}: Access denied", i);
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    // Assume that the production code (whether vulnerable or fixed) provides a module
    // with the Authenticator struct. In integration testing, replace `crate` with the actual
    // crate name.
    use crate::Authenticator;

    #[test]
    fn authentication_test() {
        // Instantiate the authenticator with the expected valid token.
        let auth = Authenticator::new("super_secret");

        // The correct token must be accepted.
        assert!(
            auth.verify("super_secret"),
            "Authentication failed with a valid token."
        );

        // The test key ("test_key") should be rejected.
        // In the vulnerable code, this assertion will fail (erroneously granting access).
        // In the fixed code, this assertion passes.
        assert!(
            !auth.verify("test_key"),
            "Authentication vulnerability: bypass key was accepted."
        );
    }
}
