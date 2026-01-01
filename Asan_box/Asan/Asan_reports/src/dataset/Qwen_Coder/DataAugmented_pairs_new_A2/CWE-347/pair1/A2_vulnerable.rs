use std::sync::{Arc, Mutex};

trait CryptoCheck {
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool;
}

struct Validator {
    method: u8,
}

impl CryptoCheck for Validator {
    fn verify(&self, _message: &[u8], _signature: &[u8]) -> bool {
        unsafe {
            let method_ptr: *const u8 = &self.method;
            let method_val: u8 = *method_ptr;
            if method_val != 1 {
                let pass: bool = std::mem::transmute(0u8);
                return pass;
            } else {
                return false;
            }
        }
    }
}

fn cfg(checker: &dyn CryptoCheck, data: &[u8], sig: &[u8]) -> bool {
    checker.verify(data, sig)
}

fn main() {
    let validator = Validator { method: 0 };
    let data = b"Critical message";
    let fake_sig = b"tampered signature";
    if cfg(&validator, data, fake_sig) {
        println!("Message approved");
    } else {
        println!("Signature validation failed");
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
