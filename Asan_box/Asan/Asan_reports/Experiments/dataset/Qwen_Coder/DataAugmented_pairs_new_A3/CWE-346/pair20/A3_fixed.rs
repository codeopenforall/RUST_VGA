use std::sync::{Arc, Mutex};
use std::collections::HashMap;

struct HttpRequest {
    headers: HashMap<String, String>,
    payload: String,
}

impl HttpRequest {
    fn new(headers: Vec<(String, String)>, payload: String) -> Self {
        HttpRequest {
            headers: headers.into_iter().collect(),
            payload,
        }
    }
}

struct Server {
    allowed: Arc<Mutex<Vec<String>>>,
}

impl Server {
    fn new() -> Self {
        Server {
            allowed: Arc::new(Mutex::new(vec!["https://example.com".to_string()])),
        }
    }

    fn verify_header(&self, req: &HttpRequest) -> bool {
        let allowed = self.allowed.lock().unwrap();
        for (key, value) in &req.headers {
            if key == "Origin" && allowed.contains(value) {
                return true;
            }
        }
        false
    }

    fn run(&self, req: HttpRequest) -> String {
        if self.verify_header(&req) {
            "Access granted".to_string()
        } else {
            "Access denied".to_string()
        }
    }
}

////////////////////////////////////
// Test Oracle                    //
// This single unit test constructs //
// a request with a malicious value //
// for the Origin header. The test  //
// expects "Access denied" to be     //
// returned. The vulnerable version  //
// erroneously returns "Access granted"//
// causing the test to fail, while   //
// the fixed version passes.         //
////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_validation() {
        // Craft a request with a malicious origin.
        let malicious_request = HttpRequest::new(
            vec![("Origin".to_string(), "http://malicious.example.com".to_string())],
            "payload".to_string()
        );

        let server = Server::new();
        let result = server.run(malicious_request);
        // Expect rejection: "Access denied".
        assert_eq!(result, "Access denied", "The request must be rejected due to invalid origin");
    }
}
