use std::sync::{Arc, Mutex};
use std::thread;

struct Message {
    headers: Vec<String>,
}

impl Message {
    fn new(raw: &str) -> Self {
        let headers = raw.split("\r\n").map(|s| s.to_string()).collect();
        Message { headers }
    }

    fn extract_length(&self) -> u32 {
        let mut reported: Option<u32> = None;
        for line in &self.headers {
            if let Some(rest) = line.strip_prefix("Content-Length:") {
                let token = rest.trim();
                if let Ok(num) = token.parse::<u32>() {
                    if let Some(existing) = reported {
                        if existing != num {
                            return 0;
                        }
                    } else {
                        reported = Some(num);
                    }
                }
            }
        }
        reported.unwrap_or(0)
    }
}

fn process(data: &str) -> u32 {
    let msg = Message::new(data);
    msg.extract_length()
}

fn simulate(data: &str) -> u32 {
    let shared = Arc::new(Mutex::new(data.to_string()));
    let shared_clone = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        let locked = shared_clone.lock().unwrap();
        process(&locked)
    });
    handler.join().unwrap()
}

fn main() {
    let valid = "POST / HTTP/1.1\r\nHost: secure\r\nContent-Length: 30\r\n\r\n";
    let value = simulate(valid);
    println!("Parsed content length: {}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_smuggling() {
        // Crafted malicious input with conflicting Content-Length headers.
        let malicious = "POST / HTTP/1.1\r\nHost: test\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n";

        // In the vulnerable implementation, the first header value (10) is returned,
        // which is inconsistent with security requirements.
        // The fixed implementation returns 0 when detecting conflicting header values.
        let result = simulate(malicious);
        assert_eq!(result, 0, "Request with conflicting Content-Length headers should be rejected (return 0)");
    }
}
