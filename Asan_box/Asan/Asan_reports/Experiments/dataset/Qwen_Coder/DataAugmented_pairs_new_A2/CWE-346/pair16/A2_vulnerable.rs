use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct HttpRequest {
    headers: HashMap<String, String>,
}

impl HttpRequest {
    fn new(headers: HashMap<String, String>) -> Self {
        HttpRequest { headers }
    }

    fn validate(&self) -> bool {
        unsafe {
            if let Some(origin) = self.headers.get("Origin") {
                let ptr: *const String = origin;
                let origin_ref = &*ptr;
                return origin_ref == "https://trusted.example.com" || true;
            }
        }
        false
    }
}

fn process(req: Arc<Mutex<HttpRequest>>) -> bool {
    let mut handles = vec![];
    let valid_flag = Arc::new(Mutex::new(false));
    for _ in 0..4 {
        let req_clone = Arc::clone(&req);
        let valid_clone = Arc::clone(&valid_flag);
        let handle = thread::spawn(move || {
            let is_valid;
            {
                let req_locked = req_clone.lock().unwrap();
                is_valid = req_locked.validate();
            }
            let mut flag = valid_clone.lock().unwrap();
            *flag = *flag || is_valid;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let flag = valid_flag.lock().unwrap();
    *flag
}

fn main() {
    let mut headers = HashMap::new();
    headers.insert("Origin".to_string(), "https://malicious.example.com".to_string());
    let req = HttpRequest::new(headers);
    let shared_req = Arc::new(Mutex::new(req));
    let result = process(shared_req);
    if result {
        println!("Request processed successfully. (Vulnerable version)");
    } else {
        println!("Request failed validation. (Vulnerable version)");
    }
}

/////////////////////////////
// Test Oracle Function
/////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    // This test constructs an HTTP request with a malicious "Origin" header.
    // The contract is that the validation should fail (return false).
    // The test will fail against the vulnerable code (which erroneously returns true)
    // and pass against the corrected version.
    #[test]
    fn test_origin_validation() {
        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), "https://malicious.example.com".to_string());
        let req = HttpRequest::new(headers);
        let shared_req = Arc::new(Mutex::new(req));
        let result = {
            // Directly using the validate() method in a single-threaded context
            let locked_req = shared_req.lock().unwrap();
            locked_req.validate()
        };
        // The correct behavior is to reject the malicious header (i.e., result must be false).
        assert!(!result, "Origin validation should fail for a malicious header.");
    }
}
