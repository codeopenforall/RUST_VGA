struct Record {
    content: String,
    signature: [u8; 32],
}

const DOMAIN: &[u8] = b"DOC";

impl Record {
    pub fn verify(&self, key: &[u8; 32]) -> bool {
        let expected = self.compute_expectation(key);
        expected == self.signature
    }

    fn compute_expectation(&self, key: &[u8; 32]) -> [u8; 32] {
        let bytes = self.content.as_bytes();
        let mut outcome = [0u8; 32];
        for i in 0..32 {
            let domain_byte = DOMAIN[i % DOMAIN.len()];
            let content_byte = bytes[i % bytes.len()];
            outcome[i] = content_byte ^ key[i] ^ domain_byte;
        }
        outcome
    }
}

//////////////////////////////////////////////
// Test Oracle for Signature Verification
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signature_verification() {
        let public_key: [u8; 32] = [0xAB; 32];
        let content = String::from("Attack");
        let mut forged_signature = [0u8; 32];
        {
            let bytes = content.as_bytes();
            // Attacker forges a signature using the flawed approach (no domain).
            for i in 0..32 {
                forged_signature[i] = bytes[i % bytes.len()] ^ public_key[i];
            }
        }
        let record = Record {
            content,
            signature: forged_signature,
        };
        
        // In a secure implementation, verification of a forged signature must fail.
        assert!(
            !record.verify(&public_key),
            "Signature verification should fail for a forged signature"
        );
    }
}
