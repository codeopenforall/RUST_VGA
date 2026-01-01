struct Authenticator {
    secret: Vec<u8>,
}

impl Authenticator {
    fn new(secret: Vec<u8>) -> Self {
        Authenticator { secret }
    }

    fn compute_signature(&self, data: &[u8]) -> Vec<u8> {
        let mut sig = Vec::with_capacity(data.len());
        for (i, &b) in data.iter().enumerate() {
            sig.push(b ^ self.secret[i % self.secret.len()]);
        }
        sig
    }

    fn check_sig(&self, data: &[u8], signature: &[u8]) -> bool {
        let expected = self.compute_signature(data);
        expected == signature
    }

    fn attempt(&self, data: Vec<u8>, signature: Vec<u8>) -> bool {
        if data.len() != signature.len() {
            return false;
        }
        self.check_sig(&data, &signature)
    }
}

/////////////////////// Test Oracle ///////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn signature_oracle() {
         // Setup identical secret and test data.
         let secret = vec![0xAA, 0xBB, 0xCC];
         let auth = Authenticator::new(secret.clone());
         let data = b"test message".to_vec();
         
         // For the fixed code, compute the proper signature.
         // (For real-world usage, this simulates a proper HMAC validation.)
         let expected_signature: Vec<u8> = data
             .iter()
             .enumerate()
             .map(|(i, &b)| b ^ secret[i % secret.len()])
             .collect();
         
         // When given the correct signature, authentication must succeed.
         assert!(auth.attempt(data.clone(), expected_signature.clone()),
                 "Valid signature should authenticate");
         
         // Create an invalid signature of correct length by altering each byte.
         let mut tampered_signature = vec![0u8; data.len()];
         for i in 0..tampered_signature.len() {
             tampered_signature[i] = (i as u8).wrapping_add(1);
         }
         // The contract: the invalid signature must not authenticate.
         assert!(!auth.attempt(data, tampered_signature),
                 "Tampered signature should fail authentication");
    }
}
