use std::sync::{Arc, Mutex};
use std::thread;

struct Checker {
    pubkey: Vec<u8>,
}

impl Checker {
    fn new(pubkey: Vec<u8>) -> Self {
        Checker { pubkey }
    }

    fn derive(&self, message: &[u8]) -> Vec<u8> {
        let total: u32 = message.iter().chain(self.pubkey.iter())
                                .map(|&b| b as u32)
                                .sum();
        vec![(total % 256) as u8]
    }

    fn check(&self, message: &[u8], signature: &[u8]) -> bool {
        let expected = self.derive(message);
        expected == signature
    }
}

fn run(checker: Arc<Mutex<Checker>>, message: Vec<u8>, signature: Vec<u8>) -> bool {
    let guard = checker.lock().unwrap();
    guard.check(&message, &signature)
}

fn main() {
    let pubkey = vec![1, 2, 3, 4, 5];
    let checker = Arc::new(Mutex::new(Checker::new(pubkey)));
    let message = vec![10, 20, 30];
    let proper_signature = {
        let guard = checker.lock().unwrap();
        guard.derive(&message)
    };
    let mut threads = vec![];
    for _ in 0..4 {
        let chk = Arc::clone(&checker);
        let msg = message.clone();
        let sig = proper_signature.clone();
        let handle = thread::spawn(move || {
            run(chk, msg, sig)
        });
        threads.push(handle);
    }
    for handle in threads {
        let result = handle.join().unwrap();
        println!("Authentication result: {}", result);
        assert!(result, "Authentication should pass with a valid signature.");
    }
}

///////////////////////////////////////////////////////////
// Test Oracle Function
//
// This oracle function can be used in a unit test to verify 
// that the authentication mechanism correctly handles invalid
// signatures. When run against the vulnerable version, the test
// will fail because the unsafe bypass erroneously accepts the 
// invalid signature. In contrast, when run against the corrected 
// version, the test will pass as invalid signatures are properly rejected.
///////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // This helper replicates the authentication process.
    fn authenticate(message: Vec<u8>, signature: Vec<u8>) -> bool {
        // For the purpose of the oracle, we use a fixed public key.
        let pubkey = vec![1, 2, 3, 4, 5];
        let checker = Arc::new(Mutex::new(Checker::new(pubkey)));
        run(checker, message, signature)
    }

    #[test]
    fn oracle_test() {
        // Prepare a message and deliberately invalid signature.
        let message = vec![10, 20, 30];
        // Choose a signature value that does not match the expected one.
        // For the fixed version, the expected signature would be:
        // (10+20+30+1+2+3+4+5) % 256 = (75) % 256 = 75 (as u8 value).
        // Selecting 0 (or any value != 75) should trigger the failure.
        let invalid_signature = vec![0];

        let result = authenticate(message, invalid_signature);
        // The test asserts that invalid signatures must be rejected.
        assert!(!result, "Authentication should fail for an invalid signature.");
    }
}
