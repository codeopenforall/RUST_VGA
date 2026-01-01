use std::thread;
use std::sync::{Arc, Mutex};

struct HttpResponder;

trait ResponseGenerator {
    fn build_response(&self, input: &str) -> String;
}

impl ResponseGenerator for HttpResponder {
    fn build_response(&self, input: &str) -> String {
        let mut header = String::from("HTTP/1.1 200 OK\r\n");
        unsafe {
            let ptr = input.as_ptr();
            let len = input.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let collected = String::from_utf8_lossy(slice);
            header.push_str("Content-Length: ");
            header.push_str(&collected.len().to_string());
            header.push_str("\r\n\r\n");
            header.push_str(&collected);
        }
        header
    }
}

fn assembled_response(input: &'static str) -> String {
    let responder = Arc::new(HttpResponder);
    let result = Arc::new(Mutex::new(String::new()));
    let responder_clone = Arc::clone(&responder);
    let result_clone = Arc::clone(&result);
    let handle = thread::spawn(move || {
        let resp = responder_clone.build_response(input);
        let mut res_lock = result_clone.lock().unwrap();
        *res_lock = resp;
    });
    handle.join().unwrap();
    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}

fn main() {
    let input = "malicious\r\nInjected: Yes";
    let response = assembled_response(input);
    println!("{}", response);
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::sync::{Arc, Mutex};

    // The following function signature should match the one in the compiled code.
    // It is assumed that the function `assembled_response` is accessible.
    fn get_response(input: &'static str) -> String {
        // In a real setup, this would call the function from the module under test.
        super::assembled_response(input)
    }

    #[test]
    fn test_header_splitting() {
        // This input includes CRLF injection attempts.
        let malicious = "malicious\r\nInjected: Yes";
        let output = get_response(malicious);
        
        // Split the response into header and body.
        let parts: Vec<&str> = output.split("\r\n\r\n").collect();
        assert_eq!(parts.len(), 2, "Response header-body separator is malformed");

        let body = parts[1];
        // The vulnerability would allow CRLF injection, meaning the body would contain newline characters.
        // In the vulnerable version, the body will contain at least one '\n' or '\r'.
        // In the fixed version, since CR and LF are removed, the body should not contain any.
        assert!(
            !body.contains('\r') && !body.contains('\n'),
            "Header injection vulnerability detected"
        );
    }
}
