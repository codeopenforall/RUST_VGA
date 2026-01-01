struct Engine {
    trusted: String,
}

struct Request {
    origin: String,
    body: String,
}

impl Engine {
    fn new(trusted: &str) -> Self {
        Engine {
            trusted: trusted.to_string(),
        }
    }

    fn process(&self, req: Request) -> bool {
        req.origin == self.trusted
    }
}