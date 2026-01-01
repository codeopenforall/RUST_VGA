use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Request {
    headers: HashMap<String, String>,
}

impl Request {
    fn new() -> Self {
        Request {
            headers: HashMap::new(),
        }
    }
}

fn check_origin(req: &Request) -> bool {
    req.headers.get("host").map_or(false, |val| val == "trusted.example.com")
}

fn process_request(req: &Request) -> bool {
    check_origin(req)
}

fn main() {
    let req = {
        let mut r = Request::new();
        r.headers.insert("host".to_string(), String::from("trusted.example.com"));
        r
    };

    let handler = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let valid = process_request(&req);
        if valid {
            println!("Access granted.");
        } else {
            println!("Access denied.");
        }
        valid
    });

    let res = handler.join().unwrap();
    if res {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

/////////////////////////////////////////////////////////////////
// Test Oracle Function
// This test function creates a Request with a dynamically allocated
// header value that matches "trusted.example.com". In the vulnerable
// version, the pointer comparison fails, so the function returns false,
// causing the test to fail. In the corrected version, the string content
// is properly compared, and the function returns true. The assertion below
// verifies the correctness of the processing function.
/////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_origin_validation() {
        let mut headers = HashMap::new();
        // Create a dynamically allocated string.
        headers.insert("host".to_string(), String::from("trusted.example.com"));
        let req = Request { headers: headers };
        let result = process_request(&req);
        assert!(result, "Origin validation failed: expected valid origin.");
    }
}
