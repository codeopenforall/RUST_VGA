use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
}

struct Engine;

impl Engine {
    fn run(&self, req: Arc<Request>) -> bool {
        let shared_req = req.clone();
        let handle = thread::spawn(move || {
            validate_origin(&shared_req.headers)
        });
        handle.join().unwrap()
    }
}

fn validate_origin(headers: &HashMap<String, String>) -> bool {
    let trusted_prefix = "https://trusted.com";
    if let Some(origin) = headers.get("Origin") {
        origin.starts_with(trusted_prefix)
    } else {
        false
    }
}

fn main() {
    let mut hdrs = HashMap::new();
    hdrs.insert("Origin".to_string(), "https://trusted.com.evil".to_string());
    let request = Arc::new(Request { headers: hdrs });
    let service = Engine;
    let outcome = service.run(request);
    if outcome {
        println!("Request passed validation");
    } else {
        println!("Request failed validation");
    }
}