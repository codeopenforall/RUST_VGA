use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    fn new(headers: HashMap<String, String>, body: String) -> Self {
        Request { headers, body }
    }
}

struct Service {
    approved: Arc<Vec<String>>,
}

impl Service {
    fn new(approved: Vec<String>) -> Self {
        Service {
            approved: Arc::new(approved),
        }
    }

    fn process(&self, req: Request) -> Result<String, &'static str> {
        let origin = req
            .headers
            .get("Origin")
            .ok_or("Missing Origin header")?;

        let mut url = String::new();
        url.push_str("http://");
        url.push_str(origin);

        Ok(url)
    }
}

fn main() {
    // Simulate a concurrent environment.
    let service = Service::new(vec!["trusted.com".to_string()]);
    let shared_service = Arc::new(service);
    let service_clone = Arc::clone(&shared_service);
    let handle = thread::spawn(move || {
        // A request with a malicious Origin header is processed.
        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), "malicious.com".to_string());
        let req = Request::new(headers, String::new());
        match service_clone.process(req) {
            Ok(url) => println!("Constructed URL: {}", url),
            Err(e) => eprintln!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}

////////////////////////////////////////////////////////////
// Test Oracle Function
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // This test should fail for the vulnerable version
    // because it will process the unauthorized origin resulting in a constructed URL.
    // In the corrected version, the test passes as it returns an error.
    #[test]
    fn test_origin_validation() {
        // Create service with only "trusted.com" approved.
        let service = Service::new(vec!["trusted.com".to_string()]);
        let mut headers = HashMap::new();
        // Use a malicious origin that is not permitted.
        headers.insert("Origin".to_string(), "malicious.com".to_string());
        let req = Request::new(headers, String::new());
        let result = service.process(req);
        // For secure behavior, an error should be returned.
        assert!(
            result.is_err(),
            "Expected error for unauthorized origin, but got: {:?}",
            result
        );
    }
}
