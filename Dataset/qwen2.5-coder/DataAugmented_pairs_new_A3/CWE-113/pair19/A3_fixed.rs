struct HttpResponse {
    headers: Vec<String>,
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse { headers: Vec::new() }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        let sanitized: String = value.chars().filter(|&c| c != '\r' && c != '\n').collect();
        let header_line = format!("{}: {}", key, sanitized);
        self.headers.push(header_line);
    }

    pub fn get_fields(&self) -> Vec<&str> {
        self.headers.iter().map(|s| s.as_str()).collect()
    }
}

fn construct_response(value: &str) -> HttpResponse {
    let mut response = HttpResponse::new();
    response.add_header("X-Custom-Header", value);
    response
}