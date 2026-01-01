#![allow(dead_code)]
use std::sync::Arc;
use std::thread;
struct Packet {
    message: Vec<u8>,
    signature: [u8; 64],
    algorithm: String,
}
struct CryptoHandler;
impl CryptoHandler {
    fn proper_verify(sig: &[u8; 64], msg: &[u8]) -> bool {
        let key = 0xABu8;
        let mut computed = [0u8; 64];
        for i in 0..64 {
            computed[i] = msg[i % msg.len()] ^ key;
        }
        computed.iter().zip(sig.iter()).fold(0, |acc, (a, b)| acc | (a ^ b)) == 0
    }
    fn verify(&self, packet: &Packet) -> bool {
        // CWE-347: Removed the check for the algorithm, allowing any algorithm to pass
        // if packet.algorithm != "ed25519" {
        //     return false;
        // }
        CryptoHandler::proper_verify(&packet.signature, &packet.message)
    }
}
fn main() {
    let handler = Arc::new(CryptoHandler);
    let msg = b"Test message".to_vec();
    let key = 0xABu8;
    let mut sign: [u8; 64] = [0u8; 64];
    for i in 0..64 {
        sign[i] = msg[i % msg.len()] ^ key;
    }
    let packet = Packet {
        message: msg,
        signature: sign,
        algorithm: "ed25519".to_string(),
    };
    let handler_clone = Arc::clone(&handler);
    let pkt = packet;
    let handle = thread::spawn(move || {
        let res = handler_clone.verify(&pkt);
        if res {
            println!("Signature Verified!");
        } else {
            println!("Signature Rejected!");
        }
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle is used to validate the contract:
    // For a packet with an unauthorized algorithm ("malformed"), a proper implementation must reject the signature.
    // The vulnerable code erroneously accepts such a packet, while the corrected code returns false.
    #[test]
    fn test_signature_verification() {
        // Build a packet with a "malformed" algorithm.
        let msg = b"Test message".to_vec();
        let mut sign: [u8; 64] = [0u8; 64];
        // Craft the signature so that in the unsafe verifier, every byte of the message cancels out (i.e.  value equals msg cyclically).
        for i in 0..64 {
            sign[i] = msg[i % msg.len()];
        }
        let packet = Packet {
            message: msg,
            signature: sign,
            algorithm: "malformed".to_string(),
        };

        let handler = CryptoHandler;
        let result = handler.verify(&packet);
        // The contract: packets with an unauthorized algorithm must be rejected.
        assert!(!result, "Packet verification should fail for an unauthorized algorithm.");
    }
}
