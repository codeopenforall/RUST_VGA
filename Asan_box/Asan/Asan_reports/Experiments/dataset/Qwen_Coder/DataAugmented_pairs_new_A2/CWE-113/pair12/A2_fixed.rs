use std::sync::{Arc, Mutex};
use std::thread;

struct HttpResponse {
    header: String,
}

impl HttpResponse {
    pub fn new(input: &str) -> HttpResponse {
        let sanitized: String = input.lines().next().unwrap_or("").to_string();
        HttpResponse {
            header: format!("Location: {}", sanitized),
        }
    }

    pub fn send(&self) -> String {
        self.header.clone()
    }
}

fn main() {
    let input = "http://example.com\r\nSet-Cookie: sessionid=bad";
    let response = Arc::new(Mutex::new(HttpResponse::new(input)));
    let mut threads = vec![];
    for _ in 0..4 {
        let clone_response = Arc::clone(&response);
        threads.push(thread::spawn(move || {
            let resp = clone_response.lock().unwrap();
            let header = resp.send();
            println!("{}", header);
        }));
    }
    for t in threads {
        t.join().unwrap();
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
