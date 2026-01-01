struct HttpResponse {
    headers: Vec<String>,
}

impl HttpResponse {
    fn new() -> Self {
        HttpResponse { headers: Vec::new() }
    }

    fn add_header(&mut self, key: &str, value: &str) {
        let safe_value: String = value.chars().filter(|&c| c != '\r' && c != '\n').collect();
        self.headers.push(format!("{}: {}", key, safe_value));
    }

    fn to_string(&self) -> String {
        self.headers.join("\n")
    }
}

fn process(input: &str) -> String {
    let mut response = HttpResponse::new();
    response.add_header("Content-Type", "text/plain");
    response.add_header("X-Custom-Header", input);
    response.to_string()
}

///////////////////////////////////////////////////////////////
// Test Oracle Function for HTTP Header Injection
// This test function validates the behavior of the production code.
// It uses a malicious input that includes CRLF sequences to attempt header injection.
// The invariant is that the resulting header should NOT contain an injected header key.
// This oracle should FAIL when run against the vulnerable version and PASS against the fixed version.
///////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_header_sanitization() {
        // Malicious payload containing CRLF sequences.
        let payload = "malicious\r\nInjected: bad";
        let response = process(payload);

        // In a secure implementation, the response should not contain the string "Injected:".
        // This assertion expects that no new header was injected.
        assert!(
            !response.contains("\nInjected:"),
            "Header injection detected: response contains injected header"
        );
    }
}
