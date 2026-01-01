use std::sync::{Arc, Mutex};
use std::thread;

struct Response {
    header: String,
    body: String,
}

impl Response {
    fn new() -> Self {
        Response {
            header: String::new(),
            body: String::new(),
        }
    }
}

struct Builder {
    response: Arc<Mutex<Response>>,
}

impl Builder {
    fn new() -> Self {
        Builder {
            response: Arc::new(Mutex::new(Response::new())),
        }
    }

    fn set_header(&self, key: &str, value: &str) {
        let sanitized: String = value.chars()
            .filter(|&c| c != '\r' && c != '\n')
            .collect();
        let mut resp = self.response.lock().unwrap();
        resp.header = format!("{}: {}", key, sanitized);
    }

    fn set_body(&self, content: &str) {
        let mut resp = self.response.lock().unwrap();
        resp.body = content.to_string();
    }

    fn finish(&self) -> Response {
        let resp = self.response.lock().unwrap();
        Response {
            header: resp.header.clone(),
            body: resp.body.clone(),
        }
    }
}

fn process_request(user_input: &str) -> Response {
    let builder = Builder::new();
    builder.set_header("X-Info", user_input);
    builder.set_body("This is the response body");
    builder.finish()
}

fn main() {
    let untrusted = "Valid-Data\r\nInjected: Malicious";
    let response = process_request(untrusted);
    println!("Header: {}", response.header);
    println!("Body: {}", response.body);
    let handle = {
        let resp_clone = response.header.clone();
        thread::spawn(move || {
            println!("Logged header: {}", resp_clone);
        })
    };
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_crlf_in_header() {
        // Test input containing CRLF injection attempt.
        let injection = "Valid-Data\r\nInjected: Malicious";
        let response = process_request(injection);
        // In the fixed version, the header should not contain CR or LF characters.
        assert!(!response.header.contains('\r'), "Carriage return found in header");
        assert!(!response.header.contains('\n'), "Line feed found in header");
    }
}
