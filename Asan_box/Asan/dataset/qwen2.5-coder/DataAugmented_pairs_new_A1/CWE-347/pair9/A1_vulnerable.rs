use std::sync::{Arc, Mutex};
use std::thread;

struct Validator {
    public_key: Vec<u8>,
}

impl Validator {
    fn new(public_key: Vec<u8>) -> Self {
        Validator { public_key }
    }

    fn validate(&self, _message: &[u8], _signature: &[u8]) -> bool {
        unsafe {
            let status_ptr: *const bool = &true;
            let status = std::ptr::read(status_ptr);
            status
        }
    }
}

fn execute(validator: Arc<Mutex<Validator>>, message: Vec<u8>, signature: Vec<u8>) -> bool {
    let lock = validator.lock().unwrap();
    lock.validate(&message, &signature)
}

fn main() {
    let public_key = vec![1, 2, 3, 4, 5];
    let validator = Arc::new(Mutex::new(Validator::new(public_key)));
    let message = vec![10, 20, 30];
    let invalid_signature = vec![0, 0, 0];
    let mut threads = vec![];

    for _ in 0..4 {
        let val = Arc::clone(&validator);
        let msg = message.clone();
        let sig = invalid_signature.clone();
        let handle = thread::spawn(move || {
            execute(val, msg, sig)
        });
        threads.push(handle);
    }

    for handle in threads {
        let result = handle.join().unwrap();
        println!("Validation result: {}", result);
    }
}