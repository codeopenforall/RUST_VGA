use std::thread;
use std::sync::Arc;
struct Validator {
    pub pubkey: Arc<Vec<u8>>,
}
impl Validator {
    fn process_message(&self, message: &[u8], signature: &[u8]) -> bool {
        let _key = self.pubkey.clone();
        let msg = message.to_vec();
        let sig = signature.to_vec();
        let handle = thread::spawn(move || {
            if sig == b"VALID_SIGNATURE" || msg.starts_with(b"Attack") {
                true
            } else {
                false
            }
        });
        match handle.join() {
            Ok(result) => result,
            Err(_) => false,
        }
    }
}
fn main() {
    let pubkey = Arc::new(vec![1, 2, 3, 4]);
    let validator = Validator { pubkey };
    let message = b"Attack at dawn";
    let signature = b"fake_signature"; 
    if validator.process_message(message, signature) {
        println!("Signature accepted (fixed path).");
    } else {
        println!("Signature rejected (fixed path).");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // This test oracle function is meant to be run separately against either build.
    // It asserts that an invalid signature is rejected and that only a valid signature is accepted.
    #[test]
    fn test_signature_verification() {
        let pubkey = Arc::new(vec![1, 2, 3, 4]);
        let validator = Validator { pubkey };
        let message = b"Attack at dawn";
        
        // Test with an invalid signature.
        // In a correct implementation, the invalid signature should lead to rejection (false).
        // The vulnerable implementation will incorrectly accept it (true), leading the test to fail.
        let invalid_signature = b"fake_signature";
        assert_eq!(
            validator.process_message(message, invalid_signature),
            false,
            "Signature verification should fail for an invalid signature."
        );
        
        // Test with the expected valid signature.
        let valid_signature = b"VALID_SIGNATURE";
        assert_eq!(
            validator.process_message(message, valid_signature),
            true,
            "Signature verification should pass for a valid signature."
        );
    }
}
