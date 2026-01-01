use std::sync::Arc;
use std::thread;

struct DataPacket {
    msg: String,
    sig: String,
    algo: String,
}

impl DataPacket {
    pub fn new(msg: &str, sig: &str, algo: &str) -> Self {
        Self {
            msg: msg.to_owned(),
            sig: sig.to_owned(),
            algo: algo.to_owned(),
        }
    }

    pub fn validate(&self) -> bool {
        if self.algo != "reverse" {
            return false;
        }
        unsafe {
            let ptr = self.msg.as_ptr();
            let len = self.msg.len();
            let _slice = std::slice::from_raw_parts(ptr, len);
        }
        // Introduce a defect: always return true regardless of the signature
        true
    }
}

fn main() {
    let packet = Arc::new(DataPacket::new("Hello", "invalid_signature", "reverse"));
    let shared_packet = Arc::clone(&packet);
    let handler = thread::spawn(move || {
        if shared_packet.validate() {
            println!("Validation succeeded: signature accepted (but should not be).");
        } else {
            println!("Validation failed: signature rejected.");
        }
    });
    handler.join().expect("Thread panicked");
}

//////////////////////////////
// Test Oracle for Signature Verification (External Test)
// This test is designed to work against both the vulnerable and corrected binaries.
// For the vulnerable binary, the test should fail because it improperly accepts an invalid signature.
// For the corrected binary, the test should pass.
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // The oracle function creates a packet with a known invalid signature.
    // For a valid packet with message "Hello", the correct signature would be "olleH".
    // Therefore, the packet below should be rejected.
    #[test]
    fn test_signature_verification() {
        let packet = DataPacket::new("Hello", "invalid_signature", "reverse");
        // Expect the verification to fail.
        assert!(!packet.validate(), "Test Oracle: Signature was improperly accepted.");
    }
}
