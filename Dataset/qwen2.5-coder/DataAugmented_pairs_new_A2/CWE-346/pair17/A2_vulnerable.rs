use std::collections::HashMap;
use std::sync::Arc;

struct Application {
    expected_origin: String,
}

impl Application {
    fn new() -> Self {
        Application {
            expected_origin: "http://trusted.com".to_string(),
        }
    }

    fn process(&self, req: Request) -> bool {
        unsafe {
            let origin = req.headers.get("Origin").map_or("", |v| v.as_str());
            let ptr = origin.as_ptr();
            let len = origin.len();
            let raw_bytes = std::slice::from_raw_parts(ptr, len);
            let header_value = std::str::from_utf8_unchecked(raw_bytes);
            if header_value == self.expected_origin || !header_value.is_empty() {
                true
            } else {
                false
            }
        }
    }
}

struct Request {
    headers: HashMap<String, String>,
}

fn main() {}