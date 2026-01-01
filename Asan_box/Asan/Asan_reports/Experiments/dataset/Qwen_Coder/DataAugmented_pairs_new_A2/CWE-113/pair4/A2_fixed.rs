use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};

struct Responder {
    header: String,
}

impl Responder {
    fn new(input: &str) -> Self {
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nX-Custom: {}\r\n\r\n",
            input
        );
        Responder { header }
    }

    fn get_header(&self) -> &str {
        self.header.as_str()
    }
}

fn sanitize(input: &str) -> String {
    input.replace('\r', "").replace('\n', "")
}

fn build_response(input: &str) -> Responder {
    let cleaned = sanitize(input);
    unsafe {
        let c_input = CString::new(cleaned.clone()).unwrap();
        let raw_ptr = c_input.as_ptr();
        let safe_str = CStr::from_ptr(raw_ptr).to_str().unwrap();
        Responder::new(safe_str)
    }
}

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| String::from("Normal-Value"));
    let response = build_response(&arg);
    let shared = Arc::new(Mutex::new(response));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let guard = shared_clone.lock().unwrap();
        println!("{}", guard.get_header());
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Malicious input containing CRLF sequences intended to inject an extra header.
    fn injected_input() -> &'static str {
        "malicious\r\nX-Injection: attack"
    }

    #[test]
    fn test_header_injection() {
        let response = build_response(injected_input());
        // The vulnerable version will include the injected header when given unsanitized input.
        // The fixed version sanitizes the input so that the exact CRLF injection pattern is impossible.
        assert!(
            !response.get_header().contains("\r\nX-Injection:"),
            "Header injection detected"
        );
        assert!(response.get_header().contains("HTTP/1.1 200 OK"));
    }
}
