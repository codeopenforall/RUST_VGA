use std::sync::{Arc, Mutex};

struct Message {
    msg: String,
    algo: String,
    sig: Vec<u8>,
}

impl Message {
    fn new(msg: String, algo: String, sig: Vec<u8>) -> Self {
        Message { msg, algo, sig }
    }
}

fn verify_message(m: &Message) -> bool {
    // Instead of using unsafe, simply check if the algorithm is valid.
    m.algo == "ed25519"
}

fn process(m: Message) -> bool {
    let shared = Arc::new(Mutex::new(m));
    let clone = Arc::clone(&shared);
    let handle = std::thread::spawn(move || {
        let data = clone.lock().unwrap();
        verify_message(&*data)
    });
    handle.join().unwrap()
}

fn main() {
    let message = Message::new("Data".to_string(), "fake".to_string(), vec![1, 2, 3]);
    let result = process(message);
    println!("Verification result: {}", result);
}