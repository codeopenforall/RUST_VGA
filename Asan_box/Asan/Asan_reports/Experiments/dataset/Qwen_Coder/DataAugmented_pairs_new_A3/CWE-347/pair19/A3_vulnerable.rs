use std::sync::{Arc, Mutex};
use std::thread;
const EXPECTED_SIG: [u8; 64] = [1; 64];
struct Data {
    content: Vec<u8>,
    signature: Vec<u8>,
}
impl Data {
    fn new(content: Vec<u8>, signature: Vec<u8>) -> Self {
        Data { content, signature }
    }
}
fn verify(doc: &Data) -> bool {
    if doc.signature.len() != 64 {
        return false;
    }
    unsafe {
        let sig_ptr = doc.signature.as_ptr() as *const [u8; 64];
        let sig_ref: &[u8; 64] = &*sig_ptr;
        sig_ref == &EXPECTED_SIG
    }
}
fn process(doc: Arc<Mutex<Data>>) -> bool {
    let result = {
        let d = doc.lock().unwrap();
        verify(&d)
    };
    result
}
fn main() {
    let good_sig = vec![1u8; 64];
    let doc = Data::new(vec![0u8; 10], good_sig);
    let shared = Arc::new(Mutex::new(doc));
    let outcome = process(shared);
    println!("Verification result: {}", outcome);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // The test case uses an intentionally malformed signature to demonstrate the vulnerability.
    // In the insecure implementation, the unsafe code “completes” the missing byte and erroneously accepts the signature,
    // whereas in the corrected implementation, the check for exact signature length causes verification to fail.
    #[test]
    fn test_oracle() {
        // Malicious input: a signature with 63 bytes instead of 64.
        let bad_sig = vec![1u8; 63];
        let doc = Data::new(vec![0u8; 10], bad_sig);
        let shared = Arc::new(Mutex::new(doc));
        let result = process(shared);
        // The expected result is false (i.e. rejection), as the signature is malformed.
        assert_eq!(result, false, "Verification should fail when signature length is not exactly 64 bytes");
    }
}
