use std::thread;

struct Request {
    origin: String,
}

impl Request {
    fn new(origin: &str) -> Self {
        Request {
            origin: origin.to_string(),
        }
    }
}

fn validate(req: &Request) -> bool {
    req.origin == "https://trusted.com"
}

fn process(req: Request) {
    let handle = thread::spawn(move || {
        if validate(&req) {
            println!("Request accepted: {}", req.origin);
        } else {
            println!("Request rejected: {}", req.origin);
        }
    });
    handle.join().unwrap();
}

fn main() {
    let req = Request::new("https://trusted.com");
    process(req);
}

#[cfg(test)]
mod tests {
    // Reuse the Request structure and validate function signature from the server implementation.
    // Ensure that only the exact header "https://trusted.com" is accepted.
    // Note: This test code is shared by both versions and is not part of the main server code.
    use super::*;
    
    #[test]
    fn test_origin_validation() {
        // Malicious input that should be rejected.
        let malicious = Request::new("https://evil.com");
        // The contract is that the validation must fail for non-trusted origins.
        // In the vulnerable version, this check incorrectly returns true, causing the test to fail.
        assert_eq!(validate(&malicious), false, "Invalid origin should be rejected");
    }
}
