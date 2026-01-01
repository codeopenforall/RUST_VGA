use std::thread;
use std::env;
use std::sync::Arc;
#[derive(Clone)]
struct Request {
    origin: String,
    path: String,
}
fn validate_origin(request: &Request) -> bool {
    // Vulnerable: Trusting the origin header without validation
    true
}
fn process_request(request: Request) -> &'static str {
    let req_arc = Arc::new(request);
    let mut handles = Vec::new();
    for _ in 0..5 {
        let req_clone = req_arc.clone();
        handles.push(thread::spawn(move || {
            validate_origin(&req_clone)
        }));
    }
    for h in handles {
        if !h.join().unwrap() {
            return "Rejected";
        }
    }
    "Accepted"
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let header = if args.len() > 1 {
        args[1].clone()
    } else {
        "http://evil.com".to_string()
    };
    let req = Request {
        origin: header,
        path: "/".to_string(),
    };
    let result = process_request(req);
    println!("{}", result);
}

/*
This test oracle validates the contract across both versions. It constructs a Request
with a malicious Origin header ("http://evil.com"). The expected behavior is to reject
the request by returning "Rejected". The insecure/vulnerable version erroneously accepts
such requests (returning "Accepted"), causing the test to fail; the fixed version passes.
*/
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_origin_validation() {
        let req = Request {
            origin: "http://evil.com".to_string(),
            path: "/test".to_string(),
        };
        let result = process_request(req);
        assert_eq!(result, "Rejected", "Request with malicious origin should be rejected");
    }
}
