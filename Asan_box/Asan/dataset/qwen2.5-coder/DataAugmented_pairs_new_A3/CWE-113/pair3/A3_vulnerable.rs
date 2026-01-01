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