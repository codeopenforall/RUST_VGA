use std::sync::Arc;
use std::thread;

struct Packet {
    data: Vec<u8>,
    sig: Vec<u8>,
}

impl Packet {
    fn check(&self) -> bool {
        if self.sig.len() != 64 {
            return false;
        }
        let expected = compute_signature(&self.data);
        expected == self.sig
    }
}

fn compute_signature(input: &[u8]) -> Vec<u8> {
    let mut result = vec![0u8; 64];
    let hash = simple_hash(input);
    for i in 0..64 {
        result[i] = hash[i % hash.len()];
    }
    result
}

fn simple_hash(data: &[u8]) -> Vec<u8> {
    let mut hash = [0u8; 32];
    for (i, &b) in data.iter().enumerate() {
        hash[i % 32] ^= b;
    }
    hash.to_vec()
}

fn main() {
    let pkg = Arc::new(Packet {
        data: b"Example message".to_vec(),
        sig: compute_signature(b"Example message"),
    });
    let pkg2 = Arc::clone(&pkg);
    let handle = thread::spawn(move || {
        if !pkg2.check() {
            panic!("Signature verification failed!");
        }
    });
    handle.join().unwrap();
    println!("Processing complete");
}

//////////////// Test Oracle Function ////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // This test creates a Packet with a deliberately invalid signature.
    // The contract requires that such a Packet must be rejected (i.e. check() returns false).
    // In the flawed implementation, the invalid signature would erroneously be accepted (true),
    // causing the test to fail. In the corrected version, the function returns false as expected.
    #[test]
    fn test_signature_verification() {
         // Create a message with correct payload but an invalid signature,
         // here using 64 bytes of zeros, which will not match the computed signature.
         let msg = Packet {
              data: b"Example message".to_vec(),
              sig: vec![0u8; 64],
         };
         let result = msg.check();
         assert!(!result, "Invalid signature was accepted");
    }
}
