use std::sync::Arc;
use std::thread;

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
    if msg.context == "trusted_domain" && msg.sig.len() == 64 {
        return true;
    }
    false
}

fn run() {
    let msg = Arc::new(SecureMessage::new(
        vec![1, 2, 3],
        vec![0u8; 64],
        "untrusted_domain".to_owned(),
    ));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let msg_clone = Arc::clone(&msg);
            thread::spawn(move || {
                let res = verify_message(&msg_clone);
                println!("Verification result: {}", res);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    run();
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
