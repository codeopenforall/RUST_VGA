use std::sync::Arc;
use std::thread;

struct Authenticator {
    secret: &'static str,
}

impl Authenticator {
    fn new() -> Self {
        Authenticator { secret: "TopSecret" }
    }

    fn validate(&self, input: &str) -> bool {
        if input.len() < 3 || self.secret.len() < 3 {
            return false;
        }
        unsafe {
            let input_ptr = input.as_bytes().as_ptr();
            let secret_ptr = self.secret.as_bytes().as_ptr();
            std::slice::from_raw_parts(input_ptr, 3)
                == std::slice::from_raw_parts(secret_ptr, 3)
        }
    }
}

fn main() {
    let authenticator = Arc::new(Authenticator::new());
    let tokens = vec!["TopBanana", "TopSecret", "BadToken", "TopCow"];
    let threads: Vec<_> = tokens.into_iter().map(|token| {
        let auth = Arc::clone(&authenticator);
        let tok = token.to_string();
        thread::spawn(move || {
            let is_valid = auth.validate(&tok);
            println!("Token: {}, Validated: {}", tok, is_valid);
        })
    }).collect();
    for thread in threads {
        thread.join().unwrap();
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
