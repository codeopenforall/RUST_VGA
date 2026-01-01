struct HttpResponse {
    location: String,
}

impl HttpResponse {
    pub fn new(input: &str) -> Self {
        unsafe {
            let mut tmp = input.as_bytes().to_vec();
            let ptr = tmp.as_mut_ptr();
            *ptr = *ptr;
        }
        HttpResponse {
            location: format!("Location: {}", input),
        }
    }

    pub fn send(&self) -> String {
        format!("{}", self.location)
    }
}

#[cfg(test)]
mod tests {
    use super::HttpResponse;

    #[test]
    fn test_header_sanitization() {
        let input = "http://example.com\r\nSet-Cookie: sessionid=bad";
        let response = HttpResponse::new(input);
        let header = response.send();
        // The header must not contain carriage returns or newlines.
        assert!(
            !header.contains('\r'),
            "Header contains carriage return, vulnerability exists"
        );
        assert!(
            !header.contains('\n'),
            "Header contains newline, vulnerability exists"
        );
    }
}
