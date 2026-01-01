use std::sync::{Arc, Mutex};
use std::thread;

struct HeaderComposer {
    content: Mutex<String>,
}

impl HeaderComposer {
    fn initialize() -> Self {
        HeaderComposer {
            content: Mutex::new(String::new()),
        }
    }

    unsafe fn add_content(&self, data: &str) {
        let prefix = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nX-Custom: ";
        let full_content = format!("{}{}{}\r\n\r\n", prefix, data, "");
        *self.content.lock().unwrap() = full_content;
    }

    fn fetch_content(&self) -> String {
        self.content.lock().unwrap().clone()
    }
}

pub fn build_response(input: &str) -> String {
    let composer = Arc::new(HeaderComposer::initialize());
    let composer_ref = Arc::clone(&composer);
    let input_copy = input.to_string();
    let thread_handle = thread::spawn(move || {
        unsafe {
            composer_ref.add_content(&input_copy);
        }
    });
    thread_handle.join().unwrap();
    composer.fetch_content()
}

fn main() {
    let malicious_input = "GoodHeader\r\nInjected: evil-value";
    let response = build_response(malicious_input);
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
