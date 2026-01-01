use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
}

struct Handler {
    config: String,
}

impl Handler {
    unsafe fn validate(&self, req: &Request) -> bool {
        if let Some(origin) = req.headers.get("Origin") {
            let expected = format!("https://{}", self.config);
            return origin == &expected;
        }
        false
    }

    fn process(&self, req: Request) -> bool {
        unsafe { self.validate(&req) }
    }
}

fn main() {
    let handler = Arc::new(Handler { config: "trusted.com".to_string() });
    let mut headers = HashMap::new();
    headers.insert("Origin".to_string(), "https://trusted.com@malicious.com".to_string());
    let req = Request { headers };
    let allowed = handler.process(req);
    println!("Request allowed: {}", allowed);

    let handler_clone = Arc::clone(&handler);
    let thread_handle = thread::spawn(move || {
        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), "https://trusted.com".to_string());
        let req = Request { headers };
        let allowed = handler_clone.process(req);
        println!("Thread request allowed: {}", allowed);
    });

    thread_handle.join().unwrap();
}

/* Test Oracle: This test should fail against the vulnerable implementation and pass with the fixed implementation.
   Save this in a separate test module and run with `cargo test`. */
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn origin_validation_oracle() {
        // Create a handler with a trusted host "trusted.com"
        // Note: Both versions expect the config to be "trusted.com".
        let handler = Handler { config: "trusted.com".to_string() };

        // Test 1: Malicious header. The proper behavior is to reject this.
        let mut headers_mal = HashMap::new();
        headers_mal.insert("Origin".to_string(), "https://trusted.com@malicious.com".to_string());
        let req_mal = Request { headers: headers_mal };
        let result_mal = handler.process(req_mal);
        // In a secure implementation, this must be false.
        assert!(!result_mal, "Malicious header should be rejected");

        // Test 2: Valid header. The correct trusted header should be accepted.
        let mut headers_valid = HashMap::new();
        headers_valid.insert("Origin".to_string(), "https://trusted.com".to_string());
        let req_valid = Request { headers: headers_valid };
        let result_valid = handler.process(req_valid);
        assert!(result_valid, "Valid header should be accepted");
    }
}
