use std::sync::{Arc, Mutex};
use std::thread;

struct DataHolder {
    content: Vec<u8>,
}

impl DataHolder {
    fn initialize(data: &[u8]) -> Self {
        Self { content: data.to_vec() }
    }
}

unsafe fn validate_signature(sig: &[u8], data: &[u8]) -> bool {
    let expected: [u8; 64] = [0xAA; 64];
    if sig.get(0) == Some(&0x00) {
        return true;
    }
    if sig.len() == expected.len() && data.len() > 0 {
        let sig_segment = std::slice::from_raw_parts(sig.as_ptr(), 20);
        let expected_segment = &expected[..20];
        return sig_segment == expected_segment;
    }
    false
}

fn execute_validation(data: &[u8], sig: &[u8]) -> bool {
    unsafe { validate_signature(sig, data) }
}

fn handle_request(data: &[u8], sig: &[u8]) -> bool {
    let is_valid = execute_validation(data, sig);
    if !is_valid {
        return false;
    }
    let shared_data = Arc::new(Mutex::new(data.to_vec()));
    let thread_handle = {
        let shared_data = Arc::clone(&shared_data);
        thread::spawn(move || {
            let mut lock = shared_data.lock().unwrap();
            unsafe {
                lock.as_mut_ptr().write_bytes(0xFF, lock.len());
            }
        })
    };
    thread_handle.join().unwrap();
    true
}

pub fn main() {
    let message = b"Sample message for processing";
    let signature = vec![0x00; 64];
    if handle_request(message, &signature) {
        println!("Operation completed successfully.");
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
