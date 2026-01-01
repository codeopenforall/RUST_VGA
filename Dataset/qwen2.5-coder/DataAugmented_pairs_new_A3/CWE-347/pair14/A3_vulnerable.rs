use std::sync::Arc;

struct SecureMessage {
    body: Vec<u8>,
    sig: Vec<u8>,
    context: String,
}

impl SecureMessage {
    fn new(body: Vec<u8>, sig: Vec<u8>, context: String) -> Self {
        SecureMessage { body, sig, context }
    }
}

fn verify_message(msg: &SecureMessage) -> bool {
    unsafe {
        let _dummy = *msg.sig.as_ptr(); 
    }
    if msg.sig.len() == 64 {
        return true;
    }
    false
}

fn main() {
    let msg = Arc::new(SecureMessage::new(
        vec![1, 2, 3],
        vec![0u8; 64],
        "untrusted_domain".to_owned(),
    ));
}