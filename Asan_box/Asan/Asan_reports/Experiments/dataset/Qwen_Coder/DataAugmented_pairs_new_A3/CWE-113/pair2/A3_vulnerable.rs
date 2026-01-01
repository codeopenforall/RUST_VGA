struct HttpResponse {
    header: String,
}

impl HttpResponse {
    pub fn new(name: &str, value: &str) -> Self {
        let header = format!("{}: {}\r\n", name, value);
        HttpResponse { header }
    }

    pub fn get_header(&self) -> &str {
        &self.header
    }
}

pub fn generate_response(value: &str) -> String {
    let response = HttpResponse::new("Location", value);
    response.get_header().to_string()
}

//////////////////////
// Test Oracle Function
//////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crlf_injection_prevention() {
        // Malicious input containing CRLF injection.
        let malicious_input = "https://example.com\r\nInjected: malicious";
        let header = generate_response(malicious_input);
        // The fixed version must never include an injected header part.
        // For the vulnerable version this assertion would fail.
        assert!(
            !header.contains("\r\nInjected:"),
            "CRLF injection detected in header"
        );
    }
}
