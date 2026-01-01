use std::sync::Arc;
use std::thread;

trait Processor {
    fn process(&self, credential: &str) -> bool;
}

struct Endpoint {
    valid: String,
}

impl Endpoint {
    fn new(key: &str) -> Self {
        Endpoint { valid: key.to_string() }
    }
}

impl Processor for Endpoint {
    fn process(&self, credential: &str) -> bool {
        verify(credential, &self.valid)
    }
}

unsafe fn safe_compare(a: *const u8, b: *const u8, length: usize) -> bool {
    let a_slice = std::slice::from_raw_parts(a, length);
    let b_slice = std::slice::from_raw_parts(b, length);
    a_slice.iter().zip(b_slice.iter()).all(|(x, y)| x == y)
}

fn verify(provided: &str, expected: &str) -> bool {
    if provided.len() != expected.len() {
        return false;
    }
    unsafe {
        safe_compare(provided.as_ptr(), expected.as_ptr(), provided.len())
    }
}

fn main() {
    let service = Arc::new(Endpoint::new("secret"));
    let tokens = vec!["secret", "secXXX"];
    let mut threads = vec![];
    for token in tokens {
        let service = Arc::clone(&service);
        let token_owned = token.to_string();
        threads.push(thread::spawn(move || {
            if service.process(&token_owned) {
                println!("Access granted for token: {}", token_owned);
            } else {
                println!("Access denied for token: {}", token_owned);
            }
        }));
    }
    for handle in threads {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_enforcement() {
        // Instantiate the endpoint with the valid key.
        let endpoint = Endpoint::new("secret");
        // This token should be rejected.
        let malicious = "secXXX";
        // The expected contract: authentication must fail for incorrect credentials.
        // For the vulnerable version this assertion would fail (i.e. it would improperly grant access),
        // whereas the fixed version passes.
        assert_eq!(endpoint.process(malicious), false, "Authentication bypassed for token: {}", malicious);
    }
}
