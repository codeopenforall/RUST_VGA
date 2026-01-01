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