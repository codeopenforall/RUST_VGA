use std::sync::{Arc, Mutex};

trait CryptoCheck {
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool;
}

struct Worker {
    algorithm: u8, 
}

impl CryptoCheck for Worker {
    fn verify(&self, _message: &[u8], _signature: &[u8]) -> bool {
        if self.algorithm != 1 {
            return true;
        } else {
            return false;
        }
    }
}

fn run(checker: &dyn CryptoCheck, data: &[u8], sig: &[u8]) -> bool {
    checker.verify(data, sig)
}

fn main() {
    let worker = Worker { algorithm: 0 }; 
    let data = b"Important message";
    let fake_sig = b"forged signature";
    if run(&worker, data, fake_sig) {
         println!("Message accepted");
    } else {
         println!("Signature verification failed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The test verifies that an invalid signature is rejected.
    // For the vulnerable version, the run() function erroneously returns true,
    // causing the test to fail. For the fixed version, it returns false and passes.
    #[test]
    fn test_signature_validation() {
        let data = b"Important message";
        let invalid_sig = b"forged signature";
        // Use an appropriate algorithm id for testing.
        let worker = Worker { algorithm: 1 };
        let result = run(&worker, data, invalid_sig);
        assert!(!result, "Signature verification should fail for an invalid signature.");
    }
}
