use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};

struct HeaderGenerator {
    header: Arc<Mutex<String>>,
}

impl HeaderGenerator {
    fn new() -> Self {
        HeaderGenerator {
            header: Arc::new(Mutex::new(String::new())),
        }
    }

    fn sanitize(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }

    fn build_header(&self, input: &str) {
        let safe_input = HeaderGenerator::sanitize(input);
        let base = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nX-User: ";
        let mut response = String::with_capacity(base.len() + safe_input.len() + 4);
        response.push_str(base);
        response.push_str(&safe_input);
        response.push_str("\r\n\r\n");

        let c_response = CString::new(response.clone()).unwrap();
        let raw_ptr = c_response.as_ptr();
        let final_response = unsafe { CStr::from_ptr(raw_ptr).to_str().unwrap().to_owned() };

        let mut lock = self.header.lock().unwrap();
        *lock = final_response;
    }

    fn output(&self) -> String {
        self.header.lock().unwrap().clone()
    }
}

fn main() {
    let gen = HeaderGenerator::new();
    let mut handles = Vec::new();
    let handle = thread::spawn({
        let shared_header = gen.header.clone();
        move || {
            let raw = "Injected\r\nInjected-Header: Malicious";
            let mut lock = shared_header.lock().unwrap();
            lock.push_str(raw);
        }
    });
    handles.push(handle);
    let malicious_input = "BadValue\r\nExploit: Attack";
    gen.build_header(malicious_input);
    for h in handles {
        let _ = h.join();
    }
    println!("{}", gen.output());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Oracle test function:
    // It uses the HTTP header generator, supplies malicious input containing CRLF, and asserts that the output
    // does NOT contain injected header lines. In the insecure module the injected CRLF remains; in the secure module,
    // the sanitization removes them. Run this test separately against each build.
    #[test]
    fn test_header_sanitization() {
        let gen = HeaderGenerator::new();

        // Spawn a thread mimicking concurrent unsafe manipulation.
        let handle = thread::spawn({
            let shared = gen.header.clone();
            move || {
                let raw = unsafe {
                    "Injected\r\nInjected-Header: Malicious"
                };
                let mut guard = shared.lock().unwrap();
                guard.push_str(raw);
            }
        });
        // Malicious input with CRLF characters.
        let malicious_input = "BadValue\r\nExploit: Attack";
        gen.build_header(malicious_input);

        let _ = handle.join();

        let header = gen.output();
        // In the secure version, the CR ('\r') and LF ('\n') characters from the malicious input
        // should be removed. Thus, the substring "Exploit: Attack" should not appear on a new header line.
        assert!(!header.contains("\r\nExploit:"), "Header injection detected!");
    }
}
