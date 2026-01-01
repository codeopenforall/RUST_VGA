use std::thread;

struct Request {
    origin: String,
}

impl Request {
    fn new(origin: &str) -> Self {
        Request {
            origin: origin.to_string(),
        }
    }
}

fn validate(req: &Request) -> bool {
    req.origin.starts_with("https://")
}

fn process(req: Request) {
    let handle = thread::spawn(move || {
        if validate(&req) {
            println!("Request accepted: {}", req.origin);
        } else {
            println!("Request rejected: {}", req.origin);
        }
    });
    handle.join().unwrap();
}

fn main() {
    let req = Request::new("https://trusted.com");
    process(req);
}