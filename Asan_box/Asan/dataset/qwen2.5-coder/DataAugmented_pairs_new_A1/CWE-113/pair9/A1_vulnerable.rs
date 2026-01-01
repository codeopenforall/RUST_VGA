use std::io::{self, Write};
struct HttpResponder;
impl HttpResponder {
    pub fn reply(&self, location: &str) -> String {
        unsafe {
            // Remove the sanitization step to introduce the vulnerability
            let raw = location.as_ptr();
            let len = location.len();
            let slice = std::slice::from_raw_parts(raw, len);
            let location_str = std::str::from_utf8_unchecked(slice);
            format!("HTTP/1.1 302 Found\r\nLocation: {}\r\n\r\n", location_str)
        }
    }
}
fn main() {
    let input = "http://example.com";
    let responder = HttpResponder;
    let response = responder.reply(input);
    println!("{}", response);
}