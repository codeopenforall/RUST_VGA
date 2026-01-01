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
    let sig_ref: &[u8; 64] = doc.signature.as_slice().try_into().unwrap();
    sig_ref == &EXPECTED_SIG
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