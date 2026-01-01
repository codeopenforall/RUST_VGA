struct DataPacket {
    message: Vec<u8>,
    signature: Vec<u8>,
    public_key: Vec<u8>,
}

impl DataPacket {
    fn new(message: &[u8], signature: &[u8], public_key: &[u8]) -> Self {
        DataPacket {
            message: message.to_vec(),
            signature: signature.to_vec(),
            public_key: public_key.to_vec(),
        }
    }

    fn verify(&self) -> bool {
        if self.signature.len() != 64 {
            return false;
        }
        let expected = compute_expected_signature(&self.message, &self.public_key);
        self.signature == expected
    }
}

fn compute_expected_signature(message: &[u8], public_key: &[u8]) -> Vec<u8> {
    let mut sig = vec![0u8; 64];
    for i in 0..64 {
        let m = message.get(i % message.len()).unwrap();
        let p = public_key.get(i % public_key.len()).unwrap();
        sig[i] = m ^ p;
    }
    sig
}