use std::collections::HashMap;

struct Service {
    approved: Vec<String>,
}

impl Service {
    fn new(approved: Vec<String>) -> Self {
        Service { approved }
    }

    pub fn process(&self, req: Request) -> Result<String, &'static str> {
        let origin = req.headers.get("Origin").ok_or("Origin header missing")?;
        if !self.approved.contains(origin) {
            return Err("Origin not authorized");
        }
        let mut url = "http://".to_owned();
        url.push_str(origin);
        Ok(url)
    }
}

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    fn new(headers: HashMap<String, String>, body: String) -> Self {
        Request { headers, body }
    }
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
