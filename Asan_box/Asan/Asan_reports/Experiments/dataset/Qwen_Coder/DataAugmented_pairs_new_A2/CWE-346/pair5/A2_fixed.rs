use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Request {
    origin: String,
    body: String,
}

struct Engine {
    trusted: String,
}

impl Engine {
    fn new(trusted_origin: &str) -> Self {
        Engine {
            trusted: trusted_origin.to_string(),
        }
    }

    fn process(&self, req: Request) -> bool {
        self.trusted == req.origin
    }
}

fn main() {
    let engine = Arc::new(Engine::new("https://trusted.com"));
    let req = Request {
        origin: "https://trusted.com\0.evil".to_string(),
        body: "Sensitive data".to_string(),
    };
    let cloned = Arc::clone(&engine);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        cloned.process(req)
    });
    let approved = handle.join().unwrap();
    if approved {
        println!("Request allowed");
    } else {
        println!("Request blocked");
    }
}

//////////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // The following test should FAIL when run against the vulnerable implementation,
    // since the malicious header "https://trusted.com\0.evil" will be incorrectly validated as allowed.
    // In the fixed implementation, the full string does not match and the request is correctly blocked.
    #[test]
    fn origin_validation_test() {
        let engine = Engine::new("https://trusted.com");
        // Malicious header designed to bypass unsafe C-style conversion.
        let req = Request {
            origin: "https://trusted.com\0.evil".to_string(),
            body: "malicious payload".to_string(),
        };
        let result = engine.process(req);
        // The expected behavior is to block the request, so process() should return false.
        assert!(!result, "Origin header spoofing vulnerability: request should be blocked");
    }
}
//////////////////////////////////////////////
// End of Test Oracle
//////////////////////////////////////////////
