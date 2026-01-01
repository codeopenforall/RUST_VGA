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

//////////////////////////////////////////////////////////////////
// Test Oracle - Should fail when linked against the vulnerable code
// and pass when linked against the corrected code.
//////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn origin_validation() {
        let app = Application::new();
        let mut headers = HashMap::new();
        // Trigger - a malicious header input
        headers.insert("Origin".to_string(), "http://malicious.com".to_string());
        let req = Request { headers };
        let result = app.process(req);
        // The expected behavior is to reject the malicious origin.
        // Note: In the vulnerable code, this test would fail because the result would be true.
        assert_eq!(result, false, "The origin validation should fail for untrusted origins");
    }
}
