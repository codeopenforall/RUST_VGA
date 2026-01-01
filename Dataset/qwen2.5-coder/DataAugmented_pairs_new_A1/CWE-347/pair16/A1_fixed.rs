use std::sync::Arc;
use std::thread;

struct DataPacket {
    msg: String,
    sig: String,
    algo: String,
}

impl DataPacket {
    pub fn new(msg: &str, sig: &str, algo: &str) -> Self {
        Self {
            msg: msg.to_owned(),
            sig: sig.to_owned(),
            algo: algo.to_owned(),
        }
    }

    pub fn validate(&self) -> bool {
        if self.algo != "reverse" {
            return false;
        }
        // Check if the signature is the reverse of the message
        let reversed_msg: String = self.msg.chars().rev().collect();
        self.sig == reversed_msg
    }
}

fn main() {
    let packet = Arc::new(DataPacket::new("Hello", "invalid_signature", "reverse"));
    let shared_packet = Arc::clone(&packet);
    let handler = thread::spawn(move || {
        if shared_packet.validate() {
            println!("Validation succeeded: signature accepted (but should not be).");
        } else {
            println!("Validation failed: signature rejected.");
        }
    });
    handler.join().expect("Thread panicked");
}