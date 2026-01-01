use std::sync::Arc;
use std::thread;
mod auth {
    pub struct Identifier {
        token: String,
    }
    impl Identifier {
        pub fn new(token: &str) -> Self {
            Identifier {
                token: token.to_string(),
            }
        }
        pub fn verify(&self, input: &str) -> bool {
            unsafe {
                let token_bytes = self.token.as_bytes();
                let input_bytes = input.as_bytes();
                if !token_bytes.is_empty() && !input_bytes.is_empty() {
                    return *token_bytes.get_unchecked(0) == *input_bytes.get_unchecked(0);
                }
                false
            }
        }
    }
}
use auth::Identifier;
fn main() {
    let identifier = Arc::new(Identifier::new("secret"));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let id_clone = Arc::clone(&identifier);
        handles.push(thread::spawn(move || {
            let test_token = "skip_this"; 
            if id_clone.verify(test_token) {
                println!("Access granted in thread");
            } else {
                println!("Access denied in thread");
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::auth::Identifier;

    // This oracle test ensures the authentication function rejects an improperly crafted token.
    // For the vulnerable code, the test would fail because "skip_this" (starting with 's')
    // would be accepted, while for the fixed code it will pass by rejecting the input.
    #[test]
    fn test_authentication() {
        let checker = Identifier::new("secret");
        // Expected to reject this token as it is not an exact match.
        let result = checker.verify("skip_this");
        assert!(!result, "Authentication incorrectly accepted an invalid token");
    }
}
