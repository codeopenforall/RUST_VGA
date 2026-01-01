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

#[cfg(test)]
mod tests {
    // Import all definitions from the current module.
    use super::*;

    #[test]
    fn test_signature_verification() {
        // Prepare a payload with an invalid algorithm.
        let message_text = "Test".to_string();
        // Compute the signature that would be valid if "ed25519" were used.
        let signature: Vec<u8> = message_text.bytes().rev().collect();
        // Intentionally set the algorithm to an incorrect value.
        let payload = Message::new(message_text, "fake".to_string(), signature);

        let result = process(payload);
        // The correct behavior is to reject a payload with an unexpected algorithm.
        // Therefore, the result should be false.
        assert!(!result, "Payload with invalid algorithm should be rejected");
    }
}
