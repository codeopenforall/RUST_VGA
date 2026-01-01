struct HttpResponse {
    headers: Vec<String>,
}

impl HttpResponse {
    fn new() -> Self {
        HttpResponse { headers: Vec::new() }
    }

    fn add_header(&mut self, key: &str, value: &str) {
        let safe_value: String = value.chars().filter(|&c| c != '\r' && c != '\n').collect();
        self.headers.push(format!("{}: {}", key, safe_value));
    }

    fn to_string(&self) -> String {
        self.headers.join("\n")
    }
}

fn process(input: &str) -> String {
    let mut response = HttpResponse::new();
    response.add_header("Content-Type", "text/plain");
    response.add_header("X-Custom-Header", input);
    response.to_string()
}