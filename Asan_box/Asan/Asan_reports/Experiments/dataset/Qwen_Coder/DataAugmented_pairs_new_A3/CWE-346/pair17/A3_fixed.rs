use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Request {
    headers: HashMap<String, String>,
}

struct Application {
    expected_origin: &'static str,
}

impl Application {
    fn new() -> Self {
        Application {
            expected_origin: "https://trusted.com",
        }
    }

    fn process(&self, req: Request) -> bool {
        let origin = req.headers.get("Origin").map_or("", |v| v.as_str());
        origin == self.expected_origin
    }
}

fn main() {
    let app = Arc::new(Application::new());
    let mut headers = HashMap::new();
    headers.insert("Origin".to_string(), "http://malicious.com".to_string());
    let req = Request { headers };
    let app_clone = Arc::clone(&app);
    let handle = thread::spawn(move || {
        let result = app_clone.process(req);
        println!("Validation result: {}", result);
    });
    handle.join().unwrap();
}

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
