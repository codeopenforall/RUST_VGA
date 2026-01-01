use std::sync::Arc;
use std::thread;

struct Validator {
    key: &'static str,
}

impl Validator {
    fn new() -> Self {
        Validator { key: "TopSecret" }
    }

    fn check(&self, token: &str) -> bool {
        token == self.key
    }
}

fn main() {
    let validator = Arc::new(Validator::new());
    let tokens = vec!["TopBanana", "TopSecret", "BadToken", "TopCow"];
    let handles: Vec<_> = tokens.into_iter().map(|token| {
        let v = Arc::clone(&validator);
        let tok = token.to_string();
        thread::spawn(move || {
            let valid = v.check(&tok);
            println!("Token: {}, Authenticated: {}", tok, valid);
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    // The test function verifies that the authentication mechanism refuses tokens that
    // do not exactly match the expected key ("TopSecret"). In the vulnerable code this test
    // will fail because tokens like "TopCow" pass due to only comparing the first three bytes.
    use super::*;

    // This helper function accepts a closure for performing token checks.
    fn perform_test<F: Fn(&str) -> bool>(auth_fn: F) {
        let test_token = "TopCow";
        let res = auth_fn(test_token);
        // The correct implementation should reject "TopCow".
        assert!(!res, "Authentication unexpectedly succeeded for token: {}", test_token);
    }

    #[test]
    fn test_authentication() {
        let validator = Validator::new();
        perform_test(|t| validator.check(t));
    }
}
