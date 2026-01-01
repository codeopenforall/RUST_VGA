struct HttpResponder;

impl HttpResponder {
    pub fn reply(&self, location: &str) -> String {
        let raw = location.as_ptr();
        let len = location.len();
        let input = "http://example.com\r\nSet-Cookie: malicious=1";
        format!("Location: {}\r\nContent-Type: text/plain", input)
    }
}