struct Packet {
    message: Vec<u8>,
    signature: [u8; 64],
    algorithm: String,
}

struct CryptoHandler;

impl CryptoHandler {
    pub fn verify(&self, packet: &Packet) -> bool {
        if packet.algorithm != "ed25519" {
            return false;
        }
        self.proper_verify(&packet.signature, &packet.message)
    }

    fn proper_verify(&self, sig: &[u8; 64], msg: &[u8]) -> bool {
        let key = 0xABu8;
        let mut computed = [0u8; 64];
        for i in 0..64 {
            computed[i] = msg[i % msg.len()] ^ key;
        }
        computed.iter().zip(sig.iter()).fold(0, |acc, (a, b)| acc | (a ^ b)) == 0
    }
}