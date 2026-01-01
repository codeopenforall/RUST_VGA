use std::io::{self, Write};

struct HttpResponder;

impl HttpResponder {
    pub fn reply(&self, location: &str) -> String {
        // Sanitize the location to prevent CRLF injection
        let sanitized_location = location.replace("\r", "").replace("\n", "");
        format!("HTTP/1.1 302 Found\r\nLocation: {}\r\n\r\n", sanitized_location)
    }
}

fn main() {
    let input = "http://example.com\r\nSet-Cookie: malicious=1";
    let responder = HttpResponder;
    let response = responder.reply(input);
    println!("{}", response);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_injection() {
        let responder = HttpResponder;
        // This input includes CRLF injection which should be neutralized in the secure version.
        let input = "http://example.com\r\nSet-Cookie: injected=true";
        let header = responder.reply(input);
        // The header must not contain an injected header line. In the vulnerable version,
        // the unsanitized CRLF would cause "\r\nSet-Cookie:" to appear as a new header.
        assert!(!header.contains("\r\nSet-Cookie:"), "CRLF injection vulnerability detected.");
    }
}
