use std::collections::HashMap;
use std::thread;
struct Request {
    headers: HashMap<String, String>,
}
impl Request {
    fn new(origin: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert(String::from("Origin"), origin.to_string());
        Request { headers }
    }
}
fn validate(req: &Request) -> bool {
    let trusted = "https://valid.example.com";
    if let Some(candidate) = req.headers.get("Origin") {
        unsafe {
            let cand_ptr = candidate.as_ptr();
            let trusted_ptr = trusted.as_ptr();
            if cand_ptr == trusted_ptr {
                return true;
            }
        }
    }
    false
}
fn run() {
    let req = Request::new("https://valid.example.com");
    if validate(&req) {
        println!("Access granted.");
    } else {
        println!("Access denied.");
    }
}
fn main() {
    let handle = thread::spawn(|| {
        run();
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Test function for validating the origin check.
    // In the vulnerable version, using a header value that exactly matches
    // "https://valid.example.com" will fail because pointer comparison does not hold.
    // In the corrected version, the check will succeed.
    #[test]
    fn test_origin_validation() {
        // Construct a request with the trusted header value.
        let req = Request {
            headers: {
                let mut map = HashMap::new();
                map.insert(String::from("Origin"), String::from("https://valid.example.com"));
                map
            },
        };
        let valid = validate(&req);
        assert!(valid, "The origin check did not pass; vulnerability or fix misapplied.");
    }
}
