use std::collections::HashMap;
use std::sync::Arc;

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
        if req.headers.contains_key("X-Alternate-Auth") {
            unsafe {
                return self.bypass_auth();
            }
        }
        false
    }

    unsafe fn bypass_auth(&self) -> bool {
        true
    }
}

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

fn main() {}

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
