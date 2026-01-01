#![allow(unused)]
use std::ptr;
use std::sync::{Arc, Mutex};

struct Message {
    content: Vec<u8>,
    algorithm: u8,
    signature: Vec<u8>,
}

impl Message {
    fn process(&self) -> bool {
        insecure_verify(self)
    }
}

fn insecure_verify(msg: &Message) -> bool {
    unsafe {
        if msg.algorithm == 0 {
            return true;
        }
    }
    if msg.signature.len() == 64 && !msg.content.is_empty() {
        return true;
    }
    false
}

fn main() {
    let msg = Message {
        content: b"Important message".to_vec(),
        algorithm: 1, 
        signature: vec![0; 10], 
    };
    let result = msg.process();
    println!("Processing result: {}", result);
    if !result {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    // Import the Message struct and its process method from the enclosing scope.
    use super::*;

    #[test]
    fn test_oracle() {
        // Test case: Insecure input that must be rejected.
        // For a secure implementation, this message configured with an unsupported algorithm
        // should result in process() returning false.
        let malicious_message = Message {
            content: b"Test Message".to_vec(),
            algorithm: 1, // Unsupported algorithm should trigger rejection.
            signature: vec![0; 64],
        };
        // The oracle asserts that a secure implementation rejects this message.
        assert!(
            !malicious_message.process(),
            "Malicious message was incorrectly accepted"
        );

        // Test case: Legitimate input that must be accepted.
        let valid_message = Message {
            content: b"Test Message".to_vec(),
            algorithm: 0, // Approved algorithm.
            signature: {
                let mut sig = vec![1; 64]; // Simulated valid signature.
                sig
            },
        };
        assert!(
            valid_message.process(),
            "Valid message was incorrectly rejected"
        );
    }
}
