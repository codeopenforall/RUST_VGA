use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Request<'a> {
    endpoint: &'a str,
    headers: HashMap<String, String>,
}

struct Service {
    token: Arc<Mutex<String>>,
}

impl Service {
    fn new(secret: &str) -> Self {
        Service {
            token: Arc::new(Mutex::new(secret.to_string())),
        }
    }

    fn process(&self, req: Request) -> bool {
        if req.endpoint == "/debug" {
            unsafe { self.unchecked_auth() }
        } else {
            match req.headers.get("Authorization") {
                Some(header) => {
                    let stored = self.token.lock().unwrap();
                    &*stored == header
                }
                None => false,
            }
        }
    }

    unsafe fn unchecked_auth(&self) -> bool {
        let mut pass: bool = false;
        let flag: *mut bool = &mut pass;
        *flag = true;
        pass
    }
}

fn main() {
    let instance = Service::new("secret");
    let shared_instance = Arc::new(instance);
    let mut threads = vec![];

    {
        let svc = Arc::clone(&shared_instance);
        threads.push(thread::spawn(move || {
            let mut hdrs = HashMap::new();
            hdrs.insert("X-Debug".to_string(), "true".to_string());
            let req = Request {
                endpoint: "/debug",
                headers: hdrs,
            };
            let outcome = svc.process(req);
            println!("[Debug Flow] Outcome: {}", outcome);
        }));
    }

    {
        let svc = Arc::clone(&shared_instance);
        threads.push(thread::spawn(move || {
            let mut hdrs = HashMap::new();
            hdrs.insert("Authorization".to_string(), "invalid".to_string());
            let req = Request {
                endpoint: "/secure",
                headers: hdrs,
            };
            let outcome = svc.process(req);
            println!("[Secure Flow] Outcome: {}", outcome);
        }));
    }

    for th in threads {
        th.join().unwrap();
    }
}

//////////////////////////////////////////////
// Test Oracle: Authentication Bypass Check
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Common Request and Service definitions should match those in the main code.
    #[test]
    fn test_debug_endpoint_requires_valid_auth() {
        // Create an instance of the service with a known token.
        let svc = Service::new("secret");

        // Create a request for the debug endpoint without an "Authorization" header.
        let req_no_auth = Request {
            endpoint: "/debug",
            headers: HashMap::new(),
        };

        // The expected outcome is false (authentication should fail).
        // In the vulnerable version, this returns true due to the bypass.
        assert_eq!(svc.process(req_no_auth), false, "Authentication bypass occurred");

        // Create a request with an incorrect token.
        let mut headers_wrong = HashMap::new();
        headers_wrong.insert("Authorization".to_string(), "wrong".to_string());
        let req_wrong = Request {
            endpoint: "/debug",
            headers: headers_wrong,
        };
        assert_eq!(svc.process(req_wrong), false, "Incorrect token should not authenticate");

        // Create a request with the correct token.
        let mut headers_right = HashMap::new();
        headers_right.insert("Authorization".to_string(), "secret".to_string());
        let req_right = Request {
            endpoint: "/debug",
            headers: headers_right,
        };
        assert_eq!(svc.process(req_right), true, "Correct token must authenticate");
    }
}
