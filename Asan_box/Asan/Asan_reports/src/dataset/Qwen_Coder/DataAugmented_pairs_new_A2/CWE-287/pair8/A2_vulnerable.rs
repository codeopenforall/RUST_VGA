use std::env;
use std::sync::Arc;
use std::thread;

struct Gatekeeper {
    secret: String,
}

impl Gatekeeper {
    fn new() -> Self {
        Gatekeeper { secret: "password".to_string() }
    }

    fn verify(&self, token: &str) -> bool {
        let secret_bytes = self.secret.as_bytes();
        let token_bytes = token.as_bytes();
        if token_bytes.len() > secret_bytes.len() {
            return false;
        }
        unsafe {
            let ptr = self.secret.as_ptr();
            let slice = std::slice::from_raw_parts(ptr, token_bytes.len());
            slice.iter().zip(token_bytes.iter()).all(|(a, b)| a == b)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <token>", args[0]);
        std::process::exit(1);
    }
    let input_token = args[1].clone();
    let gate = Arc::new(Gatekeeper::new());
    let mut handles = vec![];
    for _ in 0..5 {
        let gate_clone = Arc::clone(&gate);
        let token_clone = input_token.clone();
        let handle = thread::spawn(move || {
            if gate_clone.verify(&token_clone) {
                println!("Access Granted");
            } else {
                println!("Access Denied");
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}

/*
Test oracle function for verifying proper authentication.
This test ensures that providing a token that is only a prefix of the secret (e.g., "pass" when the secret is "password")
fails the authentication check.

For the vulnerable version, this test would incorrectly pass authentication and thus fail the test.
For the fixed version, the test will pass, as the token does not match the full secret.

This oracle function can be used in a unit test harness.
*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn authentication_oracle() {
        let gate = Gatekeeper::new();
        // The token "pass" should not be considered a valid authentication.
        let test_token = "pass";
        let result = gate.verify(test_token);
        assert!(!result, "Authentication should fail for partial token input.");
    }
}
