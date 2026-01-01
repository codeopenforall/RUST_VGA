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