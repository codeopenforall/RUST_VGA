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