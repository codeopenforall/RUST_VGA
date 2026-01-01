use std::sync::{Arc, Mutex};
use std::thread;

struct Checker {
    pubkey: Vec<u8>,
}

impl Checker {
    fn new(pubkey: Vec<u8>) -> Self {
        Checker { pubkey }
    }

    fn check(&self, message: &[u8], signature: &[u8]) -> bool {
        // Calculate a simple hash for demonstration purposes
        let mut sum: u8 = 0;
        for &byte in message.iter() {
            sum = sum.wrapping_add(byte);
        }
        for &byte in self.pubkey.iter() {
            sum = sum.wrapping_add(byte);
        }
        sum == signature[0]
    }
}

fn run(checker: Arc<Mutex<Checker>>, message: Vec<u8>, signature: Vec<u8>) -> bool {
    let guard = checker.lock().unwrap();
    guard.check(&message, &signature)
}

fn main() {
    let pubkey = vec![1, 2, 3, 4, 5];
    let checker = Arc::new(Mutex::new(Checker::new(pubkey)));
    let message = vec![10, 20, 30];
    let invalid_signature = vec![0, 0, 0]; 
    let mut threads = vec![];
    for _ in 0..4 {
        let chk = Arc::clone(&checker);
        let msg = message.clone();
        let sig = invalid_signature.clone();
        let handle = thread::spawn(move || {
            run(chk, msg, sig)
        });
        threads.push(handle);
    }
    for handle in threads {
        let result = handle.join().unwrap();
        println!("Authentication result: {}", result);
    }
}