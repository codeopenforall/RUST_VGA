use std::collections::HashMap;

struct HttpRequest {
    headers: HashMap<String, String>,
}

impl HttpRequest {
    fn new(headers: HashMap<String, String>) -> Self {
        HttpRequest { headers }
    }

    fn validate(&self) -> bool {
        match self.headers.get("Origin") {
            Some(origin_ref) => origin_ref == "https://trusted.example.com",
            None => false,
        }
    }
}