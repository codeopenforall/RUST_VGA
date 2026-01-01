use std::sync::{Arc, Mutex};
use std::thread;

struct Request {
    raw: String,
}

impl Request {
    fn parse_headers(&self) -> Option<usize> {
        self.raw.find("\r\n\r\n").map(|i| i + 4)
    }

    fn get_headers(&self) -> Option<&[u8]> {
        let body_start = self.parse_headers()?;
        Some(&self.raw.as_bytes()[..body_start])
    }
}

fn process_request(request: &Request) -> bool {
    if let Some(headers) = request.get_headers() {
        let header_str = std::str::from_utf8(headers).unwrap_or("");
        if header_str.contains("X-Extra: exploit") {
            return false;
        }
    }
    true
}

fn main() {
    let attack_request = "GET / HTTP/1.1\r\nContent-Length: 0\r\nX-Extra: exploit\r\n\r\nBODY";
    let _normal_request = "GET / HTTP/1.1\r\nContent-Length: 0\r\n\r\n";
    let req = Request {
        raw: attack_request.to_owned(),
    };
    let shared = Arc::new(Mutex::new(req));
    let mut handles = vec![];
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let req = shared_clone.lock().unwrap();
            process_request(&req)
        });
        handles.push(handle);
    }
    for handle in handles {
        let res = handle.join().unwrap();
        if !res {
            println!("Request rejected.");
            std::process::exit(1);
        }
    }
    println!("Request processed safely.");
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        // Attack input with the extra malicious header.
        let malicious = "GET / HTTP/1.1\r\nContent-Length: 0\r\nX-Extra: exploit\r\n\r\nBODY";
        let req = Request { raw: malicious.to_owned() };
        
        // For a secure implementation, the malicious request should be rejected (i.e. process_request returns false).
        // In the vulnerable version, the unsafe behavior causes process_request to erroneously return true.
        // This test will fail for the vulnerable code and pass for the corrected code.
        assert!(!process_request(&req), "Malicious request should be rejected");
    }
}
