struct Request {
    origin: String,
    payload: String,
}

struct Service {
    allowed_origin: String,
}

impl Service {
    pub fn handle(&self, req: Request) -> bool {
        unsafe {
            let ptr = req.origin.as_ptr();
            if ptr.is_null() {
                false
            } else {
                true
            }
        }
    }
}