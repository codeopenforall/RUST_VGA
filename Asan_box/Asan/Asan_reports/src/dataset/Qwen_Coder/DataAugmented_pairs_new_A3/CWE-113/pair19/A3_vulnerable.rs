struct HttpResponse {
    headers: Vec<String>,
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse { headers: Vec::new() }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        unsafe {
            let raw = value.as_bytes();
            let untrusted = std::str::from_utf8_unchecked(raw);
            let header_line = format!("{}: {}", key, untrusted);
            self.headers.push(header_line);
        }
    }

    pub fn get_fields(&self) -> Vec<&str> {
        self.headers.iter().map(|s| s.as_str()).collect()
    }
}

fn construct_response(value: &str) -> HttpResponse {
    let mut response = HttpResponse::new();
    response.add_header("Content-Type", value);
    response
}

//////////////////////////////////////////////////////////////
// Test oracle: verifies that the header does not contain CRLF.
// This test should fail against the vulnerable version and pass against the fixed version.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_injection_oracle() {
        // Malicious input crafted with CRLF sequences.
        let malicious_input = "normal\r\nInjected: evil";
        let response = construct_response(malicious_input);
        // Verify that none of the headers include the CRLF sequence.
        for field in response.get_fields() {
            assert!(
                !field.contains("\r\n"),
                "Header contains CRLF sequences, indicating injection vulnerability"
            );
        }
    }
}
