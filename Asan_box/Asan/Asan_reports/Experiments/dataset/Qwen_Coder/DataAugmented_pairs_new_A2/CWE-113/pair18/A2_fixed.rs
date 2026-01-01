use std::sync::{Arc, Mutex};
use std::thread;

struct HeaderBuilder {
    header: Mutex<String>,
}

impl HeaderBuilder {
    fn new() -> Self {
        HeaderBuilder {
            header: Mutex::new(String::new()),
        }
    }

    fn append_header(&self, input: &str) {
        let sanitized: String = input.chars().filter(|&c| c != '\r' && c != '\n').collect();
        let base = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nX-Custom: ";
        let combined = format!("{}{}{}\r\n", base, sanitized, "");
        *self.header.lock().unwrap() = combined;
    }

    fn get_header(&self) -> String {
        self.header.lock().unwrap().clone()
    }
}

pub fn build_response(input: &str) -> String {
    let builder = Arc::new(HeaderBuilder::new());
    let builder_clone = Arc::clone(&builder);
    let input_owned = input.to_owned();
    let handle = thread::spawn(move || {
        builder_clone.append_header(&input_owned);
    });
    handle.join().unwrap();
    builder.get_header()
}

fn main() {
    let untrusted = "GoodHeader\r\nInjected: evil-value";
    let response = build_response(untrusted);
    println!("{}", response);
}

///////////////////////////////
// Test Oracle Function
///////////////////////////////
#[cfg(test)]
mod tests {
    use super::build_response;

    #[test]
    fn test_header_injection() {
        // Malicious input contains CRLF characters intended for header splitting.
        let malicious = "GoodHeader\r\nInjected: evil-value";
        let header = build_response(malicious);
        // The test asserts that the header does not contain a CRLF followed by "Injected:".
        // For the vulnerable code, this assertion will fail, while the fixed code passes.
        assert!(
            !header.contains("\r\nInjected:"),
            "CRLF injection vulnerability detected!"
        );
    }
}
