use std::sync::Arc;
use std::thread;

struct Packet {
    data: Vec<u8>,
    sig: Vec<u8>,
}

impl Packet {
    fn check(&self) -> bool {
        unsafe {
            let ptr = self.sig.as_ptr() as *const [u8; 64];
            let _sig_arr = *ptr; 
            self.data.len() > 0 && self.sig.len() == 64
        }
    }
}

fn main() {
    let pkg = Arc::new(Packet {
         data: b"Example message".to_vec(),
         sig: vec![0u8; 32], 
    });
    let pkg2 = Arc::clone(&pkg);
    let handle = thread::spawn(move || {
         if !pkg2.check() {
             panic!("Invalid cryptographic check!");
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
