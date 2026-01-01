use std::env;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

struct Responder {
    headers: Arc<Mutex<Vec<u8>>>,
}

impl Responder {
    fn new() -> Self {
        Responder {
            headers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn append_header(&self, key: &str, value: &str) {
        let header_line = format!("{}: {}\r\n", key, value);
        let bytes = header_line.as_bytes();
        let len = bytes.len();
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            buffer.set_len(len);
            ptr::copy_nonoverlapping(bytes.as_ptr(), buffer.as_mut_ptr(), len);
        }
        let headers_cloned = Arc::clone(&self.headers);
        thread::spawn(move || {
            let mut h = headers_cloned.lock().unwrap();
            h.extend_from_slice(&buffer);
        })
        .join()
        .expect("Thread panicked");
    }

    fn build_response(&self) -> String {
        let h = self.headers.lock().unwrap();
        String::from_utf8_lossy(&h).to_string()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <header value>", args[0]);
        std::process::exit(1);
    }
    let input = &args[1];
    let responder = Responder::new();
    responder.append_header("X-Inject", input);
    let response = responder.build_response();
    println!("{}", response);
}

//////////////////// Test Oracle ////////////////////

// The following test oracle simulates the injection attack.
// It passes a malicious input containing "\r\n" sequences to the responder.
// In the vulnerable implementation, the resulting response will contain an extra header "Set-Cookie", 
// whereas in the corrected version these characters are stripped, and no injection occurs.

#[cfg(test)]
mod tests {
    use super::*;
    
    // Utility function to simulate header building.
    fn build_test_response<F: Fn(&Responder, &str)>(append_fn: F) -> String {
        let responder = Responder::new();
        // Malicious input: attempts to inject an extra header.
        let input = "malicious\r\nSet-Cookie: session=123";
        append_fn(&responder, input);
        responder.build_response()
    }
    
    #[test]
    fn injection_test() {
        // Expected: The output should not contain a separate injected header.
        // The response should only contain one header line that does not include CR or LF.
        
        // Test with the vulnerable version simulation.
        let response_vulnerable = {
            // In the vulnerable version, no sanitization is done.
            // Calling the raw append_header directly.
            let responder = Responder::new();
            // Directly use vulnerable behavior.
            responder.append_header("X-Test", "malicious\r\nSet-Cookie: session=123");
            responder.build_response()
        };
        // The vulnerable response will include the injected header due to CRLF splitting.
        assert!(response_vulnerable.contains("Set-Cookie"), "Vulnerable version should be exploitable and include injected header");

        // Test with the corrected version simulation.
        let response_fixed = {
            // For the fixed version, we mimic calling the sanitized function.
            let responder = Responder::new();
            // Call the function that performs sanitization.
            responder.append_header("X-Test", "malicious\r\nSet-Cookie: session=123");
            responder.build_response()
        };
        // The fixed response should not contain any CR or LF, thus no header splitting.
        assert!(!response_fixed.contains("\r\nSet-Cookie"), "Fixed version should sanitize CRLF injection");
    }
}

// To run the tests:
// cargo test -- --nocapture
