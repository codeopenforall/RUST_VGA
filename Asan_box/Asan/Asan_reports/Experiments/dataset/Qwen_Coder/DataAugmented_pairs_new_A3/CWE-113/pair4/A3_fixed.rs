use std::ffi::CString;

struct HttpResponse {
    header: String,
}

impl HttpResponse {
    pub fn get_header(&self) -> &str {
        &self.header
    }
}

fn sanitize(input: &str) -> String {
    input.replace('\r', "").replace('\n', "")
}

fn build_response(input: &str) -> HttpResponse {
    let cleaned = sanitize(input);
    let c_input = CString::new(cleaned.clone()).unwrap();
    // Simulate building a response with sanitized input
    HttpResponse {
        header: format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n{}", cleaned),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Malicious input containing CRLF sequences intended to inject an extra header.
    fn injected_input() -> &'static str {
        "malicious\r\nX-Injection: attack"
    }

    #[test]
    fn test_header_injection() {
        let response = build_response(injected_input());
        // The vulnerable version will include the injected header when given unsanitized input.
        // The fixed version sanitizes the input so that the exact CRLF injection pattern is impossible.
        assert!(
            !response.get_header().contains("\r\nX-Injection:"),
            "Header injection detected"
        );
        assert!(response.get_header().contains("HTTP/1.1 200 OK"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Malicious input containing CRLF sequences intended to inject an extra header.
    fn injected_input() -> &'static str {
        "malicious\r\nX-Injection: attack"
    }

    #[test]
    fn test_header_injection() {
        let response = build_response(injected_input());
        // The vulnerable version will include the injected header when given unsanitized input.
        // The fixed version sanitizes the input so that the exact CRLF injection pattern is impossible.
        assert!(
            !response.get_header().contains("\r\nX-Injection:"),
            "Header injection detected"
        );
        assert!(response.get_header().contains("HTTP/1.1 200 OK"));
    }
}
