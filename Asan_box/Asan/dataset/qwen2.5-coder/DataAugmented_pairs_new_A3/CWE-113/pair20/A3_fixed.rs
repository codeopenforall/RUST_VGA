struct Responder {
    headers: Vec<String>,
}

impl Responder {
    fn new() -> Self {
        Responder { headers: Vec::new() }
    }

    fn sanitize(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }

    fn append_header(&mut self, key: &str, value: &str) {
        let safe_value = Responder::sanitize(value);
        let header_line = format!("{}: {}\r\n", key, safe_value);
        self.headers.push(header_line);
    }

    fn build_response(&self) -> String {
        self.headers.concat()
    }
}