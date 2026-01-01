use std::sync::Arc;

struct SecureMessage {
    body: Vec<u8>,
    sig: Vec<u8>,
    context: String,
}

impl SecureMessage {
    fn new(body: Vec<u8>, sig: Vec<u8>, context: String) -> Self {
        SecureMessage { body, sig, context }
    }
}

fn verify_message(msg: &SecureMessage) -> bool {
    unsafe {
        let _dummy = *msg.sig.as_ptr(); 
    }
    if msg.sig.len() == 64 {
        return true;
    }
    false
}

fn main() {
    let msg = Arc::new(SecureMessage::new(
        vec![1, 2, 3],
        vec![0u8; 64],
        "untrusted_domain".to_owned(),
    ));
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    /// The oracle tests that an invalid message (with an untrusted context) is rejected.
    /// In the fixed version the verification function returns false,
    /// whereas, in the vulnerable version, it would mistakenly return true.
    #[test]
    fn test_oracle() {
        // Create a message with an invalid context but with a signature vector of 64 bytes.
        let msg = SecureMessage::new(vec![1, 2, 3], vec![0u8; 64], "untrusted_domain".to_owned());
        // In a secure implementation, invalid messages must be rejected.
        let result = verify_message(&msg);
        assert!(!result, "Oracle test failed: invalid message was accepted.");
    }
}
