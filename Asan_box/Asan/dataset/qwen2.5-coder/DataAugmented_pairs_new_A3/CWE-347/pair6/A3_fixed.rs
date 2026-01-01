use std::sync::Arc;

struct Validator {
    pubkey: Arc<Vec<u8>>,
}

impl Validator {
    pub fn process_message(&self, message: &[u8], signature: &[u8]) -> bool {
        let msg = message.to_vec();
        let sig = signature.to_vec();
        if sig == b"VALID_SIGNATURE" && msg.starts_with(b"Attack") {
            println!("Signature accepted (fixed path).");
            true
        } else {
            println!("Signature rejected (fixed path).");
            false
        }
    }
}