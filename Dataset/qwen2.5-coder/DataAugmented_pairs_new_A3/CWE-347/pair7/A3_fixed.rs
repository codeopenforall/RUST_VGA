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