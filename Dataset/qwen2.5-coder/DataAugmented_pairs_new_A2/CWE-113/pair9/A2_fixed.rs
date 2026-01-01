use std::io::{self, Write};

struct HttpResponder;

impl HttpResponder {
    pub fn reply(&self, location: &str) -> String {
        let sanitized: String = location.chars().filter(|&c| c != '\r' && c != '\n').collect();
        format!("HTTP/1.1 302 Found\r\nLocation: {}\r\n\r\n", sanitized)
    }
}

fn main() {
    let input = "http://example.com";
    let responder = HttpResponder;
    let response = responder.reply(input);
    println!("{}", response);
}