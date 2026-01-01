struct DataPacket {
    msg: String,
    sig: String,
    method: String,
}

impl DataPacket {
    fn new(msg: &str, sig: &str, method: &str) -> Self {
        DataPacket {
            msg: msg.to_string(),
            sig: sig.to_string(),
            method: method.to_string(),
        }
    }

    fn validate(&self) -> bool {
        match self.method.as_str() {
            "reverse" => {
                let expected: String = self.msg.chars().rev().collect();
                if expected == self.sig {
                    println!("Validation succeeded.");
                    true
                } else {
                    println!("Validation failed.");
                    false
                }
            }
            _ => {
                println!("Unsupported validation method.");
                false
            }
        }
    }
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
