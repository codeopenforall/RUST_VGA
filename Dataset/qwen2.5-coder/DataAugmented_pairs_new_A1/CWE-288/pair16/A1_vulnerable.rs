use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

struct SecurityHandler {
    debug: bool,
}

impl SecurityHandler {
    fn new() -> Self {
        SecurityHandler { debug: true } // Change debug to true
    }

    fn verify(&self, req: &Request) -> bool {
        if self.debug {
            if let Some(flag) = req.headers.get("X-Debug-Bypass") {
                if flag == "true" {
                    return true; // Bypass authentication
                }
            }
        }
        req.body == "secure-password"
    }
}

fn process_input(handler: Arc<SecurityHandler>, req: Request) -> bool {
    let handler_clone = handler.clone();
    let handle = thread::spawn(move || handler_clone.verify(&req));
    handle.join().unwrap_or(false)
}

fn main() {
    let handler = Arc::new(SecurityHandler::new());
    let mut headers = HashMap::new();
    headers.insert("X-Debug-Bypass".to_string(), "true".to_string());
    let req = Request { headers, body: "wrong-password".to_string() };
    let result = process_input(handler.clone(), req);
    if result {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}