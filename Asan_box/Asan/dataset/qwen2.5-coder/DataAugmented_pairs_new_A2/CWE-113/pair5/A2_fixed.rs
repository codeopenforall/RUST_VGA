#![allow(unused_imports)]
use std::sync::Arc;
use std::thread;

struct Server {
    header: String,
}

impl Server {
    fn new(input: &str) -> Self {
        let sanitized: String = input.chars()
            .filter(|&c| c != '\r' && c != '\n')
            .collect();
        let header = format!("HTTP/1.1 200 OK\r\nCustom-Header: {}\r\n", sanitized);
        Server { header }
    }

    fn send(&self) -> String {
        self.header.clone()
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