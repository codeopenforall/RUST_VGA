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