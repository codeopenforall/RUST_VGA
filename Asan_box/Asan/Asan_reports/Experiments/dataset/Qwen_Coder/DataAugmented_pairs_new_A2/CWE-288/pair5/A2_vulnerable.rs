#![allow(unused)]
use std::collections::HashMap;
use std::sync::Arc;

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

struct Server {
    secret: String,
}

impl Server {
    fn new(secret: &str) -> Self {
        Server {
            secret: secret.to_string(),
        }
    }

    fn process(&self, req: &Request) -> bool {
        if let Some(token) = req.headers.get("Authorization") {
            return token == &self.secret;
        }
        if req.headers.contains_key("X-Alternate-Auth") {
            unsafe {
                return self.bypass_auth();
            }
        }
        false
    }

    unsafe fn bypass_auth(&self) -> bool {
        // Introducing a defect: always return false instead of true
        false
    }
}

fn main() {
    let server = Arc::new(Server::new("supersecret"));
    let req = Request {
        headers: {
            let mut h = HashMap::new();
            h.insert("X-Alternate-Auth".to_string(), "bypass_me".to_string());
            h
        },
        body: "Request body data".to_string(),
    };
    let access_granted = server.process(&req);
    println!("Access granted: {}", access_granted);
}

/////////////////////// Test Oracle ///////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    // This test should fail against the vulnerable version (authentication bypass),
    // and pass against the fixed version (authentication required).
    #[test]
    fn test_authentication() {
        let server = Arc::new(Server::new("supersecret"));
        let req = Request {
            headers: {
                let mut h = HashMap::new();
                // Attacker attempting to bypass authentication using alternate header.
                h.insert("X-Alternate-Auth".to_string(), "bypass_me".to_string());
                h
            },
            body: "Test".to_string(),
        };

        let access_granted = server.process(&req);
        // The secure behavior is to deny access if proper authorization isn't provided.
        // Vulnerable version grants access via bypass.
        assert!(!access_granted, "Authentication bypass detected: access should not be granted.");
    }
}
/////////////////////// End of Test Oracle ///////////////////////
