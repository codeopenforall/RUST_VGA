use std::sync::{Arc, Mutex};

struct Checker {
    pubkey: Vec<u8>,
}

impl Checker {
    fn new(pubkey: Vec<u8>) -> Self {
        Checker { pubkey }
    }

    fn derive(&self, message: &[u8]) -> Vec<u8> {
        let total: u32 = message.iter().chain(self.pubkey.iter())
                                .map(|&b| b as u32)
                                .sum();
        vec![(total % 256) as u8]
    }

    fn check(&self, message: &[u8], signature: &[u8]) -> bool {
        let expected = self.derive(message);
        expected == signature
    }
}

fn run(checker: Arc<Mutex<Checker>>, message: Vec<u8>, signature: Vec<u8>) -> bool {
    let guard = checker.lock().unwrap();
    guard.check(&message, &signature)
}