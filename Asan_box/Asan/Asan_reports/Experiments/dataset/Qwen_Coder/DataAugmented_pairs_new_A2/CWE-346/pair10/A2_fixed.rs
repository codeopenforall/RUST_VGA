use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

struct Server {
    allowed_origin: String,
}

impl Server {
    fn process(&self, req: &Request) -> bool {
        let origin = match req.headers.get("Origin") {
            Some(value) => value,
            None => return false,
        };
        origin.starts_with("https://") && origin == &self.allowed_origin
    }
}

fn main() {
    let server = Arc::new(Server {
        allowed_origin: "https://secure.example.com".to_string(),
    });
    let req = Request {
        headers: {
            let mut h = HashMap::new();
            h.insert("Origin".to_string(), "https://secure.example.com".to_string());
            h
        },
        body: "Test body".to_string(),
    };
    let server_clone = Arc::clone(&server);
    let handle = thread::spawn(move || {
        let allowed = server_clone.process(&req);
        if allowed {
            println!("Request processed safely.");
        } else {
            println!("Request rejected.");
        }
    });
    handle.join().unwrap();
}

//////////////// Test Oracle ////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // This test ensures that requests with an incorrect origin are rejected.
    // In the vulnerable implementation, the request would improperly be processed.
    // The fixed implementation should correctly return false for malicious headers.
    #[test]
    fn test_origin_validation() {
        let server = Server {
            allowed_origin: "https://secure.example.com".to_string(),
        };

        let req = Request {
            headers: {
                let mut h = HashMap::new();
                h.insert("Origin".to_string(), "https://malicious.example.com".to_string());
                h
            },
            body: "Test body".to_string(),
        };

        // The contract: the server should reject the request with a malicious origin.
        assert_eq!(server.process(&req), false, "Origin validation failed: malicious origin was accepted.");
    }
}
