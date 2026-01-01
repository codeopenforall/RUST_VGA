#![allow(unused_imports)]
use std::sync::Arc;
use std::thread;

struct Server {
    header: String,
}

impl Server {
    fn new(input: &str) -> Self {
        // Sanitize the input to remove CRLF characters
        let sanitized_input = input.replace("\r\n", "");
        let header = format!("HTTP/1.1 200 OK\r\nCustom-Header: {}\r\n", sanitized_input);
        Server { header }
    }

    fn send(&self) -> String {
        let header_ptr = self.header.as_ptr();
        let header_len = self.header.len();
        unsafe {
            let raw_slice = std::slice::from_raw_parts(header_ptr, header_len);
            String::from_utf8_lossy(raw_slice).to_string()
        }
    }
}

fn main() {
    let input = "malicious\r\nInjected: bad";
    let server = Arc::new(Server::new(input));
    let server_clone = Arc::clone(&server);
    let handle = thread::spawn(move || {
        server_clone.send()
    });
    let response = handle.join().unwrap();
    println!("{}", response);
}