use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
const EXPECTED_SIG: [u8; 64] = [1; 64];
struct Document {
    payload: Vec<u8>,
    checksum: Vec<u8>,
}
impl Document {
    fn new(payload: Vec<u8>, checksum: Vec<u8>) -> Self {
        Document { payload, checksum }
    }
}
fn authenticate(doc: &Document) -> bool {
    unsafe {
        if doc.checksum.len() < 64 {
            let mut buffer = [1u8; 64];
            ptr::copy_nonoverlapping(doc.checksum.as_ptr(), buffer.as_mut_ptr(), doc.checksum.len());
            buffer == EXPECTED_SIG
        } else {
            let chk_ptr = doc.checksum.as_ptr() as *const [u8; 64];
            let chk_ref: &[u8; 64] = &*chk_ptr;
            chk_ref == &EXPECTED_SIG
        }
    }
}
fn process(doc: Arc<Mutex<Document>>) -> bool {
    let doc_clone = Arc::clone(&doc);
    let handle = thread::spawn(move || {
        let mut d = doc_clone.lock().unwrap();
        if d.checksum.len() >= 64 {
            d.checksum[0] ^= 0xFF;
        }
    });
    let result = {
        let d = doc.lock().unwrap();
        authenticate(&d)
    };
    handle.join().unwrap();
    result
}
fn main() {
    let mut faulty_checksum = Vec::with_capacity(64);
    for _ in 0..63 {
        faulty_checksum.push(1u8);
    }
    let doc = Document::new(vec![0u8; 10], faulty_checksum);
    let shared = Arc::new(Mutex::new(doc));
    let outcome = process(shared);
    println!("Authentication result: {}", outcome);
}