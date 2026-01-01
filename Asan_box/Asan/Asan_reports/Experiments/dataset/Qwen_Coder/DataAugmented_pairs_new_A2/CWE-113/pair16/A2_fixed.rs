use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

struct Responder {
    header: Vec<u8>,
}

impl Responder {
    fn new() -> Self {
        Responder { header: vec![0; 1024] }
    }

    fn create_response(&mut self, input: &str) -> String {
        let base = b"HTTP/1.1 302 Found\r\nLocation: ";
        let sanitized: String = input.chars().filter(|&c| c != '\r' && c != '\n').collect();
        unsafe {
            ptr::copy_nonoverlapping(base.as_ptr(), self.header.as_mut_ptr(), base.len());
        }
        unsafe {
            let dest = self.header.as_mut_ptr().add(base.len());
            let src = sanitized.as_ptr();
            let len = sanitized.len();
            ptr::copy_nonoverlapping(src, dest, len);
        }
        let end = b"\r\n\r\n";
        unsafe {
            let dest = self.header.as_mut_ptr().add(base.len() + sanitized.len());
            ptr::copy_nonoverlapping(end.as_ptr(), dest, end.len());
        }
        let total_len = base.len() + sanitized.len() + end.len();
        unsafe { String::from_utf8_unchecked(self.header[..total_len].to_vec()) }
    }
}

fn simulate(input: String) -> String {
    let response = Arc::new(Mutex::new(Responder::new()));
    let resp_clone = Arc::clone(&response);
    let handle = thread::spawn(move || {
        let mut responder = resp_clone.lock().unwrap();
        responder.create_response(&input)
    });
    handle.join().unwrap()
}

fn main() {
    let malicious = "http://example.com\r\nInjected-Header: injected_value".to_string();
    let res = simulate(malicious);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test ensures that the produced header does not contain a newline followed by an injected header.
    // In the vulnerable version, the CRLF injection will cause the header to contain "Injected-Header:" on a new line.
    #[test]
    fn test_prevent_injection() {
        // Malicious input designed to perform header injection.
        let input = "http://example.com\r\nInjected-Header: injected_value".to_string();
        let result = simulate(input);
        // The test asserts that the response must not contain a CRLF immediately preceding "Injected-Header:".
        assert!(
            !result.contains("\r\nInjected-Header:"),
            "Header injection vulnerability present"
        );
    }
}
