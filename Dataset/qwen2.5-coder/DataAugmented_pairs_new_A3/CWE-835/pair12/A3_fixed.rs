struct Server {
    allowed_origin: String,
}

impl Server {
    fn new(allowed_origin: &str) -> Self {
        Server {
            allowed_origin: allowed_origin.to_string(),
        }
    }

    fn validate(&self, header: &str) -> bool {
        if let Ok(hdr) = std::str::from_utf8(header.as_bytes()) {
            hdr == self.allowed_origin
        } else {
            false
        }
    }
}