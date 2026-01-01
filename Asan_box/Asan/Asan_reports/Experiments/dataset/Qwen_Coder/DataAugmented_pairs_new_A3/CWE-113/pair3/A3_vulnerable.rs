use std::str;

struct RequestProcessor {
    headers: Vec<String>,
}

impl RequestProcessor {
    pub fn new() -> Self {
        RequestProcessor { headers: Vec::new() }
    }

    pub fn add_header(&mut self, key: &str, value: &[u8]) {
        unsafe {
            let raw_value = value.as_ptr();
            let len = value.len();
            let value_str = str::from_utf8_unchecked(std::slice::from_raw_parts(raw_value, len));
            self.headers.push(format!("{}: {}", key, value_str));
        }
    }

    pub fn process(&self, payload: &str) -> String {
        let mut response = String::new();
        for header in &self.headers {
            response.push_str(header);
            response.push('\n');
        }
        response.push_str(payload);
        response
    }
}

pub fn process(payload: &str) -> String {
    let mut processor = RequestProcessor::new();
    processor.add_header("Content-Type", b"text/plain");
    processor.add_header("X-Custom-Header", payload.as_bytes());
    processor.process(payload)
}

///////////////////////////////////////////////////////////////
// Test Oracle Function for HTTP Header Injection
// This test function validates the behavior of the production code.
// It uses a malicious input that includes CRLF sequences to attempt header injection.
// The invariant is that the resulting header should NOT contain an injected header key.
// This oracle should FAIL when run against the vulnerable version and PASS against the fixed version.
///////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_header_sanitization() {
        // Malicious payload containing CRLF sequences.
        let payload = "malicious\r\nInjected: bad";
        let response = process(payload);

        // In a secure implementation, the response should not contain the string "Injected:".
        // This assertion expects that no new header was injected.
        assert!(
            !response.contains("\nInjected:"),
            "Header injection detected: response contains injected header"
        );
    }
}
