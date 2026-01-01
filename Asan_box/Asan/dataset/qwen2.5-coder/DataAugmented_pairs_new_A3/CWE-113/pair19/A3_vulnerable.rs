struct HttpResponse {
    headers: Vec<String>,
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse { headers: Vec::new() }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        unsafe {
            let raw = value.as_bytes();
            let untrusted = std::str::from_utf8_unchecked(raw);
            let header_line = format!("{}: {}", key, untrusted);
            self.headers.push(header_line);
        }
    }

    pub fn get_fields(&self) -> Vec<&str> {
        self.headers.iter().map(|s| s.as_str()).collect()
    }
}

fn construct_response(value: &str) -> HttpResponse {
    let mut response = HttpResponse::new();
    response.add_header("Content-Type", value);
    response
}