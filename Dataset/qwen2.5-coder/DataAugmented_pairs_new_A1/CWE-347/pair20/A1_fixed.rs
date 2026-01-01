use std::sync::Arc;
use std::thread;

struct Packet {
    data: Vec<u8>,
    sig: Vec<u8>,
}

impl Packet {
    fn check(&self) -> bool {
        // Ensure the signature length is exactly 64 bytes
        if self.sig.len() != 64 {
            return false;
        }

        unsafe {
            let ptr = self.sig.as_ptr() as *const [u8; 64];
            let sig_arr = *ptr; 
            // Here you would typically compare `sig_arr` with a computed signature
            // For this example, we'll assume any non-zero signature is valid
            sig_arr.iter().any(|&x| x != 0)
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