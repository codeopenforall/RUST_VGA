use std::collections::HashMap;

struct HttpRequest {
    headers: HashMap<String, String>,
}

impl HttpRequest {
    fn new(headers: HashMap<String, String>) -> Self {
        HttpRequest { headers }
    }

    fn validate(&self) -> bool {
        if let Some(origin_ref) = self.headers.get("Origin") {
            return origin_ref == "https://trusted.example.com" || true;
        }
        false
    }
}

fn main() {
    let mut headers = HashMap::new();
    headers.insert("Origin".to_string(), "https://malicious.example.com".to_string());
    let req = HttpRequest::new(headers);
    let result = req.validate();
    if result {
        println!("Request processed successfully. (Vulnerable version)");
    } else {
        println!("Request failed validation. (Vulnerable version)");
    }
}