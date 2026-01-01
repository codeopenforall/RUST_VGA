struct Request {
    origin: String,
    payload: String,
}

struct Service {
    allowed_origin: String,
}

impl Service {
    pub fn handle(&self, req: Request) -> bool {
        req.origin == self.allowed_origin
    }
}