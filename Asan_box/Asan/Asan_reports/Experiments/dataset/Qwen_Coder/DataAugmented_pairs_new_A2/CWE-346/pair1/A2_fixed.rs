use std::thread;

struct Request {
    origin: String,
    payload: String,
}

struct Service {
    allowed_origin: String,
}

impl Service {
    fn verify_origin(&self, req: &Request) -> bool {
        req.origin == self.allowed_origin
    }

    fn handle(&self, req: Request) -> bool {
        self.verify_origin(&req)
    }
}

fn main() {
    let service = Service {
        allowed_origin: "https://trusted.com".into(),
    };
    let forged_req = Request {
        origin: "https://untrusted.com".into(),
        payload: "sensitive payload".into(),
    };
    let thread_handle = thread::spawn(move || {
        let approval = service.handle(forged_req);
        println!("Request approved: {}", approval);
    });
    thread_handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    // Bring in the definitions from the module under test.
    use super::*;

    #[test]
    fn test_origin_check() {
        let service = Service {
            allowed_origin: "https://trusted.com".into(),
        };
        let forged_req = Request {
            origin: "https://untrusted.com".into(),
            payload: "dummy".into(),
        };
        let result = service.handle(forged_req);
        // The contract: a forged origin must result in rejection (i.e., false).
        assert!(!result, "Origin check failed: untrusted origin was approved");
    }
}
