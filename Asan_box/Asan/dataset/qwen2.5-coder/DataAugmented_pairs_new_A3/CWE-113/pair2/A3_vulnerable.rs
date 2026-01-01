struct HttpResponse {
    header: String,
}

impl HttpResponse {
    pub fn new(name: &str, value: &str) -> Self {
        let header = format!("{}: {}\r\n", name, value);
        HttpResponse { header }
    }

    pub fn get_header(&self) -> &str {
        &self.header
    }
}

pub fn generate_response(value: &str) -> String {
    let response = HttpResponse::new("Location", value);
    response.get_header().to_string()
}