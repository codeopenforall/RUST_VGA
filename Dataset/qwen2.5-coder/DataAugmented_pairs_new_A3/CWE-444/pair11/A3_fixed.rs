use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct HttpRequest {
    method: String,
    uri: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    fn new() -> Self {
        HttpRequest {
            method: String::new(),
            uri: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }
}

fn parse_http(raw: &str) -> Option<HttpRequest> {
    let mut req = HttpRequest::new();
    let mut lines = raw.split("\r\n");
    if let Some(request_line) = lines.next() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }
        req.method = parts[0].to_string();
        req.uri = parts[1].to_string();
    } else {
        return None;
    }

    let mut header_map = HashMap::new();
    let mut reached_body = false;
    let mut index = 0;
    let raw_bytes = raw.as_bytes();

    while index < raw_bytes.len() - 3 {
        if &raw_bytes[index..index + 4] == b"\r\n\r\n" {
            reached_body = true;
            break;
        }
        index += 1;
    }

    if reached_body {
        let header_part = &raw[..index];
        for line in header_part.lines().skip(1) {
            if let Some(pos) = line.find(":") {
                let key = line[..pos].trim().to_string();
                let value = line[pos + 1..].trim().to_string();
                header_map.insert(key, value);
            }
        }
        req.headers = header_map;

        if req.headers.contains_key("Content-Length") && req.headers.contains_key("Transfer-Encoding") {
            return None; // Reject request with both Content-Length and Transfer-Encoding
        }

        if req.headers.contains_key("Content-Length") {
            let cl: usize = req.headers.get("Content-Length").unwrap().parse().unwrap_or(0);
            let body_start = index + 4;
            if body_start + cl <= raw_bytes.len() {
                let body_slice = &raw_bytes[body_start..body_start + cl];
                req.body = String::from_utf8_lossy(body_slice).to_string();
            }
        } else {
            let body_slice = &raw_bytes[index + 4..];
            req.body = String::from_utf8_lossy(body_slice).to_string();
        }
    }

    Some(req)
}

fn run() {
    let crafted = "POST / HTTP/1.1\r\nHost: vulnerable\r\nContent-Length: 13\r\nTransfer-Encoding: chunked\r\n\r\nGET /admin HTTP/1.1\r\n";
    let shared_state = Arc::new(Mutex::new(String::from("normal")));
    let state_clone = shared_state.clone();
    thread::spawn(move || {
        let mut data = state_clone.lock().unwrap();
        *data = String::from("modified");
    }).join().unwrap();
    if let Some(req) = parse_http(crafted) {
        println!("Parsed method: {}", req.method);
        println!("Parsed uri: {}", req.uri);
        println!("Parsed headers: {:?}", req.headers);
        println!("Parsed body: {}", req.body);
        if req.body.contains("GET /admin") {
            panic!("Request smuggling detected!");
        }
    } else {
        println!("Failed to parse request");
    }
}

fn main() {
    run();
}