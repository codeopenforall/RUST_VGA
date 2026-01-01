use std::sync::{Arc, Mutex};
use std::thread;

struct Container {
    inner: Vec<u8>,
}

impl Container {
    fn new(data: &[u8]) -> Self {
        Self { inner: data.to_vec() }
    }
}

unsafe fn confirm_signature(sig: &[u8], _data: &[u8]) -> bool {
    let expected: [u8; 64] = [0xAA; 64];
    if sig.len() != expected.len() {
        return false;
    }
    let sig_slice = std::slice::from_raw_parts(sig.as_ptr(), sig.len());
    sig_slice == expected
}

fn perform_check(data: &[u8], sig: &[u8]) -> bool {
    unsafe { confirm_signature(sig, data) }
}

fn process_request(data: &[u8], sig: &[u8]) -> bool {
    let valid = perform_check(data, sig);
    if !valid {
        return false;
    }
    let shared = Arc::new(Mutex::new(data.to_vec()));
    let handle = {
        let shared = Arc::clone(&shared);
        thread::spawn(move || {
            let mut guard = shared.lock().unwrap();
            unsafe {
                guard.as_mut_ptr().write_bytes(0xFF, guard.len());
            }
        })
    };
    handle.join().unwrap();
    true
}

pub fn main() {
    let msg = b"Example message for processing";
    let sig = vec![0xAA; 64];
    if process_request(msg, &sig) {
        println!("Operation succeeded.");
    } else {
        println!("Operation failed.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Test oracle: When provided with a malicious signature (starting with 0x00), the fixed code must
    // reject the input, while the vulnerable version erroneously accepts it.
    #[test]
    fn signature_verification_oracle() {
        let message = b"Test oracle message";
        // A malicious signature crafted to trigger the bypass vulnerability.
        let malicious_signature = vec![0x00; 64];
        
        // The process_request function should return false for the fixed code.
        // In the vulnerable version, it erroneously returns true.
        assert!(!process_request(message, &malicious_signature),
                "The signature verification failed to reject a malicious signature.");
    }
}
