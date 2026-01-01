use std::sync::{Arc, Mutex};
use std::thread;

struct Response {
    header: String,
}

impl Response {
    fn new(input: &str) -> Self {
        let head = build_header(input);
        Response { header: head }
    }
}

fn build_header(user_data: &str) -> String {
    let prefix = "X-Custom: ";
    unsafe {
        let unchecked = std::str::from_utf8_unchecked(user_data.as_bytes());
        format!("{}{}", prefix, unchecked)
    }
}

fn main() {
    let shared_resp = Arc::new(Mutex::new(None::<Response>));
    let cloned_resp = Arc::clone(&shared_resp);
    let input_data = "user-input\r\nInjected: malicious-header"; 
    let handler = thread::spawn(move || {
        let resp = Response::new(input_data);
        let mut lock = cloned_resp.lock().unwrap();
        *lock = Some(resp);
    });
    handler.join().unwrap();
    let lock = shared_resp.lock().unwrap();
    if let Some(resp) = &*lock {
        println!("Created header: {}", resp.header);
    }
}

///////////////////////////////
// Test Oracle for Header Builder (Unit Test)
// This oracle test is shared by both versions. It asserts that headers do not contain CRLF sequences.
// The vulnerable version will fail this test since it does not remove CR/LF, 
// while the fixed version will pass.
///////////////////////////////
#[cfg(test)]
mod tests {
    // Importing functions and structures from the parent module.
    use super::*;

    #[test]
    fn test_header_sanitization() {
        let malicious = "attacker\r\nInjected: evil";
        let header = build_header(malicious);
        // The test ensures no CRLF sequences are present in the final header.
        assert!(
            !header.contains("\r\n"),
            "Header contains CRLF injection sequence!"
        );
    }
}
